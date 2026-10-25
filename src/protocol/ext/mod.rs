//! Optional Wayland protocol extensions.

#[cfg(feature = "ext-presentation-time")]
pub mod presentation_time;

#[cfg(feature = "ext-viewporter")]
pub mod viewporter;
