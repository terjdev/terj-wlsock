//! Exchanges Wayland requests and events over an existing client connection.
//!
//! [`Socket::create`] returns a connection handle and independently owned command
//! channels. Keep the handle alive and drain notifications while awaiting
//! [`Socket::close`]. The application handles compositor policy and fatal protocol
//! error events, including deciding when to close the connection.

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
