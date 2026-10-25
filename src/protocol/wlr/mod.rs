//! Provides codecs for protocols from wlr-protocols.

#[cfg(feature = "wlr-layer-shell")]
pub mod layer_shell;

#[cfg(feature = "wlr-data-control")]
pub mod data_control;

#[cfg(feature = "wlr-virtual-pointer")]
pub mod virtual_pointer;

#[cfg(feature = "wlr-screencopy")]
pub mod screencopy;
