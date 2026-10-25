//! Encodes and decodes the xx-fractional-scale protocol family.
//!
//! Experimental fractional scale uses unsigned 8.24 scale factors and supports
//! client scale requests. It is distinct from wp_fractional_scale_v1. The caller
//! validates scale values, surface uniqueness and commit-time state.
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
//! Source: [xx-fractional-scale-v2.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/experimental/xx-fractional-scale/xx-fractional-scale-v2.xml).

mod xx_fractional_scale_manager_v2_error;
#[doc(inline)]
pub use xx_fractional_scale_manager_v2_error::XxFractionalScaleManagerV2Error;

mod xx_fractional_scale_manager_v2_destroy_request;
#[doc(inline)]
pub use xx_fractional_scale_manager_v2_destroy_request::XxFractionalScaleManagerV2DestroyRequest;

mod xx_fractional_scale_manager_v2_get_fractional_scale_request;
#[doc(inline)]
pub use xx_fractional_scale_manager_v2_get_fractional_scale_request::XxFractionalScaleManagerV2GetFractionalScaleRequest;

mod xx_fractional_scale_v2_error;
#[doc(inline)]
pub use xx_fractional_scale_v2_error::XxFractionalScaleV2Error;

mod xx_fractional_scale_v2_set_scale_factor_request;
#[doc(inline)]
pub use xx_fractional_scale_v2_set_scale_factor_request::XxFractionalScaleV2SetScaleFactorRequest;

mod xx_fractional_scale_v2_destroy_request;
#[doc(inline)]
pub use xx_fractional_scale_v2_destroy_request::XxFractionalScaleV2DestroyRequest;

mod xx_fractional_scale_v2_scale_factor_event;
#[doc(inline)]
pub use xx_fractional_scale_v2_scale_factor_event::XxFractionalScaleV2ScaleFactorEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
