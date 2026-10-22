//! Serves Wayland clients through independently owned command and notification channels.
//!
//! [`Socket::create`] takes an existing Tokio Unix listener and returns a server
//! handle, a command sender, and a notification receiver. Keep the handle alive
//! and continue draining notifications while awaiting [`Socket::close`].
//! The caller owns the lifecycle of the socket's filesystem path.

mod options;
pub use options::*;

mod errors;
pub use errors::*;

mod socket;
pub use socket::*;

mod command_send;
pub use command_send::CommandSend;

mod command_recv;
pub use command_recv::CommandRecv;
