//! Encodes and decodes the xx-input-method protocol family.
//!
//! Experimental input methods support commit batches, surrounding text, actions
//! and positioned popup surfaces. The caller manages activation, serials, UTF-8
//! offsets, popup configure acknowledgements, placement and object lifetimes.
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
//! Source: [xx-input-method-v2.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/experimental/xx-input-method/xx-input-method-v2.xml).

mod xx_input_method_v1_error;
#[doc(inline)]
pub use xx_input_method_v1_error::XxInputMethodV1Error;

mod xx_input_method_v1_protocol_compat;
#[doc(inline)]
pub use xx_input_method_v1_protocol_compat::XxInputMethodV1ProtocolCompat;

mod xx_input_method_v1_perform_action_request;
#[doc(inline)]
pub use xx_input_method_v1_perform_action_request::XxInputMethodV1PerformActionRequest;

mod xx_input_method_v1_commit_string_request;
#[doc(inline)]
pub use xx_input_method_v1_commit_string_request::XxInputMethodV1CommitStringRequest;

mod xx_input_method_v1_set_preedit_string_request;
#[doc(inline)]
pub use xx_input_method_v1_set_preedit_string_request::XxInputMethodV1SetPreeditStringRequest;

mod xx_input_method_v1_delete_surrounding_text_request;
#[doc(inline)]
pub use xx_input_method_v1_delete_surrounding_text_request::XxInputMethodV1DeleteSurroundingTextRequest;

mod xx_input_method_v1_move_cursor_request;
#[doc(inline)]
pub use xx_input_method_v1_move_cursor_request::XxInputMethodV1MoveCursorRequest;

mod xx_input_method_v1_commit_request;
#[doc(inline)]
pub use xx_input_method_v1_commit_request::XxInputMethodV1CommitRequest;

mod xx_input_method_v1_get_input_popup_surface_request;
#[doc(inline)]
pub use xx_input_method_v1_get_input_popup_surface_request::XxInputMethodV1GetInputPopupSurfaceRequest;

mod xx_input_method_v1_destroy_request;
#[doc(inline)]
pub use xx_input_method_v1_destroy_request::XxInputMethodV1DestroyRequest;

mod xx_input_method_v1_activate_event;
#[doc(inline)]
pub use xx_input_method_v1_activate_event::XxInputMethodV1ActivateEvent;

mod xx_input_method_v1_deactivate_event;
#[doc(inline)]
pub use xx_input_method_v1_deactivate_event::XxInputMethodV1DeactivateEvent;

mod xx_input_method_v1_surrounding_text_event;
#[doc(inline)]
pub use xx_input_method_v1_surrounding_text_event::XxInputMethodV1SurroundingTextEvent;

mod xx_input_method_v1_text_change_cause_event;
#[doc(inline)]
pub use xx_input_method_v1_text_change_cause_event::XxInputMethodV1TextChangeCauseEvent;

mod xx_input_method_v1_content_type_event;
#[doc(inline)]
pub use xx_input_method_v1_content_type_event::XxInputMethodV1ContentTypeEvent;

mod xx_input_method_v1_set_available_actions_event;
#[doc(inline)]
pub use xx_input_method_v1_set_available_actions_event::XxInputMethodV1SetAvailableActionsEvent;

mod xx_input_method_v1_announce_supported_features_event;
#[doc(inline)]
pub use xx_input_method_v1_announce_supported_features_event::XxInputMethodV1AnnounceSupportedFeaturesEvent;

mod xx_input_method_v1_announce_protocol_compat_event;
#[doc(inline)]
pub use xx_input_method_v1_announce_protocol_compat_event::XxInputMethodV1AnnounceProtocolCompatEvent;

mod xx_input_method_v1_done_event;
#[doc(inline)]
pub use xx_input_method_v1_done_event::XxInputMethodV1DoneEvent;

mod xx_input_method_v1_unavailable_event;
#[doc(inline)]
pub use xx_input_method_v1_unavailable_event::XxInputMethodV1UnavailableEvent;

mod xx_input_popup_surface_v2_error;
#[doc(inline)]
pub use xx_input_popup_surface_v2_error::XxInputPopupSurfaceV2Error;

mod xx_input_popup_surface_v2_ack_configure_request;
#[doc(inline)]
pub use xx_input_popup_surface_v2_ack_configure_request::XxInputPopupSurfaceV2AckConfigureRequest;

mod xx_input_popup_surface_v2_reposition_request;
#[doc(inline)]
pub use xx_input_popup_surface_v2_reposition_request::XxInputPopupSurfaceV2RepositionRequest;

mod xx_input_popup_surface_v2_destroy_request;
#[doc(inline)]
pub use xx_input_popup_surface_v2_destroy_request::XxInputPopupSurfaceV2DestroyRequest;

mod xx_input_popup_surface_v2_start_configure_event;
#[doc(inline)]
pub use xx_input_popup_surface_v2_start_configure_event::XxInputPopupSurfaceV2StartConfigureEvent;

mod xx_input_popup_surface_v2_repositioned_event;
#[doc(inline)]
pub use xx_input_popup_surface_v2_repositioned_event::XxInputPopupSurfaceV2RepositionedEvent;

mod xx_input_popup_positioner_v1_error;
#[doc(inline)]
pub use xx_input_popup_positioner_v1_error::XxInputPopupPositionerV1Error;

mod xx_input_popup_positioner_v1_anchor;
#[doc(inline)]
pub use xx_input_popup_positioner_v1_anchor::XxInputPopupPositionerV1Anchor;

mod xx_input_popup_positioner_v1_gravity;
#[doc(inline)]
pub use xx_input_popup_positioner_v1_gravity::XxInputPopupPositionerV1Gravity;

mod xx_input_popup_positioner_v1_constraint_adjustment;
#[doc(inline)]
pub use xx_input_popup_positioner_v1_constraint_adjustment::XxInputPopupPositionerV1ConstraintAdjustment;

mod xx_input_popup_positioner_v1_destroy_request;
#[doc(inline)]
pub use xx_input_popup_positioner_v1_destroy_request::XxInputPopupPositionerV1DestroyRequest;

mod xx_input_popup_positioner_v1_set_size_request;
#[doc(inline)]
pub use xx_input_popup_positioner_v1_set_size_request::XxInputPopupPositionerV1SetSizeRequest;

mod xx_input_popup_positioner_v1_set_anchor_request;
#[doc(inline)]
pub use xx_input_popup_positioner_v1_set_anchor_request::XxInputPopupPositionerV1SetAnchorRequest;

mod xx_input_popup_positioner_v1_set_gravity_request;
#[doc(inline)]
pub use xx_input_popup_positioner_v1_set_gravity_request::XxInputPopupPositionerV1SetGravityRequest;

mod xx_input_popup_positioner_v1_set_constraint_adjustment_request;
#[doc(inline)]
pub use xx_input_popup_positioner_v1_set_constraint_adjustment_request::XxInputPopupPositionerV1SetConstraintAdjustmentRequest;

mod xx_input_popup_positioner_v1_set_offset_request;
#[doc(inline)]
pub use xx_input_popup_positioner_v1_set_offset_request::XxInputPopupPositionerV1SetOffsetRequest;

mod xx_input_popup_positioner_v1_set_reactive_request;
#[doc(inline)]
pub use xx_input_popup_positioner_v1_set_reactive_request::XxInputPopupPositionerV1SetReactiveRequest;

mod xx_input_method_manager_v2_get_input_method_request;
#[doc(inline)]
pub use xx_input_method_manager_v2_get_input_method_request::XxInputMethodManagerV2GetInputMethodRequest;

mod xx_input_method_manager_v2_get_positioner_request;
#[doc(inline)]
pub use xx_input_method_manager_v2_get_positioner_request::XxInputMethodManagerV2GetPositionerRequest;

mod xx_input_method_manager_v2_destroy_request;
#[doc(inline)]
pub use xx_input_method_manager_v2_destroy_request::XxInputMethodManagerV2DestroyRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
