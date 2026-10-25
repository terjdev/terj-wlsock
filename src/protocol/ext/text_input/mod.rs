//! Encodes and decodes the text-input protocol family.
//!
//! Both text-input-v1 and text-input-v3 are supported as distinct wire interfaces.
//! Version 2 of the v3 interfaces adds IME actions, language and preedit hints.
//! Nullable preedit/commit strings are preserved. The caller manages activation,
//! UTF-8 byte offsets, composition state, commit serials and input-method integration.
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
//! Source: [text-input-unstable-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/unstable/text-input/text-input-unstable-v1.xml).
//! Source: [text-input-unstable-v3.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/unstable/text-input/text-input-unstable-v3.xml).

mod zwp_text_input_v1_content_hint;
#[doc(inline)]
pub use zwp_text_input_v1_content_hint::ZwpTextInputV1ContentHint;

mod zwp_text_input_v1_content_purpose;
#[doc(inline)]
pub use zwp_text_input_v1_content_purpose::ZwpTextInputV1ContentPurpose;

mod zwp_text_input_v1_preedit_style;
#[doc(inline)]
pub use zwp_text_input_v1_preedit_style::ZwpTextInputV1PreeditStyle;

mod zwp_text_input_v1_text_direction;
#[doc(inline)]
pub use zwp_text_input_v1_text_direction::ZwpTextInputV1TextDirection;

mod zwp_text_input_v1_activate_request;
#[doc(inline)]
pub use zwp_text_input_v1_activate_request::ZwpTextInputV1ActivateRequest;

mod zwp_text_input_v1_deactivate_request;
#[doc(inline)]
pub use zwp_text_input_v1_deactivate_request::ZwpTextInputV1DeactivateRequest;

mod zwp_text_input_v1_show_input_panel_request;
#[doc(inline)]
pub use zwp_text_input_v1_show_input_panel_request::ZwpTextInputV1ShowInputPanelRequest;

mod zwp_text_input_v1_hide_input_panel_request;
#[doc(inline)]
pub use zwp_text_input_v1_hide_input_panel_request::ZwpTextInputV1HideInputPanelRequest;

mod zwp_text_input_v1_reset_request;
#[doc(inline)]
pub use zwp_text_input_v1_reset_request::ZwpTextInputV1ResetRequest;

mod zwp_text_input_v1_set_surrounding_text_request;
#[doc(inline)]
pub use zwp_text_input_v1_set_surrounding_text_request::ZwpTextInputV1SetSurroundingTextRequest;

mod zwp_text_input_v1_set_content_type_request;
#[doc(inline)]
pub use zwp_text_input_v1_set_content_type_request::ZwpTextInputV1SetContentTypeRequest;

mod zwp_text_input_v1_set_cursor_rectangle_request;
#[doc(inline)]
pub use zwp_text_input_v1_set_cursor_rectangle_request::ZwpTextInputV1SetCursorRectangleRequest;

mod zwp_text_input_v1_set_preferred_language_request;
#[doc(inline)]
pub use zwp_text_input_v1_set_preferred_language_request::ZwpTextInputV1SetPreferredLanguageRequest;

mod zwp_text_input_v1_commit_state_request;
#[doc(inline)]
pub use zwp_text_input_v1_commit_state_request::ZwpTextInputV1CommitStateRequest;

mod zwp_text_input_v1_invoke_action_request;
#[doc(inline)]
pub use zwp_text_input_v1_invoke_action_request::ZwpTextInputV1InvokeActionRequest;

mod zwp_text_input_v1_enter_event;
#[doc(inline)]
pub use zwp_text_input_v1_enter_event::ZwpTextInputV1EnterEvent;

mod zwp_text_input_v1_leave_event;
#[doc(inline)]
pub use zwp_text_input_v1_leave_event::ZwpTextInputV1LeaveEvent;

mod zwp_text_input_v1_modifiers_map_event;
#[doc(inline)]
pub use zwp_text_input_v1_modifiers_map_event::ZwpTextInputV1ModifiersMapEvent;

mod zwp_text_input_v1_input_panel_state_event;
#[doc(inline)]
pub use zwp_text_input_v1_input_panel_state_event::ZwpTextInputV1InputPanelStateEvent;

mod zwp_text_input_v1_preedit_string_event;
#[doc(inline)]
pub use zwp_text_input_v1_preedit_string_event::ZwpTextInputV1PreeditStringEvent;

mod zwp_text_input_v1_preedit_styling_event;
#[doc(inline)]
pub use zwp_text_input_v1_preedit_styling_event::ZwpTextInputV1PreeditStylingEvent;

mod zwp_text_input_v1_preedit_cursor_event;
#[doc(inline)]
pub use zwp_text_input_v1_preedit_cursor_event::ZwpTextInputV1PreeditCursorEvent;

mod zwp_text_input_v1_commit_string_event;
#[doc(inline)]
pub use zwp_text_input_v1_commit_string_event::ZwpTextInputV1CommitStringEvent;

mod zwp_text_input_v1_cursor_position_event;
#[doc(inline)]
pub use zwp_text_input_v1_cursor_position_event::ZwpTextInputV1CursorPositionEvent;

mod zwp_text_input_v1_delete_surrounding_text_event;
#[doc(inline)]
pub use zwp_text_input_v1_delete_surrounding_text_event::ZwpTextInputV1DeleteSurroundingTextEvent;

mod zwp_text_input_v1_keysym_event;
#[doc(inline)]
pub use zwp_text_input_v1_keysym_event::ZwpTextInputV1KeysymEvent;

mod zwp_text_input_v1_language_event;
#[doc(inline)]
pub use zwp_text_input_v1_language_event::ZwpTextInputV1LanguageEvent;

mod zwp_text_input_v1_text_direction_event;
#[doc(inline)]
pub use zwp_text_input_v1_text_direction_event::ZwpTextInputV1TextDirectionEvent;

mod zwp_text_input_manager_v1_create_text_input_request;
#[doc(inline)]
pub use zwp_text_input_manager_v1_create_text_input_request::ZwpTextInputManagerV1CreateTextInputRequest;

mod zwp_text_input_v3_change_cause;
#[doc(inline)]
pub use zwp_text_input_v3_change_cause::ZwpTextInputV3ChangeCause;

mod zwp_text_input_v3_content_hint;
#[doc(inline)]
pub use zwp_text_input_v3_content_hint::ZwpTextInputV3ContentHint;

mod zwp_text_input_v3_content_purpose;
#[doc(inline)]
pub use zwp_text_input_v3_content_purpose::ZwpTextInputV3ContentPurpose;

mod zwp_text_input_v3_error;
#[doc(inline)]
pub use zwp_text_input_v3_error::ZwpTextInputV3Error;

mod zwp_text_input_v3_action;
#[doc(inline)]
pub use zwp_text_input_v3_action::ZwpTextInputV3Action;

mod zwp_text_input_v3_preedit_hint;
#[doc(inline)]
pub use zwp_text_input_v3_preedit_hint::ZwpTextInputV3PreeditHint;

mod zwp_text_input_v3_destroy_request;
#[doc(inline)]
pub use zwp_text_input_v3_destroy_request::ZwpTextInputV3DestroyRequest;

mod zwp_text_input_v3_enable_request;
#[doc(inline)]
pub use zwp_text_input_v3_enable_request::ZwpTextInputV3EnableRequest;

mod zwp_text_input_v3_disable_request;
#[doc(inline)]
pub use zwp_text_input_v3_disable_request::ZwpTextInputV3DisableRequest;

mod zwp_text_input_v3_set_surrounding_text_request;
#[doc(inline)]
pub use zwp_text_input_v3_set_surrounding_text_request::ZwpTextInputV3SetSurroundingTextRequest;

mod zwp_text_input_v3_set_text_change_cause_request;
#[doc(inline)]
pub use zwp_text_input_v3_set_text_change_cause_request::ZwpTextInputV3SetTextChangeCauseRequest;

mod zwp_text_input_v3_set_content_type_request;
#[doc(inline)]
pub use zwp_text_input_v3_set_content_type_request::ZwpTextInputV3SetContentTypeRequest;

mod zwp_text_input_v3_set_cursor_rectangle_request;
#[doc(inline)]
pub use zwp_text_input_v3_set_cursor_rectangle_request::ZwpTextInputV3SetCursorRectangleRequest;

mod zwp_text_input_v3_commit_request;
#[doc(inline)]
pub use zwp_text_input_v3_commit_request::ZwpTextInputV3CommitRequest;

mod zwp_text_input_v3_set_available_actions_request;
#[doc(inline)]
pub use zwp_text_input_v3_set_available_actions_request::ZwpTextInputV3SetAvailableActionsRequest;

mod zwp_text_input_v3_show_input_panel_request;
#[doc(inline)]
pub use zwp_text_input_v3_show_input_panel_request::ZwpTextInputV3ShowInputPanelRequest;

mod zwp_text_input_v3_hide_input_panel_request;
#[doc(inline)]
pub use zwp_text_input_v3_hide_input_panel_request::ZwpTextInputV3HideInputPanelRequest;

mod zwp_text_input_v3_enter_event;
#[doc(inline)]
pub use zwp_text_input_v3_enter_event::ZwpTextInputV3EnterEvent;

mod zwp_text_input_v3_leave_event;
#[doc(inline)]
pub use zwp_text_input_v3_leave_event::ZwpTextInputV3LeaveEvent;

mod zwp_text_input_v3_preedit_string_event;
#[doc(inline)]
pub use zwp_text_input_v3_preedit_string_event::ZwpTextInputV3PreeditStringEvent;

mod zwp_text_input_v3_commit_string_event;
#[doc(inline)]
pub use zwp_text_input_v3_commit_string_event::ZwpTextInputV3CommitStringEvent;

mod zwp_text_input_v3_delete_surrounding_text_event;
#[doc(inline)]
pub use zwp_text_input_v3_delete_surrounding_text_event::ZwpTextInputV3DeleteSurroundingTextEvent;

mod zwp_text_input_v3_done_event;
#[doc(inline)]
pub use zwp_text_input_v3_done_event::ZwpTextInputV3DoneEvent;

mod zwp_text_input_v3_action_event;
#[doc(inline)]
pub use zwp_text_input_v3_action_event::ZwpTextInputV3ActionEvent;

mod zwp_text_input_v3_language_event;
#[doc(inline)]
pub use zwp_text_input_v3_language_event::ZwpTextInputV3LanguageEvent;

mod zwp_text_input_v3_preedit_hint_event;
#[doc(inline)]
pub use zwp_text_input_v3_preedit_hint_event::ZwpTextInputV3PreeditHintEvent;

mod zwp_text_input_manager_v3_destroy_request;
#[doc(inline)]
pub use zwp_text_input_manager_v3_destroy_request::ZwpTextInputManagerV3DestroyRequest;

mod zwp_text_input_manager_v3_get_text_input_request;
#[doc(inline)]
pub use zwp_text_input_manager_v3_get_text_input_request::ZwpTextInputManagerV3GetTextInputRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
