//! Encodes and decodes the ext-image-capture-source extension.
//!
//! Factories create opaque capture-source descriptors from outputs and foreign
//! toplevel handles. Source objects are frozen at version 1. The caller authorizes
//! capture, resolves external handles and tracks the lifetime of source content.
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
//! Defined by upstream [ext-image-capture-source-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/ext-image-capture-source/ext-image-capture-source-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod ext_image_capture_source_v1_destroy_request;
#[doc(inline)]
pub use ext_image_capture_source_v1_destroy_request::ExtImageCaptureSourceV1DestroyRequest;

mod ext_output_image_capture_source_manager_v1_create_source_request;
#[doc(inline)]
pub use ext_output_image_capture_source_manager_v1_create_source_request::ExtOutputImageCaptureSourceManagerV1CreateSourceRequest;

mod ext_output_image_capture_source_manager_v1_destroy_request;
#[doc(inline)]
pub use ext_output_image_capture_source_manager_v1_destroy_request::ExtOutputImageCaptureSourceManagerV1DestroyRequest;

mod ext_foreign_toplevel_image_capture_source_manager_v1_create_source_request;
#[doc(inline)]
pub use ext_foreign_toplevel_image_capture_source_manager_v1_create_source_request::ExtForeignToplevelImageCaptureSourceManagerV1CreateSourceRequest;

mod ext_foreign_toplevel_image_capture_source_manager_v1_destroy_request;
#[doc(inline)]
pub use ext_foreign_toplevel_image_capture_source_manager_v1_destroy_request::ExtForeignToplevelImageCaptureSourceManagerV1DestroyRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
