//! Encodes and decodes the xx-image-capture-color-management protocol family.
//!
//! Capture-source color objects advertise preferred image-description identities.
//! get_preferred creates a version-1 wp_image_description_reference_v1 from the
//! color-management protocol. This feature enables its codec dependency. The caller
//! manages capture-source lifetimes, image readiness, identities, descriptions,
//! capture sequencing and color conversion.
//!
//! Codecs validate receiver protocol, opcode, minimum version and complete framing.
//! New child IDs inherit the receiver version capped at the target maximum.
//! Registration precedes writes; failed or cancelled
//! writes after registration require closing the connection. Client IDs remain
//! reserved until `wl_display.delete_id`; destructors release server IDs.
//! Fixed values use `f64` with signed 24.8 encoding. Strings and fixed values are
//! validated before writes. Nullable values use `Option`; arrays retain raw bytes.
//! Unknown enum values and flag bits are preserved. Protocol semantics belong to
//! the caller. Original copyright and permission terms are in LICENSE.md.
//! Source: [xx-image-capture-color-management-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/experimental/xx-image-capture-color-management/xx-image-capture-color-management-v1.xml).

mod xx_image_capture_color_manager_v1_error;
#[doc(inline)]
pub use xx_image_capture_color_manager_v1_error::XxImageCaptureColorManagerV1Error;

mod xx_image_capture_color_manager_v1_destroy_request;
#[doc(inline)]
pub use xx_image_capture_color_manager_v1_destroy_request::XxImageCaptureColorManagerV1DestroyRequest;

mod xx_image_capture_color_manager_v1_get_capture_source_colors_request;
#[doc(inline)]
pub use xx_image_capture_color_manager_v1_get_capture_source_colors_request::XxImageCaptureColorManagerV1GetCaptureSourceColorsRequest;

mod xx_image_capture_color_manager_v1_set_frame_image_description_request;
#[doc(inline)]
pub use xx_image_capture_color_manager_v1_set_frame_image_description_request::XxImageCaptureColorManagerV1SetFrameImageDescriptionRequest;

mod xx_image_capture_source_colors_v1_destroy_request;
#[doc(inline)]
pub use xx_image_capture_source_colors_v1_destroy_request::XxImageCaptureSourceColorsV1DestroyRequest;

mod xx_image_capture_source_colors_v1_get_preferred_request;
#[doc(inline)]
pub use xx_image_capture_source_colors_v1_get_preferred_request::XxImageCaptureSourceColorsV1GetPreferredRequest;

mod xx_image_capture_source_colors_v1_preferred_changed_event;
#[doc(inline)]
pub use xx_image_capture_source_colors_v1_preferred_changed_event::XxImageCaptureSourceColorsV1PreferredChangedEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
