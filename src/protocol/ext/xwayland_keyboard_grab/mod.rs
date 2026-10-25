//! Encodes and decodes the xwayland-keyboard-grab protocol family.
//!
//! The caller restricts access to Xwayland and coordinates X11 active keyboard grabs
//! with Wayland focus, shortcuts and grab policy. Destroying a grab releases it.
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
//! This protocol family defines no events.
//! Source: [xwayland-keyboard-grab-unstable-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/unstable/xwayland-keyboard-grab/xwayland-keyboard-grab-unstable-v1.xml).

mod zwp_xwayland_keyboard_grab_manager_v1_destroy_request;
#[doc(inline)]
pub use zwp_xwayland_keyboard_grab_manager_v1_destroy_request::ZwpXwaylandKeyboardGrabManagerV1DestroyRequest;

mod zwp_xwayland_keyboard_grab_manager_v1_grab_keyboard_request;
#[doc(inline)]
pub use zwp_xwayland_keyboard_grab_manager_v1_grab_keyboard_request::ZwpXwaylandKeyboardGrabManagerV1GrabKeyboardRequest;

mod zwp_xwayland_keyboard_grab_v1_destroy_request;
#[doc(inline)]
pub use zwp_xwayland_keyboard_grab_v1_destroy_request::ZwpXwaylandKeyboardGrabV1DestroyRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
