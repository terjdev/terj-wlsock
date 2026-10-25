//! Provides codecs for protocols from wlr-protocols.

#[cfg(feature = "wlr-layer-shell")]
pub mod layer_shell;

#[cfg(feature = "wlr-data-control")]
pub mod data_control;

#[cfg(feature = "wlr-virtual-pointer")]
pub mod virtual_pointer;

#[cfg(feature = "wlr-screencopy")]
pub mod screencopy;

#[cfg(feature = "wlr-export-dmabuf")]
pub mod export_dmabuf;

#[cfg(feature = "wlr-foreign-toplevel-management")]
pub mod foreign_toplevel_management;

#[cfg(feature = "wlr-gamma-control")]
pub mod gamma_control;

#[cfg(feature = "wlr-input-inhibitor")]
pub mod input_inhibitor;

#[cfg(feature = "wlr-output-management")]
pub mod output_management;

#[cfg(feature = "wlr-output-power-management")]
pub mod output_power_management;
