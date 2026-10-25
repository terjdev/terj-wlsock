//! Encodes and decodes WLR screencopy through interface version 3.
//!
//! A manager creates frames for complete outputs or regions in logical output
//! coordinates. Frames advertise buffer requirements; clients submit a compatible
//! `wl_buffer` with `copy` or `copy_with_damage`. Successful captures send flags
//! followed by a ready timestamp; failures send `failed`. Clients destroy frames
//! after either result. Version 2 adds damage reporting, and version 3 adds DMA-BUF
//! requirements and `buffer_done` to terminate buffer-type enumeration.
//!
//! Codecs check framing, receiver metadata, message versions and new object IDs.
//! Frames inherit the manager version, capped at the frame interface maximum.
//! New IDs are registered before writes; close the connection after failed or
//! cancelled writes following registration. Client IDs remain reserved until
//! `wl_display.delete_id`. Result events leave frame IDs registered until destroy.
//! Shared-memory formats reuse the core `WlShmFormat` type; DMA-BUF formats are raw
//! DRM fourcc values. Unknown error values and flag bits are preserved.
//!
//! The caller controls capture permissions, output and buffer lifetimes, region
//! clipping, buffer compatibility, one-shot copy use, damage tracking, event order,
//! timestamp validity and rendering. This upstream protocol is deprecated in favor
//! of `ext-image-copy-capture-v1`; it is provided for client compatibility.
//!
//! Defined by upstream [wlr-screencopy-unstable-v1.xml](https://gitlab.freedesktop.org/wlroots/wlr-protocols/-/blob/master/unstable/wlr-screencopy-unstable-v1.xml).
//! Original copyright and MIT permission terms are preserved in LICENSE.md.

mod zwlr_screencopy_manager_v1_capture_output_request;
#[doc(inline)]
pub use zwlr_screencopy_manager_v1_capture_output_request::ZwlrScreencopyManagerV1CaptureOutputRequest;

mod zwlr_screencopy_manager_v1_capture_output_region_request;
#[doc(inline)]
pub use zwlr_screencopy_manager_v1_capture_output_region_request::ZwlrScreencopyManagerV1CaptureOutputRegionRequest;

mod zwlr_screencopy_manager_v1_destroy_request;
#[doc(inline)]
pub use zwlr_screencopy_manager_v1_destroy_request::ZwlrScreencopyManagerV1DestroyRequest;

mod zwlr_screencopy_frame_v1_error;
#[doc(inline)]
pub use zwlr_screencopy_frame_v1_error::ZwlrScreencopyFrameV1Error;

mod zwlr_screencopy_frame_v1_flags;
#[doc(inline)]
pub use zwlr_screencopy_frame_v1_flags::ZwlrScreencopyFrameV1Flags;

mod zwlr_screencopy_frame_v1_copy_request;
#[doc(inline)]
pub use zwlr_screencopy_frame_v1_copy_request::ZwlrScreencopyFrameV1CopyRequest;

mod zwlr_screencopy_frame_v1_destroy_request;
#[doc(inline)]
pub use zwlr_screencopy_frame_v1_destroy_request::ZwlrScreencopyFrameV1DestroyRequest;

mod zwlr_screencopy_frame_v1_copy_with_damage_request;
#[doc(inline)]
pub use zwlr_screencopy_frame_v1_copy_with_damage_request::ZwlrScreencopyFrameV1CopyWithDamageRequest;

mod zwlr_screencopy_frame_v1_buffer_event;
#[doc(inline)]
pub use zwlr_screencopy_frame_v1_buffer_event::ZwlrScreencopyFrameV1BufferEvent;

mod zwlr_screencopy_frame_v1_flags_event;
#[doc(inline)]
pub use zwlr_screencopy_frame_v1_flags_event::ZwlrScreencopyFrameV1FlagsEvent;

mod zwlr_screencopy_frame_v1_ready_event;
#[doc(inline)]
pub use zwlr_screencopy_frame_v1_ready_event::ZwlrScreencopyFrameV1ReadyEvent;

mod zwlr_screencopy_frame_v1_failed_event;
#[doc(inline)]
pub use zwlr_screencopy_frame_v1_failed_event::ZwlrScreencopyFrameV1FailedEvent;

mod zwlr_screencopy_frame_v1_damage_event;
#[doc(inline)]
pub use zwlr_screencopy_frame_v1_damage_event::ZwlrScreencopyFrameV1DamageEvent;

mod zwlr_screencopy_frame_v1_linux_dmabuf_event;
#[doc(inline)]
pub use zwlr_screencopy_frame_v1_linux_dmabuf_event::ZwlrScreencopyFrameV1LinuxDmabufEvent;

mod zwlr_screencopy_frame_v1_buffer_done_event;
#[doc(inline)]
pub use zwlr_screencopy_frame_v1_buffer_done_event::ZwlrScreencopyFrameV1BufferDoneEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
