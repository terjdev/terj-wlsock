//! Encodes and decodes the export-dmabuf protocol family.
//!
//! Captured frames advertise dimensions, modifiers and read-only DMA-BUF objects.
//! Each object transfers an owned FD, including on eventual cancellation. Ready
//! and cancel do not destroy frames. The caller handles capture permissions, plane
//! layouts, object counts, timestamps, rendering and buffer lifetimes.
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
//! Source: [wlr-export-dmabuf-unstable-v1.xml](https://gitlab.freedesktop.org/wlroots/wlr-protocols/-/blob/master/unstable/wlr-export-dmabuf-unstable-v1.xml).

mod zwlr_export_dmabuf_manager_v1_capture_output_request;
#[doc(inline)]
pub use zwlr_export_dmabuf_manager_v1_capture_output_request::ZwlrExportDmabufManagerV1CaptureOutputRequest;

mod zwlr_export_dmabuf_manager_v1_destroy_request;
#[doc(inline)]
pub use zwlr_export_dmabuf_manager_v1_destroy_request::ZwlrExportDmabufManagerV1DestroyRequest;

mod zwlr_export_dmabuf_frame_v1_flags;
#[doc(inline)]
pub use zwlr_export_dmabuf_frame_v1_flags::ZwlrExportDmabufFrameV1Flags;

mod zwlr_export_dmabuf_frame_v1_cancel_reason;
#[doc(inline)]
pub use zwlr_export_dmabuf_frame_v1_cancel_reason::ZwlrExportDmabufFrameV1CancelReason;

mod zwlr_export_dmabuf_frame_v1_destroy_request;
#[doc(inline)]
pub use zwlr_export_dmabuf_frame_v1_destroy_request::ZwlrExportDmabufFrameV1DestroyRequest;

mod zwlr_export_dmabuf_frame_v1_frame_event;
#[doc(inline)]
pub use zwlr_export_dmabuf_frame_v1_frame_event::ZwlrExportDmabufFrameV1FrameEvent;

mod zwlr_export_dmabuf_frame_v1_object_event;
#[doc(inline)]
pub use zwlr_export_dmabuf_frame_v1_object_event::ZwlrExportDmabufFrameV1ObjectEvent;

mod zwlr_export_dmabuf_frame_v1_ready_event;
#[doc(inline)]
pub use zwlr_export_dmabuf_frame_v1_ready_event::ZwlrExportDmabufFrameV1ReadyEvent;

mod zwlr_export_dmabuf_frame_v1_cancel_event;
#[doc(inline)]
pub use zwlr_export_dmabuf_frame_v1_cancel_event::ZwlrExportDmabufFrameV1CancelEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
