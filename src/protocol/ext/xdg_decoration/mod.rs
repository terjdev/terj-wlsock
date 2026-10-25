//! Encodes and decodes the xdg-decoration protocol family.
//!
//! Toplevel decoration objects negotiate client-side or server-side decoration.
//! The caller coordinates modes with xdg-shell configure/acknowledgement state,
//! surface lifetimes, decoration creation and compositor policy.
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
//! Source: [xdg-decoration-unstable-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/unstable/xdg-decoration/xdg-decoration-unstable-v1.xml).

mod zxdg_decoration_manager_v1_destroy_request;
#[doc(inline)]
pub use zxdg_decoration_manager_v1_destroy_request::ZxdgDecorationManagerV1DestroyRequest;

mod zxdg_decoration_manager_v1_get_toplevel_decoration_request;
#[doc(inline)]
pub use zxdg_decoration_manager_v1_get_toplevel_decoration_request::ZxdgDecorationManagerV1GetToplevelDecorationRequest;

mod zxdg_toplevel_decoration_v1_error;
#[doc(inline)]
pub use zxdg_toplevel_decoration_v1_error::ZxdgToplevelDecorationV1Error;

mod zxdg_toplevel_decoration_v1_mode;
#[doc(inline)]
pub use zxdg_toplevel_decoration_v1_mode::ZxdgToplevelDecorationV1Mode;

mod zxdg_toplevel_decoration_v1_destroy_request;
#[doc(inline)]
pub use zxdg_toplevel_decoration_v1_destroy_request::ZxdgToplevelDecorationV1DestroyRequest;

mod zxdg_toplevel_decoration_v1_set_mode_request;
#[doc(inline)]
pub use zxdg_toplevel_decoration_v1_set_mode_request::ZxdgToplevelDecorationV1SetModeRequest;

mod zxdg_toplevel_decoration_v1_unset_mode_request;
#[doc(inline)]
pub use zxdg_toplevel_decoration_v1_unset_mode_request::ZxdgToplevelDecorationV1UnsetModeRequest;

mod zxdg_toplevel_decoration_v1_configure_event;
#[doc(inline)]
pub use zxdg_toplevel_decoration_v1_configure_event::ZxdgToplevelDecorationV1ConfigureEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
