//! Encodes and decodes the xdg-output protocol family.
//!
//! Outputs advertise logical positions, sizes, names and descriptions. From version
//! 3, wl_output.done replaces zxdg_output_v1.done. The caller groups updates and
//! maintains logical output geometry, naming and lifetimes.
//!
//! Codecs validate receiver protocol, opcode, minimum version and complete framing.
//! New child IDs inherit the receiver version capped at the target maximum; core
//! keyboard objects use version 1. Registration precedes writes; failed or cancelled
//! writes after registration require closing the connection. Client IDs remain
//! reserved until `wl_display.delete_id`; destructors release server IDs.
//! Fixed values use `f64` with signed 24.8 encoding. Strings and fixed values are
//! validated before writes. Nullable values use `Option`; arrays retain raw bytes.
//! Unknown enum values and flag bits are preserved. Protocol semantics belong to
//! the caller. Original copyright and permission terms are in LICENSE.md.
//! Source: [xdg-output-unstable-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/unstable/xdg-output/xdg-output-unstable-v1.xml).

mod zxdg_output_manager_v1_destroy_request;
#[doc(inline)]
pub use zxdg_output_manager_v1_destroy_request::ZxdgOutputManagerV1DestroyRequest;

mod zxdg_output_manager_v1_get_xdg_output_request;
#[doc(inline)]
pub use zxdg_output_manager_v1_get_xdg_output_request::ZxdgOutputManagerV1GetXdgOutputRequest;

mod zxdg_output_v1_destroy_request;
#[doc(inline)]
pub use zxdg_output_v1_destroy_request::ZxdgOutputV1DestroyRequest;

mod zxdg_output_v1_logical_position_event;
#[doc(inline)]
pub use zxdg_output_v1_logical_position_event::ZxdgOutputV1LogicalPositionEvent;

mod zxdg_output_v1_logical_size_event;
#[doc(inline)]
pub use zxdg_output_v1_logical_size_event::ZxdgOutputV1LogicalSizeEvent;

mod zxdg_output_v1_done_event;
#[doc(inline)]
pub use zxdg_output_v1_done_event::ZxdgOutputV1DoneEvent;

mod zxdg_output_v1_name_event;
#[doc(inline)]
pub use zxdg_output_v1_name_event::ZxdgOutputV1NameEvent;

mod zxdg_output_v1_description_event;
#[doc(inline)]
pub use zxdg_output_v1_description_event::ZxdgOutputV1DescriptionEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
