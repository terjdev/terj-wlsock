//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZwpPointerGesturesV1 { version: u32 },
    ZwpPointerGestureSwipeV1 { version: u32 },
    ZwpPointerGesturePinchV1 { version: u32 },
    ZwpPointerGestureHoldV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZwpPointerGesturesV1 { .. } => "zwp_pointer_gestures_v1",
            Self::ZwpPointerGestureSwipeV1 { .. } => "zwp_pointer_gesture_swipe_v1",
            Self::ZwpPointerGesturePinchV1 { .. } => "zwp_pointer_gesture_pinch_v1",
            Self::ZwpPointerGestureHoldV1 { .. } => "zwp_pointer_gesture_hold_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZwpPointerGesturesV1 { version } => version,
            Self::ZwpPointerGestureSwipeV1 { version } => version,
            Self::ZwpPointerGesturePinchV1 { version } => version,
            Self::ZwpPointerGestureHoldV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZwpPointerGesturesV1 { .. } => 3,
            Self::ZwpPointerGestureSwipeV1 { .. } => 3,
            Self::ZwpPointerGesturePinchV1 { .. } => 3,
            Self::ZwpPointerGestureHoldV1 { .. } => 3,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwp_pointer_gestures_v1" => Self::ZwpPointerGesturesV1 { version },
            "zwp_pointer_gesture_swipe_v1" => Self::ZwpPointerGestureSwipeV1 { version },
            "zwp_pointer_gesture_pinch_v1" => Self::ZwpPointerGesturePinchV1 { version },
            "zwp_pointer_gesture_hold_v1" => Self::ZwpPointerGestureHoldV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
