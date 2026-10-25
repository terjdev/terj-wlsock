//! Encodes and decodes the stable tablet-v2 protocol, through interface version 2.
//!
//! A tablet manager creates a tablet seat, which announces tablets, tools and pads.
//! Pads announce groups, and groups announce rings, strips and dials. Initial
//! descriptive events end with `done`; tool, ring, strip and dial updates are
//! accumulated until `frame`. Removal notifications leave resources alive until
//! the client sends their destroy requests. Pad feedback describes mode-dependent
//! actions, and tool cursors are independent of the core pointer cursor.
//!
//! Codecs validate framing, receiver metadata, message versions and new object IDs.
//! Child objects inherit their parent's negotiated version. Objects introduced by
//! events use the server ID range. New IDs are registered before writes; failed or
//! cancelled writes after registration require closing the connection. Client IDs
//! remain reserved until `wl_display.delete_id`; destructors release server IDs.
//! Nullable cursor surfaces use `Option<u32>`. Fixed-point arguments use `f64`
//! with signed 24.8 wire encoding; writes reject non-finite or out-of-range values.
//! Button arrays contain raw native-endian `u32` indices. Unknown enum values are
//! preserved, and strings are validated for wire encoding. Version 2 adds bus-type
//! notifications and pad dials.
//!
//! The caller enforces device lifetimes, description and frame ordering, proximity,
//! tool capabilities and axis ranges, cursor roles, focus and grab policy, group
//! membership, modes, and feedback serials. Paths are opaque system-specific strings.
//!
//! Derived from upstream [tablet-v2.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/stable/tablet/tablet-v2.xml).
//! Original copyright notices and MIT permission terms are preserved in LICENSE.md.

mod zwp_tablet_manager_v2_get_tablet_seat_request;
#[doc(inline)]
pub use zwp_tablet_manager_v2_get_tablet_seat_request::ZwpTabletManagerV2GetTabletSeatRequest;

mod zwp_tablet_manager_v2_destroy_request;
#[doc(inline)]
pub use zwp_tablet_manager_v2_destroy_request::ZwpTabletManagerV2DestroyRequest;

mod zwp_tablet_seat_v2_destroy_request;
#[doc(inline)]
pub use zwp_tablet_seat_v2_destroy_request::ZwpTabletSeatV2DestroyRequest;

mod zwp_tablet_seat_v2_tablet_added_event;
#[doc(inline)]
pub use zwp_tablet_seat_v2_tablet_added_event::ZwpTabletSeatV2TabletAddedEvent;

mod zwp_tablet_seat_v2_tool_added_event;
#[doc(inline)]
pub use zwp_tablet_seat_v2_tool_added_event::ZwpTabletSeatV2ToolAddedEvent;

mod zwp_tablet_seat_v2_pad_added_event;
#[doc(inline)]
pub use zwp_tablet_seat_v2_pad_added_event::ZwpTabletSeatV2PadAddedEvent;

mod zwp_tablet_tool_v2_type;
#[doc(inline)]
pub use zwp_tablet_tool_v2_type::ZwpTabletToolV2Type;

mod zwp_tablet_tool_v2_capability;
#[doc(inline)]
pub use zwp_tablet_tool_v2_capability::ZwpTabletToolV2Capability;

mod zwp_tablet_tool_v2_button_state;
#[doc(inline)]
pub use zwp_tablet_tool_v2_button_state::ZwpTabletToolV2ButtonState;

mod zwp_tablet_tool_v2_error;
#[doc(inline)]
pub use zwp_tablet_tool_v2_error::ZwpTabletToolV2Error;

mod zwp_tablet_tool_v2_set_cursor_request;
#[doc(inline)]
pub use zwp_tablet_tool_v2_set_cursor_request::ZwpTabletToolV2SetCursorRequest;

mod zwp_tablet_tool_v2_destroy_request;
#[doc(inline)]
pub use zwp_tablet_tool_v2_destroy_request::ZwpTabletToolV2DestroyRequest;

mod zwp_tablet_tool_v2_type_event;
#[doc(inline)]
pub use zwp_tablet_tool_v2_type_event::ZwpTabletToolV2TypeEvent;

mod zwp_tablet_tool_v2_hardware_serial_event;
#[doc(inline)]
pub use zwp_tablet_tool_v2_hardware_serial_event::ZwpTabletToolV2HardwareSerialEvent;

mod zwp_tablet_tool_v2_hardware_id_wacom_event;
#[doc(inline)]
pub use zwp_tablet_tool_v2_hardware_id_wacom_event::ZwpTabletToolV2HardwareIdWacomEvent;

mod zwp_tablet_tool_v2_capability_event;
#[doc(inline)]
pub use zwp_tablet_tool_v2_capability_event::ZwpTabletToolV2CapabilityEvent;

mod zwp_tablet_tool_v2_done_event;
#[doc(inline)]
pub use zwp_tablet_tool_v2_done_event::ZwpTabletToolV2DoneEvent;

mod zwp_tablet_tool_v2_removed_event;
#[doc(inline)]
pub use zwp_tablet_tool_v2_removed_event::ZwpTabletToolV2RemovedEvent;

mod zwp_tablet_tool_v2_proximity_in_event;
#[doc(inline)]
pub use zwp_tablet_tool_v2_proximity_in_event::ZwpTabletToolV2ProximityInEvent;

mod zwp_tablet_tool_v2_proximity_out_event;
#[doc(inline)]
pub use zwp_tablet_tool_v2_proximity_out_event::ZwpTabletToolV2ProximityOutEvent;

mod zwp_tablet_tool_v2_down_event;
#[doc(inline)]
pub use zwp_tablet_tool_v2_down_event::ZwpTabletToolV2DownEvent;

mod zwp_tablet_tool_v2_up_event;
#[doc(inline)]
pub use zwp_tablet_tool_v2_up_event::ZwpTabletToolV2UpEvent;

mod zwp_tablet_tool_v2_motion_event;
#[doc(inline)]
pub use zwp_tablet_tool_v2_motion_event::ZwpTabletToolV2MotionEvent;

mod zwp_tablet_tool_v2_pressure_event;
#[doc(inline)]
pub use zwp_tablet_tool_v2_pressure_event::ZwpTabletToolV2PressureEvent;

mod zwp_tablet_tool_v2_distance_event;
#[doc(inline)]
pub use zwp_tablet_tool_v2_distance_event::ZwpTabletToolV2DistanceEvent;

mod zwp_tablet_tool_v2_tilt_event;
#[doc(inline)]
pub use zwp_tablet_tool_v2_tilt_event::ZwpTabletToolV2TiltEvent;

mod zwp_tablet_tool_v2_rotation_event;
#[doc(inline)]
pub use zwp_tablet_tool_v2_rotation_event::ZwpTabletToolV2RotationEvent;

mod zwp_tablet_tool_v2_slider_event;
#[doc(inline)]
pub use zwp_tablet_tool_v2_slider_event::ZwpTabletToolV2SliderEvent;

mod zwp_tablet_tool_v2_wheel_event;
#[doc(inline)]
pub use zwp_tablet_tool_v2_wheel_event::ZwpTabletToolV2WheelEvent;

mod zwp_tablet_tool_v2_button_event;
#[doc(inline)]
pub use zwp_tablet_tool_v2_button_event::ZwpTabletToolV2ButtonEvent;

mod zwp_tablet_tool_v2_frame_event;
#[doc(inline)]
pub use zwp_tablet_tool_v2_frame_event::ZwpTabletToolV2FrameEvent;

mod zwp_tablet_v2_bustype;
#[doc(inline)]
pub use zwp_tablet_v2_bustype::ZwpTabletV2Bustype;

mod zwp_tablet_v2_destroy_request;
#[doc(inline)]
pub use zwp_tablet_v2_destroy_request::ZwpTabletV2DestroyRequest;

mod zwp_tablet_v2_name_event;
#[doc(inline)]
pub use zwp_tablet_v2_name_event::ZwpTabletV2NameEvent;

mod zwp_tablet_v2_id_event;
#[doc(inline)]
pub use zwp_tablet_v2_id_event::ZwpTabletV2IdEvent;

mod zwp_tablet_v2_path_event;
#[doc(inline)]
pub use zwp_tablet_v2_path_event::ZwpTabletV2PathEvent;

mod zwp_tablet_v2_done_event;
#[doc(inline)]
pub use zwp_tablet_v2_done_event::ZwpTabletV2DoneEvent;

mod zwp_tablet_v2_removed_event;
#[doc(inline)]
pub use zwp_tablet_v2_removed_event::ZwpTabletV2RemovedEvent;

mod zwp_tablet_v2_bustype_event;
#[doc(inline)]
pub use zwp_tablet_v2_bustype_event::ZwpTabletV2BustypeEvent;

mod zwp_tablet_pad_ring_v2_source;
#[doc(inline)]
pub use zwp_tablet_pad_ring_v2_source::ZwpTabletPadRingV2Source;

mod zwp_tablet_pad_ring_v2_set_feedback_request;
#[doc(inline)]
pub use zwp_tablet_pad_ring_v2_set_feedback_request::ZwpTabletPadRingV2SetFeedbackRequest;

mod zwp_tablet_pad_ring_v2_destroy_request;
#[doc(inline)]
pub use zwp_tablet_pad_ring_v2_destroy_request::ZwpTabletPadRingV2DestroyRequest;

mod zwp_tablet_pad_ring_v2_source_event;
#[doc(inline)]
pub use zwp_tablet_pad_ring_v2_source_event::ZwpTabletPadRingV2SourceEvent;

mod zwp_tablet_pad_ring_v2_angle_event;
#[doc(inline)]
pub use zwp_tablet_pad_ring_v2_angle_event::ZwpTabletPadRingV2AngleEvent;

mod zwp_tablet_pad_ring_v2_stop_event;
#[doc(inline)]
pub use zwp_tablet_pad_ring_v2_stop_event::ZwpTabletPadRingV2StopEvent;

mod zwp_tablet_pad_ring_v2_frame_event;
#[doc(inline)]
pub use zwp_tablet_pad_ring_v2_frame_event::ZwpTabletPadRingV2FrameEvent;

mod zwp_tablet_pad_strip_v2_source;
#[doc(inline)]
pub use zwp_tablet_pad_strip_v2_source::ZwpTabletPadStripV2Source;

mod zwp_tablet_pad_strip_v2_set_feedback_request;
#[doc(inline)]
pub use zwp_tablet_pad_strip_v2_set_feedback_request::ZwpTabletPadStripV2SetFeedbackRequest;

mod zwp_tablet_pad_strip_v2_destroy_request;
#[doc(inline)]
pub use zwp_tablet_pad_strip_v2_destroy_request::ZwpTabletPadStripV2DestroyRequest;

mod zwp_tablet_pad_strip_v2_source_event;
#[doc(inline)]
pub use zwp_tablet_pad_strip_v2_source_event::ZwpTabletPadStripV2SourceEvent;

mod zwp_tablet_pad_strip_v2_position_event;
#[doc(inline)]
pub use zwp_tablet_pad_strip_v2_position_event::ZwpTabletPadStripV2PositionEvent;

mod zwp_tablet_pad_strip_v2_stop_event;
#[doc(inline)]
pub use zwp_tablet_pad_strip_v2_stop_event::ZwpTabletPadStripV2StopEvent;

mod zwp_tablet_pad_strip_v2_frame_event;
#[doc(inline)]
pub use zwp_tablet_pad_strip_v2_frame_event::ZwpTabletPadStripV2FrameEvent;

mod zwp_tablet_pad_group_v2_destroy_request;
#[doc(inline)]
pub use zwp_tablet_pad_group_v2_destroy_request::ZwpTabletPadGroupV2DestroyRequest;

mod zwp_tablet_pad_group_v2_buttons_event;
#[doc(inline)]
pub use zwp_tablet_pad_group_v2_buttons_event::ZwpTabletPadGroupV2ButtonsEvent;

mod zwp_tablet_pad_group_v2_ring_event;
#[doc(inline)]
pub use zwp_tablet_pad_group_v2_ring_event::ZwpTabletPadGroupV2RingEvent;

mod zwp_tablet_pad_group_v2_strip_event;
#[doc(inline)]
pub use zwp_tablet_pad_group_v2_strip_event::ZwpTabletPadGroupV2StripEvent;

mod zwp_tablet_pad_group_v2_modes_event;
#[doc(inline)]
pub use zwp_tablet_pad_group_v2_modes_event::ZwpTabletPadGroupV2ModesEvent;

mod zwp_tablet_pad_group_v2_done_event;
#[doc(inline)]
pub use zwp_tablet_pad_group_v2_done_event::ZwpTabletPadGroupV2DoneEvent;

mod zwp_tablet_pad_group_v2_mode_switch_event;
#[doc(inline)]
pub use zwp_tablet_pad_group_v2_mode_switch_event::ZwpTabletPadGroupV2ModeSwitchEvent;

mod zwp_tablet_pad_group_v2_dial_event;
#[doc(inline)]
pub use zwp_tablet_pad_group_v2_dial_event::ZwpTabletPadGroupV2DialEvent;

mod zwp_tablet_pad_v2_button_state;
#[doc(inline)]
pub use zwp_tablet_pad_v2_button_state::ZwpTabletPadV2ButtonState;

mod zwp_tablet_pad_v2_set_feedback_request;
#[doc(inline)]
pub use zwp_tablet_pad_v2_set_feedback_request::ZwpTabletPadV2SetFeedbackRequest;

mod zwp_tablet_pad_v2_destroy_request;
#[doc(inline)]
pub use zwp_tablet_pad_v2_destroy_request::ZwpTabletPadV2DestroyRequest;

mod zwp_tablet_pad_v2_group_event;
#[doc(inline)]
pub use zwp_tablet_pad_v2_group_event::ZwpTabletPadV2GroupEvent;

mod zwp_tablet_pad_v2_path_event;
#[doc(inline)]
pub use zwp_tablet_pad_v2_path_event::ZwpTabletPadV2PathEvent;

mod zwp_tablet_pad_v2_buttons_event;
#[doc(inline)]
pub use zwp_tablet_pad_v2_buttons_event::ZwpTabletPadV2ButtonsEvent;

mod zwp_tablet_pad_v2_done_event;
#[doc(inline)]
pub use zwp_tablet_pad_v2_done_event::ZwpTabletPadV2DoneEvent;

mod zwp_tablet_pad_v2_button_event;
#[doc(inline)]
pub use zwp_tablet_pad_v2_button_event::ZwpTabletPadV2ButtonEvent;

mod zwp_tablet_pad_v2_enter_event;
#[doc(inline)]
pub use zwp_tablet_pad_v2_enter_event::ZwpTabletPadV2EnterEvent;

mod zwp_tablet_pad_v2_leave_event;
#[doc(inline)]
pub use zwp_tablet_pad_v2_leave_event::ZwpTabletPadV2LeaveEvent;

mod zwp_tablet_pad_v2_removed_event;
#[doc(inline)]
pub use zwp_tablet_pad_v2_removed_event::ZwpTabletPadV2RemovedEvent;

mod zwp_tablet_pad_dial_v2_set_feedback_request;
#[doc(inline)]
pub use zwp_tablet_pad_dial_v2_set_feedback_request::ZwpTabletPadDialV2SetFeedbackRequest;

mod zwp_tablet_pad_dial_v2_destroy_request;
#[doc(inline)]
pub use zwp_tablet_pad_dial_v2_destroy_request::ZwpTabletPadDialV2DestroyRequest;

mod zwp_tablet_pad_dial_v2_delta_event;
#[doc(inline)]
pub use zwp_tablet_pad_dial_v2_delta_event::ZwpTabletPadDialV2DeltaEvent;

mod zwp_tablet_pad_dial_v2_frame_event;
#[doc(inline)]
pub use zwp_tablet_pad_dial_v2_frame_event::ZwpTabletPadDialV2FrameEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
