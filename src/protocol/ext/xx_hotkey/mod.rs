//! Encodes and decodes the xx-hotkey protocol family.
//!
//! Managers identify applications and create reconfigurable global hotkeys. Each
//! commit receives bound or denied; bindings can be revoked, triggered and released.
//! Modifier masks describe exact semantic modifiers. The caller validates triggers,
//! serials, app IDs, exclusivity, commit ordering and hotkey authorization.
//!
//! Codecs validate receiver protocol, opcode, minimum version and complete framing.
//! New child IDs inherit the receiver version capped at the target maximum.
//! Registration precedes writes; failed or cancelled
//! writes after registration require closing the connection. Client IDs remain
//! reserved until `wl_display.delete_id`; destructors release server IDs.
//! Fixed values use `f64` with signed 24.8 encoding. Strings and fixed values are
//! validated before writes. Nullable values use `Option`; arrays retain raw bytes.
//! Unknown enum values and flag bits are preserved. Protocol semantics belong to
//! the caller. Original copyright and permission terms are in LICENSE.md.
//! Source: [xx-hotkey-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/experimental/xx-hotkey/xx-hotkey-v1.xml).

mod xx_hotkey_manager_v1_error;
#[doc(inline)]
pub use xx_hotkey_manager_v1_error::XxHotkeyManagerV1Error;

mod xx_hotkey_manager_v1_modifiers;
#[doc(inline)]
pub use xx_hotkey_manager_v1_modifiers::XxHotkeyManagerV1Modifiers;

mod xx_hotkey_manager_v1_destroy_request;
#[doc(inline)]
pub use xx_hotkey_manager_v1_destroy_request::XxHotkeyManagerV1DestroyRequest;

mod xx_hotkey_manager_v1_set_app_id_request;
#[doc(inline)]
pub use xx_hotkey_manager_v1_set_app_id_request::XxHotkeyManagerV1SetAppIdRequest;

mod xx_hotkey_manager_v1_create_hotkey_request;
#[doc(inline)]
pub use xx_hotkey_manager_v1_create_hotkey_request::XxHotkeyManagerV1CreateHotkeyRequest;

mod xx_hotkey_v1_error;
#[doc(inline)]
pub use xx_hotkey_v1_error::XxHotkeyV1Error;

mod xx_hotkey_v1_deny_reason;
#[doc(inline)]
pub use xx_hotkey_v1_deny_reason::XxHotkeyV1DenyReason;

mod xx_hotkey_v1_destroy_request;
#[doc(inline)]
pub use xx_hotkey_v1_destroy_request::XxHotkeyV1DestroyRequest;

mod xx_hotkey_v1_set_description_request;
#[doc(inline)]
pub use xx_hotkey_v1_set_description_request::XxHotkeyV1SetDescriptionRequest;

mod xx_hotkey_v1_set_seat_request;
#[doc(inline)]
pub use xx_hotkey_v1_set_seat_request::XxHotkeyV1SetSeatRequest;

mod xx_hotkey_v1_set_key_trigger_request;
#[doc(inline)]
pub use xx_hotkey_v1_set_key_trigger_request::XxHotkeyV1SetKeyTriggerRequest;

mod xx_hotkey_v1_set_button_trigger_request;
#[doc(inline)]
pub use xx_hotkey_v1_set_button_trigger_request::XxHotkeyV1SetButtonTriggerRequest;

mod xx_hotkey_v1_commit_request;
#[doc(inline)]
pub use xx_hotkey_v1_commit_request::XxHotkeyV1CommitRequest;

mod xx_hotkey_v1_bound_event;
#[doc(inline)]
pub use xx_hotkey_v1_bound_event::XxHotkeyV1BoundEvent;

mod xx_hotkey_v1_denied_event;
#[doc(inline)]
pub use xx_hotkey_v1_denied_event::XxHotkeyV1DeniedEvent;

mod xx_hotkey_v1_revoked_event;
#[doc(inline)]
pub use xx_hotkey_v1_revoked_event::XxHotkeyV1RevokedEvent;

mod xx_hotkey_v1_triggered_event;
#[doc(inline)]
pub use xx_hotkey_v1_triggered_event::XxHotkeyV1TriggeredEvent;

mod xx_hotkey_v1_released_event;
#[doc(inline)]
pub use xx_hotkey_v1_released_event::XxHotkeyV1ReleasedEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
