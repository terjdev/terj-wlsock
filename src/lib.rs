//! Provides server-side Wayland socket transport and core protocol messages.
//!
//! The package license and third-party notices are included below.
#![doc = include_str!("../LICENSE.md")]

#[cfg(target_os = "macos")]
compile_error!("terj-wlsock does not support macOS.");

#[cfg(target_os = "android")]
compile_error!("terj-wlsock does not support Android.");

#[cfg(not(unix))]
compile_error!("terj-wlsock is supported only on Unix.");

pub mod client;
pub mod protocol;
pub mod server;
pub mod utils;
