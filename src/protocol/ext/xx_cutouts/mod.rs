//! Encodes and decodes the xx-cutouts protocol family.
//!
//! Surface objects advertise hardware cutout boxes and rounded corners, grouped
//! by configure. Element IDs are valid for one configure sequence; unhandled IDs
//! are native-endian u32 arrays. The caller validates roles and IDs, orders object
//! destruction, applies acknowledgement state and positions content around cutouts.
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
//! Source: [xx-cutouts-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/experimental/xx-cutouts/xx-cutouts-v1.xml).

mod xx_cutouts_manager_v1_error;
#[doc(inline)]
pub use xx_cutouts_manager_v1_error::XxCutoutsManagerV1Error;

mod xx_cutouts_manager_v1_destroy_request;
#[doc(inline)]
pub use xx_cutouts_manager_v1_destroy_request::XxCutoutsManagerV1DestroyRequest;

mod xx_cutouts_manager_v1_get_cutouts_request;
#[doc(inline)]
pub use xx_cutouts_manager_v1_get_cutouts_request::XxCutoutsManagerV1GetCutoutsRequest;

mod xx_cutouts_v1_type;
#[doc(inline)]
pub use xx_cutouts_v1_type::XxCutoutsV1Type;

mod xx_cutouts_v1_corner_position;
#[doc(inline)]
pub use xx_cutouts_v1_corner_position::XxCutoutsV1CornerPosition;

mod xx_cutouts_v1_error;
#[doc(inline)]
pub use xx_cutouts_v1_error::XxCutoutsV1Error;

mod xx_cutouts_v1_destroy_request;
#[doc(inline)]
pub use xx_cutouts_v1_destroy_request::XxCutoutsV1DestroyRequest;

mod xx_cutouts_v1_set_unhandled_request;
#[doc(inline)]
pub use xx_cutouts_v1_set_unhandled_request::XxCutoutsV1SetUnhandledRequest;

mod xx_cutouts_v1_cutout_box_event;
#[doc(inline)]
pub use xx_cutouts_v1_cutout_box_event::XxCutoutsV1CutoutBoxEvent;

mod xx_cutouts_v1_cutout_corner_event;
#[doc(inline)]
pub use xx_cutouts_v1_cutout_corner_event::XxCutoutsV1CutoutCornerEvent;

mod xx_cutouts_v1_configure_event;
#[doc(inline)]
pub use xx_cutouts_v1_configure_event::XxCutoutsV1ConfigureEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
