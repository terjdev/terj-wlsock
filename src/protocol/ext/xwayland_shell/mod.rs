//! Encodes and decodes the xwayland-shell extension.
//!
//! Xwayland surfaces associate wl_surface objects with split 64-bit X11 serials.
//! The caller restricts the global to Xwayland and applies serial associations
//! on surface commit while enforcing role and lifetime constraints.
//!
//! Codecs validate framing, receiver metadata, message versions and new object IDs.
//! Child versions inherit the parent version, capped at the target maximum; core
//! buffers use version 1. New IDs are registered before writes. A failed or
//! cancelled write after registration requires closing the connection. Client IDs
//! remain reserved until `wl_display.delete_id`; destructors release server IDs.
//! Strings and fixed values are checked for wire encoding. Nullable arguments use
//! `Option`; arrays retain raw bytes. Unknown enum values and flag bits are preserved.
//! The caller enforces protocol semantics, object relationships and compositor policy.
//! This extension defines no events.
//!
//! Defined by upstream [xwayland-shell-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/xwayland-shell/xwayland-shell-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod xwayland_shell_v1_error;
#[doc(inline)]
pub use xwayland_shell_v1_error::XwaylandShellV1Error;

mod xwayland_shell_v1_destroy_request;
#[doc(inline)]
pub use xwayland_shell_v1_destroy_request::XwaylandShellV1DestroyRequest;

mod xwayland_shell_v1_get_xwayland_surface_request;
#[doc(inline)]
pub use xwayland_shell_v1_get_xwayland_surface_request::XwaylandShellV1GetXwaylandSurfaceRequest;

mod xwayland_surface_v1_error;
#[doc(inline)]
pub use xwayland_surface_v1_error::XwaylandSurfaceV1Error;

mod xwayland_surface_v1_set_serial_request;
#[doc(inline)]
pub use xwayland_surface_v1_set_serial_request::XwaylandSurfaceV1SetSerialRequest;

mod xwayland_surface_v1_destroy_request;
#[doc(inline)]
pub use xwayland_surface_v1_destroy_request::XwaylandSurfaceV1DestroyRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
