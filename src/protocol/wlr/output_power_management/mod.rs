//! Encodes and decodes the output-power-management protocol family.
//!
//! Per-output controls report and request on/off power states. Failed controls
//! become unusable and should be destroyed. The caller enforces exclusive control,
//! output lifetimes, permissions and backend power transitions.
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
//! Source: [wlr-output-power-management-unstable-v1.xml](https://gitlab.freedesktop.org/wlroots/wlr-protocols/-/blob/master/unstable/wlr-output-power-management-unstable-v1.xml).

mod zwlr_output_power_manager_v1_get_output_power_request;
#[doc(inline)]
pub use zwlr_output_power_manager_v1_get_output_power_request::ZwlrOutputPowerManagerV1GetOutputPowerRequest;

mod zwlr_output_power_manager_v1_destroy_request;
#[doc(inline)]
pub use zwlr_output_power_manager_v1_destroy_request::ZwlrOutputPowerManagerV1DestroyRequest;

mod zwlr_output_power_v1_mode;
#[doc(inline)]
pub use zwlr_output_power_v1_mode::ZwlrOutputPowerV1Mode;

mod zwlr_output_power_v1_error;
#[doc(inline)]
pub use zwlr_output_power_v1_error::ZwlrOutputPowerV1Error;

mod zwlr_output_power_v1_set_mode_request;
#[doc(inline)]
pub use zwlr_output_power_v1_set_mode_request::ZwlrOutputPowerV1SetModeRequest;

mod zwlr_output_power_v1_destroy_request;
#[doc(inline)]
pub use zwlr_output_power_v1_destroy_request::ZwlrOutputPowerV1DestroyRequest;

mod zwlr_output_power_v1_mode_event;
#[doc(inline)]
pub use zwlr_output_power_v1_mode_event::ZwlrOutputPowerV1ModeEvent;

mod zwlr_output_power_v1_failed_event;
#[doc(inline)]
pub use zwlr_output_power_v1_failed_event::ZwlrOutputPowerV1FailedEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
