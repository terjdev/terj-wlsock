//! Encodes and decodes the input-inhibitor protocol family.
//!
//! An inhibitor suppresses input to other clients and compositor shortcuts until
//! destruction. The caller restricts access to trusted clients, enforces a single
//! active inhibitor and implements input routing. This is distinct from session lock.
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
//! This protocol family defines no events.
//! Source: [wlr-input-inhibitor-unstable-v1.xml](https://gitlab.freedesktop.org/wlroots/wlr-protocols/-/blob/master/unstable/wlr-input-inhibitor-unstable-v1.xml).

mod zwlr_input_inhibit_manager_v1_error;
#[doc(inline)]
pub use zwlr_input_inhibit_manager_v1_error::ZwlrInputInhibitManagerV1Error;

mod zwlr_input_inhibit_manager_v1_get_inhibitor_request;
#[doc(inline)]
pub use zwlr_input_inhibit_manager_v1_get_inhibitor_request::ZwlrInputInhibitManagerV1GetInhibitorRequest;

mod zwlr_input_inhibitor_v1_destroy_request;
#[doc(inline)]
pub use zwlr_input_inhibitor_v1_destroy_request::ZwlrInputInhibitorV1DestroyRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
