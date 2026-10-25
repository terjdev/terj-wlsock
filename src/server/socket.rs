use crate::protocol;
use crate::protocol::ClientState;
use crate::server::{AcceptError, ClosingError, CreationError, Options, SendCommandError};
use crate::server::{CommandRecv, CommandSend};
use crate::utils::reader::FdReaderState;
use crate::utils::reader::{FdPayloadReader, FdReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::collections::HashMap;
use std::os::unix::net::UnixStream;
use std::sync::{Arc, Mutex};
use tokio::io::unix::AsyncFd;
use tokio::net::unix::SocketAddr;
use tokio::net::{UnixListener as TokioUnixListener, UnixStream as TokioUnixStream};
use tokio::runtime::Handle;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

/// Controls a background Wayland server and its client tasks.
///
/// Dropping this handle requests abrupt cancellation without waiting for resource
/// cleanup or guaranteeing disconnect notifications. Use [`Self::close`] for
/// an orderly shutdown. Closing the listener does not remove its filesystem path.
#[derive(Debug)]
pub struct Socket {
    worker: Option<tokio::task::JoinHandle<()>>,
    cancelation_token: CancellationToken,
}

impl Socket {
    /// Starts serving an existing listener and returns the handle and application channels.
    ///
    /// Returns `(socket, command_sender, notification_receiver)`. The sender may
    /// be cloned and the receiver may be processed independently of the handle.
    /// Dropping all command senders initiates shutdown after queued commands are
    /// processed. Dropping the notification receiver prevents further delivery.
    ///
    /// Takes ownership of `listener`, including on error. Its original I/O runtime
    /// must remain active; using the same runtime for listener creation and this
    /// call is recommended. The current runtime must have I/O enabled for accepted
    /// connections. The caller remains responsible for removing the socket path.
    ///
    /// # Errors
    /// Returns [`CreationError`] for zero queue capacities or a missing Tokio context.
    pub fn create(
        listener: TokioUnixListener,
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

        let (send_tx, send_rx) = mpsc::channel::<CommandSend>(options.send_queue_limit);
        let (recv_tx, recv_rx) = mpsc::channel::<CommandRecv>(options.recv_queue_limit);
        let cancelation_token = CancellationToken::new();
        let cancelation_handle = cancelation_token.clone();
        let worker_runtime = runtime.clone();
        let worker = Some(runtime.spawn(async move {
            handle_server(
                worker_runtime,
                listener,
                options,
                cancelation_handle,
                send_rx,
                recv_tx,
            )
            .await
        }));

        Ok((
            Self {
                worker,
                cancelation_token,
            },
            send_tx,
            recv_rx,
        ))
    }

    /// Requests shutdown and waits for the server and its client tasks to finish.
    ///
    /// Continue draining the returned notification receiver concurrently, until
    /// it returns `None`. Shutdown can wait indefinitely for space in that queue;
    /// pending sends in the server loop can also delay observing cancellation.
    /// Disconnect notifications are queued for remaining registered clients when
    /// the receiver remains open. Successful completion does not mean the
    /// application has processed those notifications.
    ///
    /// Cancelling this future leaves the shutdown request active and retains the
    /// worker handle, allowing another call to resume waiting. Once a call has
    /// returned, subsequent calls return `Ok(())`, including after a reported error.
    ///
    /// # Errors
    /// Returns [`ClosingError::Failed`] if the server task panicked or was aborted.
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

#[derive(Debug)]
struct ClientsRegistryEntry {
    send_tx: mpsc::Sender<protocol::CommandEvent>,
    disconnect_tx: Option<oneshot::Sender<protocol::DisconnectReason>>,
}

#[derive(Debug, Default)]
struct ClientsRegistry {
    join_set: JoinSet<protocol::DisconnectReason>,
    tasks_to_ids: HashMap<tokio::task::Id, protocol::ClientId>,
    clients_rx: HashMap<protocol::ClientId, ClientsRegistryEntry>,
    next_client_id: u64,
}

impl ClientsRegistry {
    fn next_client_id(&mut self) -> Option<u64> {
        match self.next_client_id.checked_add(1) {
            Some(value) => {
                self.next_client_id = value;
                Some(value)
            }
            None => None,
        }
    }
}

async fn handle_server(
    runtime: Handle,
    listener: TokioUnixListener,
    options: Options,
    cancelation_token: CancellationToken,
    mut send_rx: mpsc::Receiver<CommandSend>,
    mut recv_tx: mpsc::Sender<CommandRecv>,
) {
    let mut clients_registry = ClientsRegistry::default();
    loop {
        tokio::select! {
            _ = cancelation_token.cancelled() => {
                break;
            },
            accepted = listener.accept() => {
                if accept_client(&runtime, &options, &mut clients_registry, accepted, &mut recv_tx).await.is_err() {
                    break;
                }
            },
            command = send_rx.recv() => {
                match command {
                    Some(command) => {
                        if send_command(command, &mut clients_registry, &mut recv_tx).await.is_err() {
                            break;
                        }
                    },
                    None => break,
                }
            },
            completed = clients_registry.join_set.join_next_with_id(), if !clients_registry.join_set.is_empty() => {
                match completed {
                    None => continue,
                    Some(completed) => {
                        if let Some(cmd) = handle_client_disconnect(&mut clients_registry, completed).await
                            && recv_tx.send(cmd).await.is_err()
                        {
                            break;
                        }
                    },
                }
            }
        }
    }

    drop(listener);
    drop(send_rx);
    clients_registry.join_set.abort_all();
    while let Some(completed) = clients_registry.join_set.join_next_with_id().await {
        if let Some(cmd) = handle_client_disconnect(&mut clients_registry, completed).await
            && recv_tx.send(cmd).await.is_err()
        {
            // recv_tx already closed, wait for shutdown without explicit cleanup
            clients_registry.join_set.shutdown().await;
            break;
        }
    }
}

async fn handle_client_disconnect(
    clients_registry: &mut ClientsRegistry,
    completed: Result<(tokio::task::Id, protocol::DisconnectReason), tokio::task::JoinError>,
) -> Option<CommandRecv> {
    let (task_id, reason) = match completed {
        Ok((task_id, result)) => (task_id, result),
        Err(error) => {
            let task_id = error.id();
            let protocol_error = if error.is_cancelled() {
                protocol::DisconnectReason::Server
            } else {
                protocol::DisconnectReason::Error(protocol::ClientError::UnhandledTaskError(error))
            };
            (task_id, protocol_error)
        }
    };

    if let Some(client_id) = clients_registry.tasks_to_ids.remove(&task_id) {
        clients_registry.clients_rx.remove(&client_id);

        return Some(CommandRecv::Disconnect { client_id, reason });
    }

    None
}

async fn handle_client(
    id: protocol::ClientId,
    fd_read: AsyncFd<UnixStream>,
    fd_write: AsyncFd<UnixStream>,
    recv_tx: mpsc::Sender<CommandRecv>,
    send_rx: mpsc::Receiver<protocol::CommandEvent>,
    disconnect_rx: oneshot::Receiver<protocol::DisconnectReason>,
) -> protocol::DisconnectReason {
    let state = Arc::new(Mutex::new(ClientState::default()));
    // Prioritize disconnection without giving either I/O direction a fixed priority.
    tokio::select! {
        biased;
        result = disconnect_rx => {
            match result {
                Ok(reason) => reason,
                Err(_) => protocol::DisconnectReason::Server,
            }
        },
        result = async {
            // A busy reader can exhaust the shared Tokio task budget before
            // writes are polled. Randomize the I/O polling order to avoid this.
            // Keep both futures alive so partial I/O is never restarted.
            tokio::select! {
                result = read_requests(id, fd_read, state.clone(), recv_tx) => result,
                result = write_events(fd_write, state, send_rx) => result,
            }
        } => result,
    }
}

async fn send_command(
    command: CommandSend,
    registry: &mut ClientsRegistry,
    recv_tx: &mut mpsc::Sender<CommandRecv>,
) -> Result<(), SendCommandError> {
    let client_id = match command {
        CommandSend::Event { client_id, .. } => client_id,
        CommandSend::Disconnect { client_id } => client_id,
    };

    if let Some(client_rx) = registry.clients_rx.get_mut(&client_id) {
        match command {
            CommandSend::Event { command, .. } => {
                if let Err(error) = client_rx.send_tx.try_send(command) {
                    let disconnect_error = match error {
                        mpsc::error::TrySendError::Full(_) => {
                            protocol::ClientError::SendQueueOverflow
                        }
                        mpsc::error::TrySendError::Closed(_) => {
                            protocol::ClientError::SendQueueClosed
                        }
                    };

                    let disconnect_reason = protocol::DisconnectReason::Error(disconnect_error);
                    if let Some(disconnect_tx) = client_rx.disconnect_tx.take() {
                        // Ignore when receiver already closed (already disconnected from inside)
                        _ = disconnect_tx.send(disconnect_reason);
                    }
                }
            }
            CommandSend::Disconnect { .. } => {
                if let Some(disconnect_tx) = client_rx.disconnect_tx.take() {
                    // Ignore when receiver already closed (already disconnected from inside)
                    _ = disconnect_tx.send(protocol::DisconnectReason::Server);
                }
            }
        }
    } else {
        let cmd = CommandRecv::Error {
            protocol_error: protocol::ServerError::ClientNotFound(client_id),
        };
        recv_tx.send(cmd).await?;
    }

    Ok(())
}

async fn write_events(
    fd: AsyncFd<UnixStream>,
    state: Arc<Mutex<ClientState>>,
    mut send_rx: mpsc::Receiver<protocol::CommandEvent>,
) -> protocol::DisconnectReason {
    let mut writer = FdWriter::new(fd);
    while let Some(command) = send_rx.recv().await {
        if let Err(error) = write_event(command, &mut writer, &state).await {
            return protocol::DisconnectReason::Error(protocol::ClientError::Write(error));
        }

        if send_rx.is_empty()
            && let Err(error) = writer.flush().await
        {
            return protocol::DisconnectReason::Error(protocol::ClientError::Write(error));
        }
    }

    protocol::DisconnectReason::Server
}

async fn write_event(
    command: protocol::CommandEvent,
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

async fn read_requests(
    id: protocol::ClientId,
    fd: AsyncFd<UnixStream>,
    state: Arc<Mutex<ClientState>>,
    recv_tx: mpsc::Sender<CommandRecv>,
) -> protocol::DisconnectReason {
    let mut reader = FdReader::new(fd);
    while *reader.state() == FdReaderState::Open {
        let object_id = match reader.next_id().await {
            Ok(object_id) => object_id,
            Err(FdReaderError::EndOfStream) => {
                return protocol::DisconnectReason::Client;
            }
            Err(error) => {
                return protocol::DisconnectReason::Error(protocol::ClientError::Read(error));
            }
        };

        let read_result = match read_request(object_id, &mut reader, &state).await {
            Ok(result) => result,
            Err(error) => {
                return protocol::DisconnectReason::Error(protocol::ClientError::Read(error));
            }
        };

        let cmd = CommandRecv::Request {
            client_id: id,
            command: read_result,
        };

        if recv_tx.send(cmd).await.is_err() {
            return protocol::DisconnectReason::Server;
        }
    }

    match reader.state() {
        FdReaderState::EndOfStream => protocol::DisconnectReason::Client,
        _ => protocol::DisconnectReason::Error(protocol::ClientError::ConnectionFailed),
    }
}

async fn read_request(
    object_id: u32,
    reader: &mut FdReader,
    state: &Arc<Mutex<ClientState>>,
) -> Result<protocol::CommandRequest, FdReaderError> {
    use crate::protocol::CommandReader as _;
    let word = reader.next_u32().await?;
    let size = (word >> 16) as usize;
    let opcode = (word & 0xffff) as u16;
    if size < 8 || !size.is_multiple_of(4) {
        return Err(FdReaderError::InvalidProtocol);
    }

    let payload_size = size - 8;
    let mut payload_reader = FdPayloadReader::new(reader, payload_size)?;
    let result =
        protocol::CommandRequest::read(object_id, opcode, &mut payload_reader, state).await?;
    Ok(result)
}

async fn accept_client(
    runtime: &Handle,
    options: &Options,
    clients_registry: &mut ClientsRegistry,
    accepted: std::io::Result<(TokioUnixStream, SocketAddr)>,
    recv_tx: &mut mpsc::Sender<CommandRecv>,
) -> Result<(), AcceptError> {
    let (stream, _) = match accepted {
        Ok(value) => value,
        Err(error) => {
            let cmd = CommandRecv::Error {
                protocol_error: protocol::ServerError::ClientAccept(error),
            };
            recv_tx.send(cmd).await?;
            return Ok(());
        }
    };

    let client_id = match clients_registry.next_client_id() {
        Some(id) => protocol::ClientId(id),
        None => {
            let cmd = CommandRecv::Error {
                protocol_error: protocol::ServerError::ClientIndexOverflow,
            };
            recv_tx.send(cmd).await?;
            return Err(AcceptError::IndexOverflow);
        }
    };

    let (fd_read, fd_write) = match crate::utils::stream_to_async_fd(stream) {
        Ok(value) => {
            let cmd = CommandRecv::Connect { client_id };
            recv_tx.send(cmd).await?;
            value
        }
        Err(error) => {
            let cmd = CommandRecv::Error {
                protocol_error: protocol::ServerError::ClientAccept(error),
            };
            recv_tx.send(cmd).await?;
            return Ok(());
        }
    };

    let (client_send_tx, client_send_rx) =
        mpsc::channel::<protocol::CommandEvent>(options.send_queue_limit);

    let (client_disconnect_tx, client_disconnect_rx) =
        oneshot::channel::<protocol::DisconnectReason>();

    let clients_registry_entry = ClientsRegistryEntry {
        send_tx: client_send_tx,
        disconnect_tx: Some(client_disconnect_tx),
    };

    clients_registry
        .clients_rx
        .insert(client_id, clients_registry_entry);
    let task = clients_registry.join_set.spawn_on(
        handle_client(
            client_id,
            fd_read,
            fd_write,
            recv_tx.clone(),
            client_send_rx,
            client_disconnect_rx,
        ),
        runtime,
    );
    clients_registry.tasks_to_ids.insert(task.id(), client_id);
    Ok(())
}
