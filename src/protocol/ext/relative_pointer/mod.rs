//! Encodes and decodes the relative-pointer protocol family.
//!
//! Relative events carry accelerated and unaccelerated fixed-point deltas and split
//! microsecond timestamps. The caller routes motion according to pointer focus
//! and integrates with pointer locks without clamping deltas at output boundaries.
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
//! Source: [relative-pointer-unstable-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/unstable/relative-pointer/relative-pointer-unstable-v1.xml).

mod zwp_relative_pointer_manager_v1_destroy_request;
#[doc(inline)]
pub use zwp_relative_pointer_manager_v1_destroy_request::ZwpRelativePointerManagerV1DestroyRequest;

mod zwp_relative_pointer_manager_v1_get_relative_pointer_request;
#[doc(inline)]
pub use zwp_relative_pointer_manager_v1_get_relative_pointer_request::ZwpRelativePointerManagerV1GetRelativePointerRequest;

mod zwp_relative_pointer_v1_destroy_request;
#[doc(inline)]
pub use zwp_relative_pointer_v1_destroy_request::ZwpRelativePointerV1DestroyRequest;

mod zwp_relative_pointer_v1_relative_motion_event;
#[doc(inline)]
pub use zwp_relative_pointer_v1_relative_motion_event::ZwpRelativePointerV1RelativeMotionEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
