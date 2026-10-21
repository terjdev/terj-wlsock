use std::{os::unix::net::UnixStream, sync::Arc, sync::Mutex};

use tokio::{io::unix::AsyncFd, net::UnixStream as TokioUnixStream, runtime::Handle, sync::mpsc};
use tokio_util::sync::CancellationToken;

use crate::{client::{CommandRecv, CommandSend, CreationError, Options}, protocol::{ClientState, DisconnectReason}};

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
            handle_client(fd_read, fd_write, send_rx, recv_tx, options, cancelation_handle).await;
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
}

async fn handle_client(
    fd_read: AsyncFd<UnixStream>,
    fd_write: AsyncFd<UnixStream>,
    send_rx: mpsc::Receiver<CommandSend>,
    recv_tx: mpsc::Sender<CommandRecv>,
    options: Options,
    cancelation_token: CancellationToken,
) {
    let state = Arc::new(Mutex::new(ClientState::default()));

    let reason = {
        tokio::select! {
            biased;
            _ = cancelation_token.cancelled() => {
                DisconnectReason::Client
            },
            reason = read_events(fd_read, state.clone(), recv_tx.clone()) => {
                reason
            },
            reason = write_requests(fd_write, state, send_rx) => {
                reason
            },
        }
    };

    // TODO: review
    let _ = recv_tx.send(CommandRecv::Disconnect { reason }).await;
}
