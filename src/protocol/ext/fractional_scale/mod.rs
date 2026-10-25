//! Encodes and decodes the fractional-scale extension.
//!
//! Per-surface objects advertise preferred fractional scales in units of 1/120.
//! The caller chooses scale values, tracks surface lifetimes and avoids duplicate
//! fractional-scale objects for the same surface.
//!
//! Codecs validate framing, receiver metadata, message versions and new object IDs.
//! Child versions inherit the parent version, capped at the target maximum; core
//! buffers use version 1. New IDs are registered before writes. A failed or
//! cancelled write after registration requires closing the connection. Client IDs
//! remain reserved until `wl_display.delete_id`; destructors release server IDs.
//! Strings and fixed values are checked for wire encoding. Nullable arguments use
//! `Option`; arrays retain raw bytes. Unknown enum values and flag bits are preserved.
//! The caller enforces protocol semantics, object relationships and compositor policy.
//!
//! Defined by upstream [fractional-scale-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/fractional-scale/fractional-scale-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod wp_fractional_scale_manager_v1_error;
#[doc(inline)]
pub use wp_fractional_scale_manager_v1_error::WpFractionalScaleManagerV1Error;

mod wp_fractional_scale_manager_v1_destroy_request;
#[doc(inline)]
pub use wp_fractional_scale_manager_v1_destroy_request::WpFractionalScaleManagerV1DestroyRequest;

mod wp_fractional_scale_manager_v1_get_fractional_scale_request;
#[doc(inline)]
pub use wp_fractional_scale_manager_v1_get_fractional_scale_request::WpFractionalScaleManagerV1GetFractionalScaleRequest;

mod wp_fractional_scale_v1_destroy_request;
#[doc(inline)]
pub use wp_fractional_scale_v1_destroy_request::WpFractionalScaleV1DestroyRequest;

mod wp_fractional_scale_v1_preferred_scale_event;
#[doc(inline)]
pub use wp_fractional_scale_v1_preferred_scale_event::WpFractionalScaleV1PreferredScaleEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
