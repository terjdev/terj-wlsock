//! Provides codecs for protocols from wlr-protocols.

#[cfg(feature = "wlr-layer-shell")]
pub mod layer_shell;

#[cfg(feature = "wlr-data-control")]
pub mod data_control;
