//! Encodes and decodes the fullscreen-shell protocol family.
//!
//! Fullscreen surfaces can target outputs and request display modes. Mode feedback
//! destroys itself on success, failure or cancellation. The caller implements mode
//! selection, capability advertising, presentation methods and output lifetimes.
//!
//! Codecs validate receiver protocol, opcode, minimum version and complete framing.
//! New child IDs inherit the receiver version capped at the target maximum; core
//! keyboard objects use version 1. Registration precedes writes; failed or cancelled
//! writes after registration require closing the connection. Client IDs remain
//! reserved until `wl_display.delete_id`; destructors release server IDs.
//! Fixed values use `f64` with signed 24.8 encoding. Strings and fixed values are
//! validated before writes. Nullable values use `Option`; arrays retain raw bytes.
//! Unknown enum values and flag bits are preserved. Protocol semantics belong to
//! the caller. Original copyright and permission terms are in LICENSE.md.
//! Source: [fullscreen-shell-unstable-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/unstable/fullscreen-shell/fullscreen-shell-unstable-v1.xml).

mod zwp_fullscreen_shell_v1_capability;
#[doc(inline)]
pub use zwp_fullscreen_shell_v1_capability::ZwpFullscreenShellV1Capability;

mod zwp_fullscreen_shell_v1_present_method;
#[doc(inline)]
pub use zwp_fullscreen_shell_v1_present_method::ZwpFullscreenShellV1PresentMethod;

mod zwp_fullscreen_shell_v1_error;
#[doc(inline)]
pub use zwp_fullscreen_shell_v1_error::ZwpFullscreenShellV1Error;

mod zwp_fullscreen_shell_v1_release_request;
#[doc(inline)]
pub use zwp_fullscreen_shell_v1_release_request::ZwpFullscreenShellV1ReleaseRequest;

mod zwp_fullscreen_shell_v1_present_surface_request;
#[doc(inline)]
pub use zwp_fullscreen_shell_v1_present_surface_request::ZwpFullscreenShellV1PresentSurfaceRequest;

mod zwp_fullscreen_shell_v1_present_surface_for_mode_request;
#[doc(inline)]
pub use zwp_fullscreen_shell_v1_present_surface_for_mode_request::ZwpFullscreenShellV1PresentSurfaceForModeRequest;

mod zwp_fullscreen_shell_v1_capability_event;
#[doc(inline)]
pub use zwp_fullscreen_shell_v1_capability_event::ZwpFullscreenShellV1CapabilityEvent;

mod zwp_fullscreen_shell_mode_feedback_v1_mode_successful_event;
#[doc(inline)]
pub use zwp_fullscreen_shell_mode_feedback_v1_mode_successful_event::ZwpFullscreenShellModeFeedbackV1ModeSuccessfulEvent;

mod zwp_fullscreen_shell_mode_feedback_v1_mode_failed_event;
#[doc(inline)]
pub use zwp_fullscreen_shell_mode_feedback_v1_mode_failed_event::ZwpFullscreenShellModeFeedbackV1ModeFailedEvent;

mod zwp_fullscreen_shell_mode_feedback_v1_present_cancelled_event;
#[doc(inline)]
pub use zwp_fullscreen_shell_mode_feedback_v1_present_cancelled_event::ZwpFullscreenShellModeFeedbackV1PresentCancelledEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
