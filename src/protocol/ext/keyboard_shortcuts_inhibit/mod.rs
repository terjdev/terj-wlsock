//! Encodes and decodes the keyboard-shortcuts-inhibit protocol family.
//!
//! Per-surface, per-seat inhibitors advertise active and inactive states. The caller
//! manages focus, inhibition policy, uniqueness and seat/surface lifetimes.
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
//! Source: [keyboard-shortcuts-inhibit-unstable-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/unstable/keyboard-shortcuts-inhibit/keyboard-shortcuts-inhibit-unstable-v1.xml).

mod zwp_keyboard_shortcuts_inhibit_manager_v1_error;
#[doc(inline)]
pub use zwp_keyboard_shortcuts_inhibit_manager_v1_error::ZwpKeyboardShortcutsInhibitManagerV1Error;

mod zwp_keyboard_shortcuts_inhibit_manager_v1_destroy_request;
#[doc(inline)]
pub use zwp_keyboard_shortcuts_inhibit_manager_v1_destroy_request::ZwpKeyboardShortcutsInhibitManagerV1DestroyRequest;

mod zwp_keyboard_shortcuts_inhibit_manager_v1_inhibit_shortcuts_request;
#[doc(inline)]
pub use zwp_keyboard_shortcuts_inhibit_manager_v1_inhibit_shortcuts_request::ZwpKeyboardShortcutsInhibitManagerV1InhibitShortcutsRequest;

mod zwp_keyboard_shortcuts_inhibitor_v1_destroy_request;
#[doc(inline)]
pub use zwp_keyboard_shortcuts_inhibitor_v1_destroy_request::ZwpKeyboardShortcutsInhibitorV1DestroyRequest;

mod zwp_keyboard_shortcuts_inhibitor_v1_active_event;
#[doc(inline)]
pub use zwp_keyboard_shortcuts_inhibitor_v1_active_event::ZwpKeyboardShortcutsInhibitorV1ActiveEvent;

mod zwp_keyboard_shortcuts_inhibitor_v1_inactive_event;
#[doc(inline)]
pub use zwp_keyboard_shortcuts_inhibitor_v1_inactive_event::ZwpKeyboardShortcutsInhibitorV1InactiveEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
