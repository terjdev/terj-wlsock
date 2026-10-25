//! Encodes and decodes the xdg-system-bell extension.
//!
//! Clients request a system bell, optionally associated with a surface. The caller
//! selects audible or visual feedback and applies accessibility and rate policy.
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
//! Defined by upstream [xdg-system-bell-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/xdg-system-bell/xdg-system-bell-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod xdg_system_bell_v1_destroy_request;
#[doc(inline)]
pub use xdg_system_bell_v1_destroy_request::XdgSystemBellV1DestroyRequest;

mod xdg_system_bell_v1_ring_request;
#[doc(inline)]
pub use xdg_system_bell_v1_ring_request::XdgSystemBellV1RingRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
