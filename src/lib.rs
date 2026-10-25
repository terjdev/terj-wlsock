//! Provides client and server Wayland socket transport and core protocol messages.

#[cfg(not(unix))]
compile_error!("terj-wlsock supports only Unix-based systems.");

pub mod client;
pub mod protocol;
pub mod server;
pub mod utils;
