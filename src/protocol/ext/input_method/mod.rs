//! Encodes and decodes the input-method protocol family.
//!
//! Legacy input-method-v1 contexts are introduced by server events. Keyboard grabs
//! create core wl_keyboard objects at version 1. The caller relays text-input state,
//! manages context activation, input-panel roles, serials, focus and keyboard grabs.
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
//! Source: [input-method-unstable-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/unstable/input-method/input-method-unstable-v1.xml).

mod zwp_input_method_context_v1_destroy_request;
#[doc(inline)]
pub use zwp_input_method_context_v1_destroy_request::ZwpInputMethodContextV1DestroyRequest;

mod zwp_input_method_context_v1_commit_string_request;
#[doc(inline)]
pub use zwp_input_method_context_v1_commit_string_request::ZwpInputMethodContextV1CommitStringRequest;

mod zwp_input_method_context_v1_preedit_string_request;
#[doc(inline)]
pub use zwp_input_method_context_v1_preedit_string_request::ZwpInputMethodContextV1PreeditStringRequest;

mod zwp_input_method_context_v1_preedit_styling_request;
#[doc(inline)]
pub use zwp_input_method_context_v1_preedit_styling_request::ZwpInputMethodContextV1PreeditStylingRequest;

mod zwp_input_method_context_v1_preedit_cursor_request;
#[doc(inline)]
pub use zwp_input_method_context_v1_preedit_cursor_request::ZwpInputMethodContextV1PreeditCursorRequest;

mod zwp_input_method_context_v1_delete_surrounding_text_request;
#[doc(inline)]
pub use zwp_input_method_context_v1_delete_surrounding_text_request::ZwpInputMethodContextV1DeleteSurroundingTextRequest;

mod zwp_input_method_context_v1_cursor_position_request;
#[doc(inline)]
pub use zwp_input_method_context_v1_cursor_position_request::ZwpInputMethodContextV1CursorPositionRequest;

mod zwp_input_method_context_v1_modifiers_map_request;
#[doc(inline)]
pub use zwp_input_method_context_v1_modifiers_map_request::ZwpInputMethodContextV1ModifiersMapRequest;

mod zwp_input_method_context_v1_keysym_request;
#[doc(inline)]
pub use zwp_input_method_context_v1_keysym_request::ZwpInputMethodContextV1KeysymRequest;

mod zwp_input_method_context_v1_grab_keyboard_request;
#[doc(inline)]
pub use zwp_input_method_context_v1_grab_keyboard_request::ZwpInputMethodContextV1GrabKeyboardRequest;

mod zwp_input_method_context_v1_key_request;
#[doc(inline)]
pub use zwp_input_method_context_v1_key_request::ZwpInputMethodContextV1KeyRequest;

mod zwp_input_method_context_v1_modifiers_request;
#[doc(inline)]
pub use zwp_input_method_context_v1_modifiers_request::ZwpInputMethodContextV1ModifiersRequest;

mod zwp_input_method_context_v1_language_request;
#[doc(inline)]
pub use zwp_input_method_context_v1_language_request::ZwpInputMethodContextV1LanguageRequest;

mod zwp_input_method_context_v1_text_direction_request;
#[doc(inline)]
pub use zwp_input_method_context_v1_text_direction_request::ZwpInputMethodContextV1TextDirectionRequest;

mod zwp_input_method_context_v1_surrounding_text_event;
#[doc(inline)]
pub use zwp_input_method_context_v1_surrounding_text_event::ZwpInputMethodContextV1SurroundingTextEvent;

mod zwp_input_method_context_v1_reset_event;
#[doc(inline)]
pub use zwp_input_method_context_v1_reset_event::ZwpInputMethodContextV1ResetEvent;

mod zwp_input_method_context_v1_content_type_event;
#[doc(inline)]
pub use zwp_input_method_context_v1_content_type_event::ZwpInputMethodContextV1ContentTypeEvent;

mod zwp_input_method_context_v1_invoke_action_event;
#[doc(inline)]
pub use zwp_input_method_context_v1_invoke_action_event::ZwpInputMethodContextV1InvokeActionEvent;

mod zwp_input_method_context_v1_commit_state_event;
#[doc(inline)]
pub use zwp_input_method_context_v1_commit_state_event::ZwpInputMethodContextV1CommitStateEvent;

mod zwp_input_method_context_v1_preferred_language_event;
#[doc(inline)]
pub use zwp_input_method_context_v1_preferred_language_event::ZwpInputMethodContextV1PreferredLanguageEvent;

mod zwp_input_method_v1_activate_event;
#[doc(inline)]
pub use zwp_input_method_v1_activate_event::ZwpInputMethodV1ActivateEvent;

mod zwp_input_method_v1_deactivate_event;
#[doc(inline)]
pub use zwp_input_method_v1_deactivate_event::ZwpInputMethodV1DeactivateEvent;

mod zwp_input_panel_v1_get_input_panel_surface_request;
#[doc(inline)]
pub use zwp_input_panel_v1_get_input_panel_surface_request::ZwpInputPanelV1GetInputPanelSurfaceRequest;

mod zwp_input_panel_surface_v1_position;
#[doc(inline)]
pub use zwp_input_panel_surface_v1_position::ZwpInputPanelSurfaceV1Position;

mod zwp_input_panel_surface_v1_set_toplevel_request;
#[doc(inline)]
pub use zwp_input_panel_surface_v1_set_toplevel_request::ZwpInputPanelSurfaceV1SetToplevelRequest;

mod zwp_input_panel_surface_v1_set_overlay_panel_request;
#[doc(inline)]
pub use zwp_input_panel_surface_v1_set_overlay_panel_request::ZwpInputPanelSurfaceV1SetOverlayPanelRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
