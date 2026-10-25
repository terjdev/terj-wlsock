//! Encodes and decodes the pointer-warp extension.
//!
//! Clients request a pointer position relative to a surface using fixed coordinates
//! and an enter serial. The caller checks focus, serials and pointer lifetimes
//! and decides whether to honor the request. This does not constrain the pointer.
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
//! Defined by upstream [pointer-warp-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/pointer-warp/pointer-warp-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod wp_pointer_warp_v1_destroy_request;
#[doc(inline)]
pub use wp_pointer_warp_v1_destroy_request::WpPointerWarpV1DestroyRequest;

mod wp_pointer_warp_v1_warp_pointer_request;
#[doc(inline)]
pub use wp_pointer_warp_v1_warp_pointer_request::WpPointerWarpV1WarpPointerRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
