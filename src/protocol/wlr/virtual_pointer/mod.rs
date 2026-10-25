//! Encodes and decodes WLR virtual-pointer through interface version 2.
//!
//! The manager creates virtual pointers with optional seat and output suggestions.
//! Pointers submit relative or absolute motion, buttons, scroll axes and frame
//! boundaries. Version 2 adds creation with an output suggestion. This protocol
//! defines no events. Button states, axes and axis sources reuse core wl_pointer
//! value types, preserving unknown values.
//!
//! Codecs check framing, receiver metadata, message versions and new object IDs.
//! Pointers inherit the manager version, capped at the interface maximum. New IDs
//! are registered before writes; close the connection after failed or cancelled
//! writes following registration. Client IDs remain reserved until
//! `wl_display.delete_id`. Nullable seats and outputs use `Option<u32>`.
//! Fixed arguments use `f64` with signed 24.8 wire encoding; writes reject
//! non-finite or out-of-range values before writing arguments.
//!
//! The caller controls client access, device and seat lifetimes, seat and output
//! assignments, absolute-coordinate bounds, axis validity, button semantics and
//! frame grouping. Destroying the manager leaves existing pointers unaffected.
//!
//! Defined by upstream [wlr-virtual-pointer-unstable-v1.xml](https://gitlab.freedesktop.org/wlroots/wlr-protocols/-/blob/master/unstable/wlr-virtual-pointer-unstable-v1.xml).
//! Original copyright and MIT permission terms are preserved in LICENSE.md.

mod zwlr_virtual_pointer_v1_error;
#[doc(inline)]
pub use zwlr_virtual_pointer_v1_error::ZwlrVirtualPointerV1Error;

mod zwlr_virtual_pointer_v1_motion_request;
#[doc(inline)]
pub use zwlr_virtual_pointer_v1_motion_request::ZwlrVirtualPointerV1MotionRequest;

mod zwlr_virtual_pointer_v1_motion_absolute_request;
#[doc(inline)]
pub use zwlr_virtual_pointer_v1_motion_absolute_request::ZwlrVirtualPointerV1MotionAbsoluteRequest;

mod zwlr_virtual_pointer_v1_button_request;
#[doc(inline)]
pub use zwlr_virtual_pointer_v1_button_request::ZwlrVirtualPointerV1ButtonRequest;

mod zwlr_virtual_pointer_v1_axis_request;
#[doc(inline)]
pub use zwlr_virtual_pointer_v1_axis_request::ZwlrVirtualPointerV1AxisRequest;

mod zwlr_virtual_pointer_v1_frame_request;
#[doc(inline)]
pub use zwlr_virtual_pointer_v1_frame_request::ZwlrVirtualPointerV1FrameRequest;

mod zwlr_virtual_pointer_v1_axis_source_request;
#[doc(inline)]
pub use zwlr_virtual_pointer_v1_axis_source_request::ZwlrVirtualPointerV1AxisSourceRequest;

mod zwlr_virtual_pointer_v1_axis_stop_request;
#[doc(inline)]
pub use zwlr_virtual_pointer_v1_axis_stop_request::ZwlrVirtualPointerV1AxisStopRequest;

mod zwlr_virtual_pointer_v1_axis_discrete_request;
#[doc(inline)]
pub use zwlr_virtual_pointer_v1_axis_discrete_request::ZwlrVirtualPointerV1AxisDiscreteRequest;

mod zwlr_virtual_pointer_v1_destroy_request;
#[doc(inline)]
pub use zwlr_virtual_pointer_v1_destroy_request::ZwlrVirtualPointerV1DestroyRequest;

mod zwlr_virtual_pointer_manager_v1_create_virtual_pointer_request;
#[doc(inline)]
pub use zwlr_virtual_pointer_manager_v1_create_virtual_pointer_request::ZwlrVirtualPointerManagerV1CreateVirtualPointerRequest;

mod zwlr_virtual_pointer_manager_v1_destroy_request;
#[doc(inline)]
pub use zwlr_virtual_pointer_manager_v1_destroy_request::ZwlrVirtualPointerManagerV1DestroyRequest;

mod zwlr_virtual_pointer_manager_v1_create_virtual_pointer_with_output_request;
#[doc(inline)]
pub use zwlr_virtual_pointer_manager_v1_create_virtual_pointer_with_output_request::ZwlrVirtualPointerManagerV1CreateVirtualPointerWithOutputRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
