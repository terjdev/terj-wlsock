//! Encodes and decodes the color-representation extension.
//!
//! Surfaces specify alpha modes, matrix coefficients, ranges and chroma locations.
//! The manager advertises supported combinations. The caller manages surface
//! associations, commit state, metadata validity and reconstruction of pixel colors.
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
//! Defined by upstream [color-representation-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/color-representation/color-representation-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod wp_color_representation_manager_v1_error;
#[doc(inline)]
pub use wp_color_representation_manager_v1_error::WpColorRepresentationManagerV1Error;

mod wp_color_representation_manager_v1_destroy_request;
#[doc(inline)]
pub use wp_color_representation_manager_v1_destroy_request::WpColorRepresentationManagerV1DestroyRequest;

mod wp_color_representation_manager_v1_get_surface_request;
#[doc(inline)]
pub use wp_color_representation_manager_v1_get_surface_request::WpColorRepresentationManagerV1GetSurfaceRequest;

mod wp_color_representation_manager_v1_supported_alpha_mode_event;
#[doc(inline)]
pub use wp_color_representation_manager_v1_supported_alpha_mode_event::WpColorRepresentationManagerV1SupportedAlphaModeEvent;

mod wp_color_representation_manager_v1_supported_coefficients_and_ranges_event;
#[doc(inline)]
pub use wp_color_representation_manager_v1_supported_coefficients_and_ranges_event::WpColorRepresentationManagerV1SupportedCoefficientsAndRangesEvent;

mod wp_color_representation_manager_v1_done_event;
#[doc(inline)]
pub use wp_color_representation_manager_v1_done_event::WpColorRepresentationManagerV1DoneEvent;

mod wp_color_representation_surface_v1_error;
#[doc(inline)]
pub use wp_color_representation_surface_v1_error::WpColorRepresentationSurfaceV1Error;

mod wp_color_representation_surface_v1_alpha_mode;
#[doc(inline)]
pub use wp_color_representation_surface_v1_alpha_mode::WpColorRepresentationSurfaceV1AlphaMode;

mod wp_color_representation_surface_v1_coefficients;
#[doc(inline)]
pub use wp_color_representation_surface_v1_coefficients::WpColorRepresentationSurfaceV1Coefficients;

mod wp_color_representation_surface_v1_range;
#[doc(inline)]
pub use wp_color_representation_surface_v1_range::WpColorRepresentationSurfaceV1Range;

mod wp_color_representation_surface_v1_chroma_location;
#[doc(inline)]
pub use wp_color_representation_surface_v1_chroma_location::WpColorRepresentationSurfaceV1ChromaLocation;

mod wp_color_representation_surface_v1_destroy_request;
#[doc(inline)]
pub use wp_color_representation_surface_v1_destroy_request::WpColorRepresentationSurfaceV1DestroyRequest;

mod wp_color_representation_surface_v1_set_alpha_mode_request;
#[doc(inline)]
pub use wp_color_representation_surface_v1_set_alpha_mode_request::WpColorRepresentationSurfaceV1SetAlphaModeRequest;

mod wp_color_representation_surface_v1_set_coefficients_and_range_request;
#[doc(inline)]
pub use wp_color_representation_surface_v1_set_coefficients_and_range_request::WpColorRepresentationSurfaceV1SetCoefficientsAndRangeRequest;

mod wp_color_representation_surface_v1_set_chroma_location_request;
#[doc(inline)]
pub use wp_color_representation_surface_v1_set_chroma_location_request::WpColorRepresentationSurfaceV1SetChromaLocationRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
