//! Encodes and decodes the xx-text-input protocol family.
//!
//! Experimental text input supports composition, surrounding text, cursor movement
//! and actions. It is distinct from zwp_text_input_v3. The caller manages focus,
//! supported features, commit batches, serials and UTF-8 byte offsets.
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
//! Source: [xx-text-input-v3.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/experimental/xx-text-input/xx-text-input-v3.xml).

mod xx_text_input_v3_change_cause;
#[doc(inline)]
pub use xx_text_input_v3_change_cause::XxTextInputV3ChangeCause;

mod xx_text_input_v3_content_hint;
#[doc(inline)]
pub use xx_text_input_v3_content_hint::XxTextInputV3ContentHint;

mod xx_text_input_v3_content_purpose;
#[doc(inline)]
pub use xx_text_input_v3_content_purpose::XxTextInputV3ContentPurpose;

mod xx_text_input_v3_action;
#[doc(inline)]
pub use xx_text_input_v3_action::XxTextInputV3Action;

mod xx_text_input_v3_supported_features;
#[doc(inline)]
pub use xx_text_input_v3_supported_features::XxTextInputV3SupportedFeatures;

mod xx_text_input_v3_destroy_request;
#[doc(inline)]
pub use xx_text_input_v3_destroy_request::XxTextInputV3DestroyRequest;

mod xx_text_input_v3_enable_request;
#[doc(inline)]
pub use xx_text_input_v3_enable_request::XxTextInputV3EnableRequest;

mod xx_text_input_v3_disable_request;
#[doc(inline)]
pub use xx_text_input_v3_disable_request::XxTextInputV3DisableRequest;

mod xx_text_input_v3_set_surrounding_text_request;
#[doc(inline)]
pub use xx_text_input_v3_set_surrounding_text_request::XxTextInputV3SetSurroundingTextRequest;

mod xx_text_input_v3_set_text_change_cause_request;
#[doc(inline)]
pub use xx_text_input_v3_set_text_change_cause_request::XxTextInputV3SetTextChangeCauseRequest;

mod xx_text_input_v3_set_content_type_request;
#[doc(inline)]
pub use xx_text_input_v3_set_content_type_request::XxTextInputV3SetContentTypeRequest;

mod xx_text_input_v3_set_cursor_rectangle_request;
#[doc(inline)]
pub use xx_text_input_v3_set_cursor_rectangle_request::XxTextInputV3SetCursorRectangleRequest;

mod xx_text_input_v3_commit_request;
#[doc(inline)]
pub use xx_text_input_v3_commit_request::XxTextInputV3CommitRequest;

mod xx_text_input_v3_set_available_actions_request;
#[doc(inline)]
pub use xx_text_input_v3_set_available_actions_request::XxTextInputV3SetAvailableActionsRequest;

mod xx_text_input_v3_announce_supported_features_request;
#[doc(inline)]
pub use xx_text_input_v3_announce_supported_features_request::XxTextInputV3AnnounceSupportedFeaturesRequest;

mod xx_text_input_v3_enter_event;
#[doc(inline)]
pub use xx_text_input_v3_enter_event::XxTextInputV3EnterEvent;

mod xx_text_input_v3_leave_event;
#[doc(inline)]
pub use xx_text_input_v3_leave_event::XxTextInputV3LeaveEvent;

mod xx_text_input_v3_preedit_string_event;
#[doc(inline)]
pub use xx_text_input_v3_preedit_string_event::XxTextInputV3PreeditStringEvent;

mod xx_text_input_v3_commit_string_event;
#[doc(inline)]
pub use xx_text_input_v3_commit_string_event::XxTextInputV3CommitStringEvent;

mod xx_text_input_v3_delete_surrounding_text_event;
#[doc(inline)]
pub use xx_text_input_v3_delete_surrounding_text_event::XxTextInputV3DeleteSurroundingTextEvent;

mod xx_text_input_v3_move_cursor_event;
#[doc(inline)]
pub use xx_text_input_v3_move_cursor_event::XxTextInputV3MoveCursorEvent;

mod xx_text_input_v3_done_event;
#[doc(inline)]
pub use xx_text_input_v3_done_event::XxTextInputV3DoneEvent;

mod xx_text_input_v3_perform_action_event;
#[doc(inline)]
pub use xx_text_input_v3_perform_action_event::XxTextInputV3PerformActionEvent;

mod xx_text_input_manager_v3_destroy_request;
#[doc(inline)]
pub use xx_text_input_manager_v3_destroy_request::XxTextInputManagerV3DestroyRequest;

mod xx_text_input_manager_v3_get_text_input_request;
#[doc(inline)]
pub use xx_text_input_manager_v3_get_text_input_request::XxTextInputManagerV3GetTextInputRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
