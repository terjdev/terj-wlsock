//! Encodes and decodes the input-timestamps protocol family.
//!
//! Timestamp objects attach high-resolution timestamps to core keyboard, pointer or
//! touch events. The caller preserves event association, clock domains, ordering
//! and timestamp validity, and makes objects inert when the input device disappears.
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
//! Source: [input-timestamps-unstable-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/unstable/input-timestamps/input-timestamps-unstable-v1.xml).

mod zwp_input_timestamps_manager_v1_destroy_request;
#[doc(inline)]
pub use zwp_input_timestamps_manager_v1_destroy_request::ZwpInputTimestampsManagerV1DestroyRequest;

mod zwp_input_timestamps_manager_v1_get_keyboard_timestamps_request;
#[doc(inline)]
pub use zwp_input_timestamps_manager_v1_get_keyboard_timestamps_request::ZwpInputTimestampsManagerV1GetKeyboardTimestampsRequest;

mod zwp_input_timestamps_manager_v1_get_pointer_timestamps_request;
#[doc(inline)]
pub use zwp_input_timestamps_manager_v1_get_pointer_timestamps_request::ZwpInputTimestampsManagerV1GetPointerTimestampsRequest;

mod zwp_input_timestamps_manager_v1_get_touch_timestamps_request;
#[doc(inline)]
pub use zwp_input_timestamps_manager_v1_get_touch_timestamps_request::ZwpInputTimestampsManagerV1GetTouchTimestampsRequest;

mod zwp_input_timestamps_v1_destroy_request;
#[doc(inline)]
pub use zwp_input_timestamps_v1_destroy_request::ZwpInputTimestampsV1DestroyRequest;

mod zwp_input_timestamps_v1_timestamp_event;
#[doc(inline)]
pub use zwp_input_timestamps_v1_timestamp_event::ZwpInputTimestampsV1TimestampEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
