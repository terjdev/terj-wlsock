use crate::{client::ClosingError, utils::writer::FdPayloadWriter};
use std::{os::unix::net::UnixStream, sync::Arc, sync::Mutex};

use tokio::{io::unix::AsyncFd, net::UnixStream as TokioUnixStream, runtime::Handle, sync::mpsc};
use tokio_util::sync::CancellationToken;

use crate::{
    client::{CommandRecv, CommandSend, CreationError, Options},
    protocol::{ClientError, ClientState, CommandEvent, CommandRequest, DisconnectReason},
    utils::{
        reader::{FdPayloadReader, FdReader, FdReaderError},
        writer::{FdWriter, FdWriterError},
    },
};

pub struct Socket {
    worker: Option<tokio::task::JoinHandle<()>>,
    cancelation_token: CancellationToken,
}

impl Socket {
    pub fn create(
        stream: TokioUnixStream,
        options: Options,
    ) -> Result<(Self, mpsc::Sender<CommandSend>, mpsc::Receiver<CommandRecv>), CreationError> {
        if options.send_queue_limit == 0 {
            return Err(CreationError::InvalidSendQueueLimit);
        }

        if options.recv_queue_limit == 0 {
            return Err(CreationError::InvalidRecvQueueLimit);
        }

        let runtime = match Handle::try_current() {
            Ok(runtime) => runtime,
            Err(_) => return Err(CreationError::InvalidAsyncContext),
        };

        let (fd_read, fd_write) = crate::utils::stream_to_async_fd(stream)?;
        let (send_tx, send_rx) = mpsc::channel(options.send_queue_limit);
        let (recv_tx, recv_rx) = mpsc::channel(options.recv_queue_limit);

        let cancelation_token = CancellationToken::new();
        let cancelation_handle = cancelation_token.clone();

        let worker = runtime.spawn(async move {
            handle_client(fd_read, fd_write, send_rx, recv_tx, cancelation_handle).await;
        });

        Ok((
            Self {
                worker: Some(worker),
                cancelation_token,
            },
            send_tx,
            recv_rx,
        ))
    }

    pub async fn close(&mut self) -> Result<(), ClosingError> {
        self.cancelation_token.cancel();

        let result = match self.worker.as_mut() {
            Some(worker) => worker.await,
            None => return Ok(()),
        };

        self.worker = None;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(ClosingError::Failed),
        }
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        if let Some(worker) = self.worker.take() {
            worker.abort();
        }
    }
}

async fn handle_client(
    fd_read: AsyncFd<UnixStream>,
    fd_write: AsyncFd<UnixStream>,
    send_rx: mpsc::Receiver<CommandSend>,
    recv_tx: mpsc::Sender<CommandRecv>,
    cancelation_token: CancellationToken,
) {
    let state = Arc::new(Mutex::new(ClientState::default()));

    let reason = {
        // Prioritize cancellation without giving either I/O direction a fixed priority.
        tokio::select! {
            biased;
            _ = cancelation_token.cancelled() => {
                DisconnectReason::Client
            },
            reason = async {
                // A busy reader can exhaust the shared Tokio task budget before
                // writes are polled. Randomize the I/O polling order to avoid this.
                // Keep both futures alive so partial I/O is never restarted.
                tokio::select! {
                    reason = read_events(fd_read, state.clone(), recv_tx.clone()) => reason,
                    reason = write_requests(fd_write, state, send_rx) => reason,
                }
            } => reason,
        }
    };

    // Ignore send failure: the application has closed the notification receiver.
    let _ = recv_tx.send(CommandRecv::Disconnect { reason }).await;
}

async fn read_events(
    fd: AsyncFd<UnixStream>,
    state: Arc<Mutex<ClientState>>,
    recv_tx: mpsc::Sender<CommandRecv>,
) -> DisconnectReason {
    let mut reader = FdReader::new(fd);

    loop {
        let object_id = match reader.next_u32().await {
            Ok(id) => {
                id // TODO: fix issue when partially received object ID also returns EOF instead of protocol error.
            }
            Err(FdReaderError::EndOfStream) => {
                return DisconnectReason::Server;
            }
            Err(error) => {
                return DisconnectReason::Error(ClientError::Read(error));
            }
        };

        let command = match read_event(object_id, &mut reader, &state).await {
            Ok(command) => command,
            Err(error) => {
                return DisconnectReason::Error(ClientError::Read(error));
            }
        };

        if recv_tx.send(CommandRecv::Event { command }).await.is_err() {
            return DisconnectReason::Client;
        }
    }
}

async fn read_event(
    object_id: u32,
    reader: &mut FdReader,
    state: &Arc<Mutex<ClientState>>,
) -> Result<CommandEvent, FdReaderError> {
    use crate::protocol::CommandReader as _;
    let word = reader.next_u32().await?;
    let size = (word >> 16) as usize;
    let opcode = (word & 0xffff) as u16;

    if size < 8 || size % 4 != 0 {
        return Err(FdReaderError::InvalidProtocol);
    }

    let mut payload = FdPayloadReader::new(reader, size - 8)?;
    CommandEvent::read(object_id, opcode, &mut payload, state).await
}

async fn write_requests(
    fd: AsyncFd<UnixStream>,
    state: Arc<Mutex<ClientState>>,
    mut send_rx: mpsc::Receiver<CommandSend>,
) -> DisconnectReason {
    let mut writer = FdWriter::new(fd);

    while let Some(command) = send_rx.recv().await {
        let CommandSend::Request { command } = command;
        if let Err(error) = write_request(command, &mut writer, &state).await {
            return DisconnectReason::Error(ClientError::Write(error));
        }

        if send_rx.is_empty() {
            if let Err(error) = writer.flush().await {
                return DisconnectReason::Error(ClientError::Write(error));
            }
        }
    }

    DisconnectReason::Client
}

async fn write_request(
    command: CommandRequest,
    writer: &mut FdWriter,
    state: &Arc<Mutex<ClientState>>,
) -> Result<(), FdWriterError> {
    use crate::protocol::CommandWriter as _;
    let payload_size = command.payload_size();
    let size = payload_size
        .checked_add(8)
        .filter(|size| *size <= usize::from(u16::MAX) && *size % 4 == 0)
        .ok_or(FdWriterError::InvalidProtocol)?;

    let object_id = command.object_id();
    let opcode = command.opcode();
    let header_word = ((size as u32) << 16) | u32::from(opcode);

    writer.next_u32(object_id).await?;
    writer.next_u32(header_word).await?;

    let mut payload = FdPayloadWriter::new(writer, payload_size)?;
    command.write(&mut payload, state).await?;
    Ok(())
}
