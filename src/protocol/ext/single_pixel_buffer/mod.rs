//! Encodes and decodes the single-pixel-buffer extension.
//!
//! The manager creates core wl_buffer objects at version 1 from four u32 channels.
//! The caller interprets channel values according to the specification and
//! implements buffer content, rendering and release notifications.
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
//! Defined by upstream [single-pixel-buffer-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/single-pixel-buffer/single-pixel-buffer-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod wp_single_pixel_buffer_manager_v1_destroy_request;
#[doc(inline)]
pub use wp_single_pixel_buffer_manager_v1_destroy_request::WpSinglePixelBufferManagerV1DestroyRequest;

mod wp_single_pixel_buffer_manager_v1_create_u32_rgba_buffer_request;
#[doc(inline)]
pub use wp_single_pixel_buffer_manager_v1_create_u32_rgba_buffer_request::WpSinglePixelBufferManagerV1CreateU32RgbaBufferRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
