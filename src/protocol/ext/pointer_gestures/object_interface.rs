//! Defines interfaces and negotiated versions for this protocol.
/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ZwpPointerGesturesV1 { version: u32 },
    ZwpPointerGestureSwipeV1 { version: u32 },
    ZwpPointerGesturePinchV1 { version: u32 },
    ZwpPointerGestureHoldV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ZwpPointerGesturesV1 { .. } => "zwp_pointer_gestures_v1",
            Self::ZwpPointerGestureSwipeV1 { .. } => "zwp_pointer_gesture_swipe_v1",
            Self::ZwpPointerGesturePinchV1 { .. } => "zwp_pointer_gesture_pinch_v1",
            Self::ZwpPointerGestureHoldV1 { .. } => "zwp_pointer_gesture_hold_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ZwpPointerGesturesV1 { version } => version,
            Self::ZwpPointerGestureSwipeV1 { version } => version,
            Self::ZwpPointerGesturePinchV1 { version } => version,
            Self::ZwpPointerGestureHoldV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ZwpPointerGesturesV1 { .. } => 3,
            Self::ZwpPointerGestureSwipeV1 { .. } => 3,
            Self::ZwpPointerGesturePinchV1 { .. } => 3,
            Self::ZwpPointerGestureHoldV1 { .. } => 3,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
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
