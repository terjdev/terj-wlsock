//! Encodes and decodes the gamma-control protocol family.
//!
//! Controls advertise a gamma-ramp size and accept an owned table FD. Tables contain
//! consecutive red, green and blue ramps of native-endian u16 values. Failed
//! controls remain registered until destroy. The caller validates table sizes,
//! manages exclusive output ownership and restores gamma state on destruction.
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
//! Source: [wlr-gamma-control-unstable-v1.xml](https://gitlab.freedesktop.org/wlroots/wlr-protocols/-/blob/master/unstable/wlr-gamma-control-unstable-v1.xml).

mod zwlr_gamma_control_manager_v1_get_gamma_control_request;
#[doc(inline)]
pub use zwlr_gamma_control_manager_v1_get_gamma_control_request::ZwlrGammaControlManagerV1GetGammaControlRequest;

mod zwlr_gamma_control_manager_v1_destroy_request;
#[doc(inline)]
pub use zwlr_gamma_control_manager_v1_destroy_request::ZwlrGammaControlManagerV1DestroyRequest;

mod zwlr_gamma_control_v1_error;
#[doc(inline)]
pub use zwlr_gamma_control_v1_error::ZwlrGammaControlV1Error;

mod zwlr_gamma_control_v1_set_gamma_request;
#[doc(inline)]
pub use zwlr_gamma_control_v1_set_gamma_request::ZwlrGammaControlV1SetGammaRequest;

mod zwlr_gamma_control_v1_destroy_request;
#[doc(inline)]
pub use zwlr_gamma_control_v1_destroy_request::ZwlrGammaControlV1DestroyRequest;

mod zwlr_gamma_control_v1_gamma_size_event;
#[doc(inline)]
pub use zwlr_gamma_control_v1_gamma_size_event::ZwlrGammaControlV1GammaSizeEvent;

mod zwlr_gamma_control_v1_failed_event;
#[doc(inline)]
pub use zwlr_gamma_control_v1_failed_event::ZwlrGammaControlV1FailedEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
