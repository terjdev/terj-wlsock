//! Optional Wayland protocol extensions.

#[cfg(feature = "ext-presentation-time")]
pub mod presentation_time;

#[cfg(feature = "ext-viewporter")]
pub mod viewporter;

#[cfg(feature = "ext-dmabuf")]
pub mod dmabuf;

#[cfg(feature = "ext-xdg-shell")]
pub mod xdg_shell;

#[cfg(feature = "ext-tablet")]
pub mod tablet;

#[cfg(feature = "ext-alpha-modifier")]
pub mod alpha_modifier;

#[cfg(feature = "ext-commit-timing")]
pub mod commit_timing;

#[cfg(feature = "ext-content-type")]
pub mod content_type;

#[cfg(feature = "ext-cursor-shape")]
pub mod cursor_shape;

#[cfg(feature = "ext-drm-lease")]
pub mod drm_lease;

#[cfg(feature = "ext-background-effect")]
pub mod background_effect;

#[cfg(feature = "ext-data-control")]
pub mod data_control;
