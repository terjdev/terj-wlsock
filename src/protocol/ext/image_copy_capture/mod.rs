//! Encodes and decodes the ext-image-copy-capture extension.
//!
//! Sessions advertise buffer constraints, then capture frames into client buffers.
//! Cursor sessions add cursor position, hotspot and enter/leave updates. Arrays
//! preserve raw device and modifier bytes. The caller enforces buffer compatibility,
//! capture ordering, damage, timestamps, source permissions and performs rendering.
//!
//! Codecs validate framing, receiver metadata, message versions and new object IDs.
//! Child versions inherit the parent version, capped at the target maximum; core
//! buffers use version 1. New IDs are registered before writes. A failed or
//! cancelled write after registration requires closing the connection. Client IDs
//! remain reserved until `wl_display.delete_id`; destructors release server IDs.
//! Strings and fixed values are checked for wire encoding. Nullable arguments use
//! `Option`; arrays retain raw bytes. Unknown enum values and flag bits are preserved.
//! The caller enforces protocol semantics, object relationships and compositor policy.
//!
//! Defined by upstream [ext-image-copy-capture-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/ext-image-copy-capture/ext-image-copy-capture-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod ext_image_copy_capture_manager_v1_error;
#[doc(inline)]
pub use ext_image_copy_capture_manager_v1_error::ExtImageCopyCaptureManagerV1Error;

mod ext_image_copy_capture_manager_v1_options;
#[doc(inline)]
pub use ext_image_copy_capture_manager_v1_options::ExtImageCopyCaptureManagerV1Options;

mod ext_image_copy_capture_manager_v1_create_session_request;
#[doc(inline)]
pub use ext_image_copy_capture_manager_v1_create_session_request::ExtImageCopyCaptureManagerV1CreateSessionRequest;

mod ext_image_copy_capture_manager_v1_create_pointer_cursor_session_request;
#[doc(inline)]
pub use ext_image_copy_capture_manager_v1_create_pointer_cursor_session_request::ExtImageCopyCaptureManagerV1CreatePointerCursorSessionRequest;

mod ext_image_copy_capture_manager_v1_destroy_request;
#[doc(inline)]
pub use ext_image_copy_capture_manager_v1_destroy_request::ExtImageCopyCaptureManagerV1DestroyRequest;

mod ext_image_copy_capture_session_v1_error;
#[doc(inline)]
pub use ext_image_copy_capture_session_v1_error::ExtImageCopyCaptureSessionV1Error;

mod ext_image_copy_capture_session_v1_create_frame_request;
#[doc(inline)]
pub use ext_image_copy_capture_session_v1_create_frame_request::ExtImageCopyCaptureSessionV1CreateFrameRequest;

mod ext_image_copy_capture_session_v1_destroy_request;
#[doc(inline)]
pub use ext_image_copy_capture_session_v1_destroy_request::ExtImageCopyCaptureSessionV1DestroyRequest;

mod ext_image_copy_capture_session_v1_buffer_size_event;
#[doc(inline)]
pub use ext_image_copy_capture_session_v1_buffer_size_event::ExtImageCopyCaptureSessionV1BufferSizeEvent;

mod ext_image_copy_capture_session_v1_shm_format_event;
#[doc(inline)]
pub use ext_image_copy_capture_session_v1_shm_format_event::ExtImageCopyCaptureSessionV1ShmFormatEvent;

mod ext_image_copy_capture_session_v1_dmabuf_device_event;
#[doc(inline)]
pub use ext_image_copy_capture_session_v1_dmabuf_device_event::ExtImageCopyCaptureSessionV1DmabufDeviceEvent;

mod ext_image_copy_capture_session_v1_dmabuf_format_event;
#[doc(inline)]
pub use ext_image_copy_capture_session_v1_dmabuf_format_event::ExtImageCopyCaptureSessionV1DmabufFormatEvent;

mod ext_image_copy_capture_session_v1_done_event;
#[doc(inline)]
pub use ext_image_copy_capture_session_v1_done_event::ExtImageCopyCaptureSessionV1DoneEvent;

mod ext_image_copy_capture_session_v1_stopped_event;
#[doc(inline)]
pub use ext_image_copy_capture_session_v1_stopped_event::ExtImageCopyCaptureSessionV1StoppedEvent;

mod ext_image_copy_capture_frame_v1_error;
#[doc(inline)]
pub use ext_image_copy_capture_frame_v1_error::ExtImageCopyCaptureFrameV1Error;

mod ext_image_copy_capture_frame_v1_failure_reason;
#[doc(inline)]
pub use ext_image_copy_capture_frame_v1_failure_reason::ExtImageCopyCaptureFrameV1FailureReason;

mod ext_image_copy_capture_frame_v1_destroy_request;
#[doc(inline)]
pub use ext_image_copy_capture_frame_v1_destroy_request::ExtImageCopyCaptureFrameV1DestroyRequest;

mod ext_image_copy_capture_frame_v1_attach_buffer_request;
#[doc(inline)]
pub use ext_image_copy_capture_frame_v1_attach_buffer_request::ExtImageCopyCaptureFrameV1AttachBufferRequest;

mod ext_image_copy_capture_frame_v1_damage_buffer_request;
#[doc(inline)]
pub use ext_image_copy_capture_frame_v1_damage_buffer_request::ExtImageCopyCaptureFrameV1DamageBufferRequest;

mod ext_image_copy_capture_frame_v1_capture_request;
#[doc(inline)]
pub use ext_image_copy_capture_frame_v1_capture_request::ExtImageCopyCaptureFrameV1CaptureRequest;

mod ext_image_copy_capture_frame_v1_transform_event;
#[doc(inline)]
pub use ext_image_copy_capture_frame_v1_transform_event::ExtImageCopyCaptureFrameV1TransformEvent;

mod ext_image_copy_capture_frame_v1_damage_event;
#[doc(inline)]
pub use ext_image_copy_capture_frame_v1_damage_event::ExtImageCopyCaptureFrameV1DamageEvent;

mod ext_image_copy_capture_frame_v1_presentation_time_event;
#[doc(inline)]
pub use ext_image_copy_capture_frame_v1_presentation_time_event::ExtImageCopyCaptureFrameV1PresentationTimeEvent;

mod ext_image_copy_capture_frame_v1_ready_event;
#[doc(inline)]
pub use ext_image_copy_capture_frame_v1_ready_event::ExtImageCopyCaptureFrameV1ReadyEvent;

mod ext_image_copy_capture_frame_v1_failed_event;
#[doc(inline)]
pub use ext_image_copy_capture_frame_v1_failed_event::ExtImageCopyCaptureFrameV1FailedEvent;

mod ext_image_copy_capture_cursor_session_v1_error;
#[doc(inline)]
pub use ext_image_copy_capture_cursor_session_v1_error::ExtImageCopyCaptureCursorSessionV1Error;

mod ext_image_copy_capture_cursor_session_v1_destroy_request;
#[doc(inline)]
pub use ext_image_copy_capture_cursor_session_v1_destroy_request::ExtImageCopyCaptureCursorSessionV1DestroyRequest;

mod ext_image_copy_capture_cursor_session_v1_get_capture_session_request;
#[doc(inline)]
pub use ext_image_copy_capture_cursor_session_v1_get_capture_session_request::ExtImageCopyCaptureCursorSessionV1GetCaptureSessionRequest;

mod ext_image_copy_capture_cursor_session_v1_enter_event;
#[doc(inline)]
pub use ext_image_copy_capture_cursor_session_v1_enter_event::ExtImageCopyCaptureCursorSessionV1EnterEvent;

mod ext_image_copy_capture_cursor_session_v1_leave_event;
#[doc(inline)]
pub use ext_image_copy_capture_cursor_session_v1_leave_event::ExtImageCopyCaptureCursorSessionV1LeaveEvent;

mod ext_image_copy_capture_cursor_session_v1_position_event;
#[doc(inline)]
pub use ext_image_copy_capture_cursor_session_v1_position_event::ExtImageCopyCaptureCursorSessionV1PositionEvent;

mod ext_image_copy_capture_cursor_session_v1_hotspot_event;
#[doc(inline)]
pub use ext_image_copy_capture_cursor_session_v1_hotspot_event::ExtImageCopyCaptureCursorSessionV1HotspotEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
