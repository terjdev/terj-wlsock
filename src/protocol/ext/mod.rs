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

#[cfg(feature = "ext-color-management")]
pub mod color_management;

#[cfg(feature = "ext-color-representation")]
pub mod color_representation;

#[cfg(feature = "ext-foreign-toplevel-list")]
pub mod foreign_toplevel_list;

#[cfg(feature = "ext-idle-notify")]
pub mod idle_notify;

#[cfg(feature = "ext-image-capture-source")]
pub mod image_capture_source;

#[cfg(feature = "ext-image-copy-capture")]
pub mod image_copy_capture;

#[cfg(feature = "ext-session-lock")]
pub mod session_lock;

#[cfg(feature = "ext-transient-seat")]
pub mod transient_seat;

#[cfg(feature = "ext-workspace")]
pub mod workspace;

#[cfg(feature = "ext-fifo")]
pub mod fifo;

#[cfg(feature = "ext-fractional-scale")]
pub mod fractional_scale;

#[cfg(feature = "ext-linux-drm-syncobj")]
pub mod linux_drm_syncobj;

#[cfg(feature = "ext-pointer-warp")]
pub mod pointer_warp;

#[cfg(feature = "ext-security-context")]
pub mod security_context;

#[cfg(feature = "ext-single-pixel-buffer")]
pub mod single_pixel_buffer;

#[cfg(feature = "ext-tearing-control")]
pub mod tearing_control;

#[cfg(feature = "ext-xdg-activation")]
pub mod xdg_activation;

#[cfg(feature = "ext-xdg-dialog")]
pub mod xdg_dialog;

#[cfg(feature = "ext-xdg-session-management")]
pub mod xdg_session_management;

#[cfg(feature = "ext-xdg-system-bell")]
pub mod xdg_system_bell;

#[cfg(feature = "ext-xdg-toplevel-drag")]
pub mod xdg_toplevel_drag;

#[cfg(feature = "ext-xdg-toplevel-icon")]
pub mod xdg_toplevel_icon;

#[cfg(feature = "ext-xdg-toplevel-tag")]
pub mod xdg_toplevel_tag;

#[cfg(feature = "ext-xwayland-shell")]
pub mod xwayland_shell;

#[cfg(feature = "ext-fullscreen-shell")]
pub mod fullscreen_shell;

#[cfg(feature = "ext-idle-inhibit")]
pub mod idle_inhibit;

#[cfg(feature = "ext-input-method")]
pub mod input_method;

#[cfg(feature = "ext-input-timestamps")]
pub mod input_timestamps;

#[cfg(feature = "ext-keyboard-shortcuts-inhibit")]
pub mod keyboard_shortcuts_inhibit;

#[cfg(feature = "ext-linux-explicit-synchronization")]
pub mod linux_explicit_synchronization;

#[cfg(feature = "ext-pointer-constraints")]
pub mod pointer_constraints;

#[cfg(feature = "ext-pointer-gestures")]
pub mod pointer_gestures;

#[cfg(feature = "ext-primary-selection")]
pub mod primary_selection;

#[cfg(feature = "ext-relative-pointer")]
pub mod relative_pointer;

#[cfg(feature = "ext-text-input")]
pub mod text_input;

#[cfg(feature = "ext-xdg-decoration")]
pub mod xdg_decoration;

#[cfg(feature = "ext-xdg-foreign")]
pub mod xdg_foreign;

#[cfg(feature = "ext-xdg-output")]
pub mod xdg_output;

#[cfg(feature = "ext-xwayland-keyboard-grab")]
pub mod xwayland_keyboard_grab;
