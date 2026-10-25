//! Encodes and decodes the cursor-shape extension.
//!
//! The manager creates shape devices for core pointers or tablet-v2 tools. Shapes
//! replace cursor images without client-owned cursor surfaces. Device objects become
//! inert when their pointer capability or tablet tool is removed. Version 2 adds
//! dnd_ask and all_resize shapes. The caller validates shape values against the
//! negotiated version, focus, enter/proximity serials and device lifetimes. Tablet
//! tool arguments are wire IDs; the tablet codec need not be enabled.
//!
//! Codecs validate framing, receiver metadata, message versions and new object IDs.
//! Child versions inherit the receiver version, capped at the target maximum.
//! New IDs are registered before writes; close the connection after failed or
//! cancelled writes following registration. Client IDs remain reserved until
//! wl_display.delete_id; destructors release server IDs. Strings are checked for
//! wire encoding, and unknown enum values are preserved. Protocol semantics and
//! compositor policy belong to the caller.
//! This extension defines no events.
//!
//! Defined by upstream [cursor-shape-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/cursor-shape/cursor-shape-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod wp_cursor_shape_manager_v1_destroy_request;
#[doc(inline)]
pub use wp_cursor_shape_manager_v1_destroy_request::WpCursorShapeManagerV1DestroyRequest;

mod wp_cursor_shape_manager_v1_get_pointer_request;
#[doc(inline)]
pub use wp_cursor_shape_manager_v1_get_pointer_request::WpCursorShapeManagerV1GetPointerRequest;

mod wp_cursor_shape_manager_v1_get_tablet_tool_v2_request;
#[doc(inline)]
pub use wp_cursor_shape_manager_v1_get_tablet_tool_v2_request::WpCursorShapeManagerV1GetTabletToolV2Request;

mod wp_cursor_shape_device_v1_shape;
#[doc(inline)]
pub use wp_cursor_shape_device_v1_shape::WpCursorShapeDeviceV1Shape;

mod wp_cursor_shape_device_v1_error;
#[doc(inline)]
pub use wp_cursor_shape_device_v1_error::WpCursorShapeDeviceV1Error;

mod wp_cursor_shape_device_v1_destroy_request;
#[doc(inline)]
pub use wp_cursor_shape_device_v1_destroy_request::WpCursorShapeDeviceV1DestroyRequest;

mod wp_cursor_shape_device_v1_set_shape_request;
#[doc(inline)]
pub use wp_cursor_shape_device_v1_set_shape_request::WpCursorShapeDeviceV1SetShapeRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
