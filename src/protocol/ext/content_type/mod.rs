//! Encodes and decodes the content-type extension through interface version 1.
//!
//! A manager creates one content-type object per wl_surface. Content types are
//! hints for compositor policy: none, photo, video and game. Changes are
//! double-buffered until wl_surface.commit; destruction resets the hint to none
//! with the same semantics. Objects become inert when their surface is destroyed.
//! The caller enforces unique surface associations, object lifetimes and pending
//! state application. Unknown content-type and error values are preserved.
//!
//! Codecs validate framing, receiver metadata, message versions and new object IDs.
//! Child versions inherit the receiver version, capped at the target maximum.
//! New IDs are registered before writes; close the connection after failed or
//! cancelled writes following registration. Client IDs remain reserved until
//! wl_display.delete_id. Protocol semantics and compositor policy belong to the
//! caller. This extension defines no events.
//!
//! Defined by upstream [content-type-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/content-type/content-type-v1.xml).
//! Original copyright and MIT permission terms are preserved in LICENSE.md.

mod wp_content_type_manager_v1_error;
#[doc(inline)]
pub use wp_content_type_manager_v1_error::WpContentTypeManagerV1Error;

mod wp_content_type_manager_v1_destroy_request;
#[doc(inline)]
pub use wp_content_type_manager_v1_destroy_request::WpContentTypeManagerV1DestroyRequest;

mod wp_content_type_manager_v1_get_surface_content_type_request;
#[doc(inline)]
pub use wp_content_type_manager_v1_get_surface_content_type_request::WpContentTypeManagerV1GetSurfaceContentTypeRequest;

mod wp_content_type_v1_type;
#[doc(inline)]
pub use wp_content_type_v1_type::WpContentTypeV1Type;

mod wp_content_type_v1_destroy_request;
#[doc(inline)]
pub use wp_content_type_v1_destroy_request::WpContentTypeV1DestroyRequest;

mod wp_content_type_v1_set_content_type_request;
#[doc(inline)]
pub use wp_content_type_v1_set_content_type_request::WpContentTypeV1SetContentTypeRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
