//! Encodes and decodes the Viewporter extension, interface version 1.
//!
//! Defined by the upstream [Viewporter XML](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/stable/viewporter/viewporter.xml).
//! `wp_viewporter` creates a `wp_viewport` for cropping and scaling a `wl_surface`.
//! Source rectangles and destination sizes are independent, initially unset, and
//! double-buffered until `wl_surface.commit`. Buffer transform and buffer scale
//! precede cropping and scaling; source coordinates refer to that transformed space.
//! An unset source uses the whole buffer. A set destination determines the surface
//! size, except that a null buffer has no content or size. Without a destination,
//! a set source must have integer width and height when surface state is applied.
//! Destroying a viewport removes crop and scale state on the next surface commit;
//! destroying the viewporter leaves existing viewports unaffected.
//!
//! Codecs validate wire framing, receiver metadata, and fixed-point representability.
//! The caller enforces one viewport per surface, source bounds, positive sizes,
//! surface lifetimes, and commit-time protocol errors. Unknown error codes are preserved.
//! This extension defines no events. Original copyright and MIT permission terms
//! are preserved in this directory's LICENSE.md.

mod wp_viewporter_error;
#[doc(inline)]
pub use wp_viewporter_error::WpViewporterError;

mod wp_viewporter_destroy_request;
#[doc(inline)]
pub use wp_viewporter_destroy_request::WpViewporterDestroyRequest;

mod wp_viewporter_get_viewport_request;
#[doc(inline)]
pub use wp_viewporter_get_viewport_request::WpViewporterGetViewportRequest;

mod wp_viewport_error;
#[doc(inline)]
pub use wp_viewport_error::WpViewportError;

mod wp_viewport_destroy_request;
#[doc(inline)]
pub use wp_viewport_destroy_request::WpViewportDestroyRequest;

mod wp_viewport_set_source_request;
#[doc(inline)]
pub use wp_viewport_set_source_request::WpViewportSetSourceRequest;

mod wp_viewport_set_destination_request;
#[doc(inline)]
pub use wp_viewport_set_destination_request::WpViewportSetDestinationRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
