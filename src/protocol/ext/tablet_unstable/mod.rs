//! Encodes and decodes the tablet-unstable protocol family.
//!
//! This is the legacy tablet-v1 protocol with distinct _v1 wire interfaces. The
//! stable tablet module covers tablet-v2 instead. The caller manages tool proximity,
//! frames, hardware identity, capabilities, axis ranges, cursor roles and removal.
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
//! Source: [tablet-unstable-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/unstable/tablet/tablet-unstable-v1.xml).

mod zwp_tablet_manager_v1_get_tablet_seat_request;
#[doc(inline)]
pub use zwp_tablet_manager_v1_get_tablet_seat_request::ZwpTabletManagerV1GetTabletSeatRequest;

mod zwp_tablet_manager_v1_destroy_request;
#[doc(inline)]
pub use zwp_tablet_manager_v1_destroy_request::ZwpTabletManagerV1DestroyRequest;

mod zwp_tablet_seat_v1_destroy_request;
#[doc(inline)]
pub use zwp_tablet_seat_v1_destroy_request::ZwpTabletSeatV1DestroyRequest;

mod zwp_tablet_seat_v1_tablet_added_event;
#[doc(inline)]
pub use zwp_tablet_seat_v1_tablet_added_event::ZwpTabletSeatV1TabletAddedEvent;

mod zwp_tablet_seat_v1_tool_added_event;
#[doc(inline)]
pub use zwp_tablet_seat_v1_tool_added_event::ZwpTabletSeatV1ToolAddedEvent;

mod zwp_tablet_tool_v1_type;
#[doc(inline)]
pub use zwp_tablet_tool_v1_type::ZwpTabletToolV1Type;

mod zwp_tablet_tool_v1_capability;
#[doc(inline)]
pub use zwp_tablet_tool_v1_capability::ZwpTabletToolV1Capability;

mod zwp_tablet_tool_v1_button_state;
#[doc(inline)]
pub use zwp_tablet_tool_v1_button_state::ZwpTabletToolV1ButtonState;

mod zwp_tablet_tool_v1_error;
#[doc(inline)]
pub use zwp_tablet_tool_v1_error::ZwpTabletToolV1Error;

mod zwp_tablet_tool_v1_set_cursor_request;
#[doc(inline)]
pub use zwp_tablet_tool_v1_set_cursor_request::ZwpTabletToolV1SetCursorRequest;

mod zwp_tablet_tool_v1_destroy_request;
#[doc(inline)]
pub use zwp_tablet_tool_v1_destroy_request::ZwpTabletToolV1DestroyRequest;

mod zwp_tablet_tool_v1_type_event;
#[doc(inline)]
pub use zwp_tablet_tool_v1_type_event::ZwpTabletToolV1TypeEvent;

mod zwp_tablet_tool_v1_hardware_serial_event;
#[doc(inline)]
pub use zwp_tablet_tool_v1_hardware_serial_event::ZwpTabletToolV1HardwareSerialEvent;

mod zwp_tablet_tool_v1_hardware_id_wacom_event;
#[doc(inline)]
pub use zwp_tablet_tool_v1_hardware_id_wacom_event::ZwpTabletToolV1HardwareIdWacomEvent;

mod zwp_tablet_tool_v1_capability_event;
#[doc(inline)]
pub use zwp_tablet_tool_v1_capability_event::ZwpTabletToolV1CapabilityEvent;

mod zwp_tablet_tool_v1_done_event;
#[doc(inline)]
pub use zwp_tablet_tool_v1_done_event::ZwpTabletToolV1DoneEvent;

mod zwp_tablet_tool_v1_removed_event;
#[doc(inline)]
pub use zwp_tablet_tool_v1_removed_event::ZwpTabletToolV1RemovedEvent;

mod zwp_tablet_tool_v1_proximity_in_event;
#[doc(inline)]
pub use zwp_tablet_tool_v1_proximity_in_event::ZwpTabletToolV1ProximityInEvent;

mod zwp_tablet_tool_v1_proximity_out_event;
#[doc(inline)]
pub use zwp_tablet_tool_v1_proximity_out_event::ZwpTabletToolV1ProximityOutEvent;

mod zwp_tablet_tool_v1_down_event;
#[doc(inline)]
pub use zwp_tablet_tool_v1_down_event::ZwpTabletToolV1DownEvent;

mod zwp_tablet_tool_v1_up_event;
#[doc(inline)]
pub use zwp_tablet_tool_v1_up_event::ZwpTabletToolV1UpEvent;

mod zwp_tablet_tool_v1_motion_event;
#[doc(inline)]
pub use zwp_tablet_tool_v1_motion_event::ZwpTabletToolV1MotionEvent;

mod zwp_tablet_tool_v1_pressure_event;
#[doc(inline)]
pub use zwp_tablet_tool_v1_pressure_event::ZwpTabletToolV1PressureEvent;

mod zwp_tablet_tool_v1_distance_event;
#[doc(inline)]
pub use zwp_tablet_tool_v1_distance_event::ZwpTabletToolV1DistanceEvent;

mod zwp_tablet_tool_v1_tilt_event;
#[doc(inline)]
pub use zwp_tablet_tool_v1_tilt_event::ZwpTabletToolV1TiltEvent;

mod zwp_tablet_tool_v1_rotation_event;
#[doc(inline)]
pub use zwp_tablet_tool_v1_rotation_event::ZwpTabletToolV1RotationEvent;

mod zwp_tablet_tool_v1_slider_event;
#[doc(inline)]
pub use zwp_tablet_tool_v1_slider_event::ZwpTabletToolV1SliderEvent;

mod zwp_tablet_tool_v1_wheel_event;
#[doc(inline)]
pub use zwp_tablet_tool_v1_wheel_event::ZwpTabletToolV1WheelEvent;

mod zwp_tablet_tool_v1_button_event;
#[doc(inline)]
pub use zwp_tablet_tool_v1_button_event::ZwpTabletToolV1ButtonEvent;

mod zwp_tablet_tool_v1_frame_event;
#[doc(inline)]
pub use zwp_tablet_tool_v1_frame_event::ZwpTabletToolV1FrameEvent;

mod zwp_tablet_v1_destroy_request;
#[doc(inline)]
pub use zwp_tablet_v1_destroy_request::ZwpTabletV1DestroyRequest;

mod zwp_tablet_v1_name_event;
#[doc(inline)]
pub use zwp_tablet_v1_name_event::ZwpTabletV1NameEvent;

mod zwp_tablet_v1_id_event;
#[doc(inline)]
pub use zwp_tablet_v1_id_event::ZwpTabletV1IdEvent;

mod zwp_tablet_v1_path_event;
#[doc(inline)]
pub use zwp_tablet_v1_path_event::ZwpTabletV1PathEvent;

mod zwp_tablet_v1_done_event;
#[doc(inline)]
pub use zwp_tablet_v1_done_event::ZwpTabletV1DoneEvent;

mod zwp_tablet_v1_removed_event;
#[doc(inline)]
pub use zwp_tablet_v1_removed_event::ZwpTabletV1RemovedEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
