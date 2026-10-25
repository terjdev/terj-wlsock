//! Encodes and decodes the pointer-gestures protocol family.
//!
//! Swipe, pinch and hold objects group semantic gesture events for core pointers.
//! Hold requests require interface version 3. The caller routes gestures to the
//! focused client and maintains serials, cancellation and gesture sequences.
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
//! Source: [pointer-gestures-unstable-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/unstable/pointer-gestures/pointer-gestures-unstable-v1.xml).

mod zwp_pointer_gestures_v1_get_swipe_gesture_request;
#[doc(inline)]
pub use zwp_pointer_gestures_v1_get_swipe_gesture_request::ZwpPointerGesturesV1GetSwipeGestureRequest;

mod zwp_pointer_gestures_v1_get_pinch_gesture_request;
#[doc(inline)]
pub use zwp_pointer_gestures_v1_get_pinch_gesture_request::ZwpPointerGesturesV1GetPinchGestureRequest;

mod zwp_pointer_gestures_v1_release_request;
#[doc(inline)]
pub use zwp_pointer_gestures_v1_release_request::ZwpPointerGesturesV1ReleaseRequest;

mod zwp_pointer_gestures_v1_get_hold_gesture_request;
#[doc(inline)]
pub use zwp_pointer_gestures_v1_get_hold_gesture_request::ZwpPointerGesturesV1GetHoldGestureRequest;

mod zwp_pointer_gesture_swipe_v1_destroy_request;
#[doc(inline)]
pub use zwp_pointer_gesture_swipe_v1_destroy_request::ZwpPointerGestureSwipeV1DestroyRequest;

mod zwp_pointer_gesture_swipe_v1_begin_event;
#[doc(inline)]
pub use zwp_pointer_gesture_swipe_v1_begin_event::ZwpPointerGestureSwipeV1BeginEvent;

mod zwp_pointer_gesture_swipe_v1_update_event;
#[doc(inline)]
pub use zwp_pointer_gesture_swipe_v1_update_event::ZwpPointerGestureSwipeV1UpdateEvent;

mod zwp_pointer_gesture_swipe_v1_end_event;
#[doc(inline)]
pub use zwp_pointer_gesture_swipe_v1_end_event::ZwpPointerGestureSwipeV1EndEvent;

mod zwp_pointer_gesture_pinch_v1_destroy_request;
#[doc(inline)]
pub use zwp_pointer_gesture_pinch_v1_destroy_request::ZwpPointerGesturePinchV1DestroyRequest;

mod zwp_pointer_gesture_pinch_v1_begin_event;
#[doc(inline)]
pub use zwp_pointer_gesture_pinch_v1_begin_event::ZwpPointerGesturePinchV1BeginEvent;

mod zwp_pointer_gesture_pinch_v1_update_event;
#[doc(inline)]
pub use zwp_pointer_gesture_pinch_v1_update_event::ZwpPointerGesturePinchV1UpdateEvent;

mod zwp_pointer_gesture_pinch_v1_end_event;
#[doc(inline)]
pub use zwp_pointer_gesture_pinch_v1_end_event::ZwpPointerGesturePinchV1EndEvent;

mod zwp_pointer_gesture_hold_v1_destroy_request;
#[doc(inline)]
pub use zwp_pointer_gesture_hold_v1_destroy_request::ZwpPointerGestureHoldV1DestroyRequest;

mod zwp_pointer_gesture_hold_v1_begin_event;
#[doc(inline)]
pub use zwp_pointer_gesture_hold_v1_begin_event::ZwpPointerGestureHoldV1BeginEvent;

mod zwp_pointer_gesture_hold_v1_end_event;
#[doc(inline)]
pub use zwp_pointer_gesture_hold_v1_end_event::ZwpPointerGestureHoldV1EndEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
