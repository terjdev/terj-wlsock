//! Encodes and decodes the Linux DMA-BUF extension, through interface version 6.
//!
//! Buffer parameters collect DMA-BUF planes and create core `wl_buffer` objects,
//! asynchronously with `create` or immediately with `create_immed`. File descriptors
//! are owned and transferred through the ancillary-data channel, without payload bytes.
//! Feedback supplies a read-only format-table FD and preference tranches. Device
//! arrays contain native `dev_t` bytes; format indices contain native-endian `u16`
//! values. The table contains 16-byte entries: a `u32` format, four padding bytes,
//! and a `u64` modifier. Arrays remain raw bytes; mapping and interpreting them is
//! the caller's responsibility.
//!
//! Codecs validate framing, message versions, receiver metadata and object creation.
//! New extension objects inherit their parent's version; core buffers use version 1.
//! Client IDs remain reserved until `wl_display.delete_id`. Failed or cancelled
//! writes after object registration require closing the connection.
//! The caller enforces plane completeness, modifier compatibility, dimensions,
//! one-shot parameter use, device-array sizes, feedback ordering, synchronization,
//! and buffer import policy. Unknown enum values and flag bits are preserved.
//! Legacy format/modifier events are prohibited from version 4; main_device is
//! prohibited from version 6. Version 6 adds sampling-device selection and feedback.
//!
//! Derived from upstream `linux-dmabuf-v1.xml`; original MIT terms are in LICENSE.md.

mod zwp_linux_dmabuf_v1_destroy_request;
#[doc(inline)]
pub use zwp_linux_dmabuf_v1_destroy_request::ZwpLinuxDmabufV1DestroyRequest;

mod zwp_linux_dmabuf_v1_create_params_request;
#[doc(inline)]
pub use zwp_linux_dmabuf_v1_create_params_request::ZwpLinuxDmabufV1CreateParamsRequest;

mod zwp_linux_dmabuf_v1_get_default_feedback_request;
#[doc(inline)]
pub use zwp_linux_dmabuf_v1_get_default_feedback_request::ZwpLinuxDmabufV1GetDefaultFeedbackRequest;

mod zwp_linux_dmabuf_v1_get_surface_feedback_request;
#[doc(inline)]
pub use zwp_linux_dmabuf_v1_get_surface_feedback_request::ZwpLinuxDmabufV1GetSurfaceFeedbackRequest;

mod zwp_linux_dmabuf_v1_format_event;
#[doc(inline)]
pub use zwp_linux_dmabuf_v1_format_event::ZwpLinuxDmabufV1FormatEvent;

mod zwp_linux_dmabuf_v1_modifier_event;
#[doc(inline)]
pub use zwp_linux_dmabuf_v1_modifier_event::ZwpLinuxDmabufV1ModifierEvent;

mod zwp_linux_buffer_params_v1_error;
#[doc(inline)]
pub use zwp_linux_buffer_params_v1_error::ZwpLinuxBufferParamsV1Error;

mod zwp_linux_buffer_params_v1_flags;
#[doc(inline)]
pub use zwp_linux_buffer_params_v1_flags::ZwpLinuxBufferParamsV1Flags;

mod zwp_linux_buffer_params_v1_destroy_request;
#[doc(inline)]
pub use zwp_linux_buffer_params_v1_destroy_request::ZwpLinuxBufferParamsV1DestroyRequest;

mod zwp_linux_buffer_params_v1_add_request;
#[doc(inline)]
pub use zwp_linux_buffer_params_v1_add_request::ZwpLinuxBufferParamsV1AddRequest;

mod zwp_linux_buffer_params_v1_create_request;
#[doc(inline)]
pub use zwp_linux_buffer_params_v1_create_request::ZwpLinuxBufferParamsV1CreateRequest;

mod zwp_linux_buffer_params_v1_create_immed_request;
#[doc(inline)]
pub use zwp_linux_buffer_params_v1_create_immed_request::ZwpLinuxBufferParamsV1CreateImmedRequest;

mod zwp_linux_buffer_params_v1_set_sampling_device_request;
#[doc(inline)]
pub use zwp_linux_buffer_params_v1_set_sampling_device_request::ZwpLinuxBufferParamsV1SetSamplingDeviceRequest;

mod zwp_linux_buffer_params_v1_created_event;
#[doc(inline)]
pub use zwp_linux_buffer_params_v1_created_event::ZwpLinuxBufferParamsV1CreatedEvent;

mod zwp_linux_buffer_params_v1_failed_event;
#[doc(inline)]
pub use zwp_linux_buffer_params_v1_failed_event::ZwpLinuxBufferParamsV1FailedEvent;

mod zwp_linux_dmabuf_feedback_v1_tranche_flags;
#[doc(inline)]
pub use zwp_linux_dmabuf_feedback_v1_tranche_flags::ZwpLinuxDmabufFeedbackV1TrancheFlags;

mod zwp_linux_dmabuf_feedback_v1_destroy_request;
#[doc(inline)]
pub use zwp_linux_dmabuf_feedback_v1_destroy_request::ZwpLinuxDmabufFeedbackV1DestroyRequest;

mod zwp_linux_dmabuf_feedback_v1_done_event;
#[doc(inline)]
pub use zwp_linux_dmabuf_feedback_v1_done_event::ZwpLinuxDmabufFeedbackV1DoneEvent;

mod zwp_linux_dmabuf_feedback_v1_format_table_event;
#[doc(inline)]
pub use zwp_linux_dmabuf_feedback_v1_format_table_event::ZwpLinuxDmabufFeedbackV1FormatTableEvent;

mod zwp_linux_dmabuf_feedback_v1_main_device_event;
#[doc(inline)]
pub use zwp_linux_dmabuf_feedback_v1_main_device_event::ZwpLinuxDmabufFeedbackV1MainDeviceEvent;

mod zwp_linux_dmabuf_feedback_v1_tranche_done_event;
#[doc(inline)]
pub use zwp_linux_dmabuf_feedback_v1_tranche_done_event::ZwpLinuxDmabufFeedbackV1TrancheDoneEvent;

mod zwp_linux_dmabuf_feedback_v1_tranche_target_device_event;
#[doc(inline)]
pub use zwp_linux_dmabuf_feedback_v1_tranche_target_device_event::ZwpLinuxDmabufFeedbackV1TrancheTargetDeviceEvent;

mod zwp_linux_dmabuf_feedback_v1_tranche_formats_event;
#[doc(inline)]
pub use zwp_linux_dmabuf_feedback_v1_tranche_formats_event::ZwpLinuxDmabufFeedbackV1TrancheFormatsEvent;

mod zwp_linux_dmabuf_feedback_v1_tranche_flags_event;
#[doc(inline)]
pub use zwp_linux_dmabuf_feedback_v1_tranche_flags_event::ZwpLinuxDmabufFeedbackV1TrancheFlagsEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
