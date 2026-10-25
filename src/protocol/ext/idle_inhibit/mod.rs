//! Encodes and decodes the idle-inhibit protocol family.
//!
//! Surface inhibitors request that visible content prevent idle behavior. The caller
//! tracks surface visibility, inhibitor lifetimes and idle/blanking policy.
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
//! Source: [idle-inhibit-unstable-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/unstable/idle-inhibit/idle-inhibit-unstable-v1.xml).

mod zwp_idle_inhibit_manager_v1_destroy_request;
#[doc(inline)]
pub use zwp_idle_inhibit_manager_v1_destroy_request::ZwpIdleInhibitManagerV1DestroyRequest;

mod zwp_idle_inhibit_manager_v1_create_inhibitor_request;
#[doc(inline)]
pub use zwp_idle_inhibit_manager_v1_create_inhibitor_request::ZwpIdleInhibitManagerV1CreateInhibitorRequest;

mod zwp_idle_inhibitor_v1_destroy_request;
#[doc(inline)]
pub use zwp_idle_inhibitor_v1_destroy_request::ZwpIdleInhibitorV1DestroyRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
