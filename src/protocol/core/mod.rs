//! Encodes and decodes the core Wayland protocol on the server side.
//!
//! Requests are read through [`CommandRequest`](crate::protocol::CommandRequest),
//! and events are written through [`CommandEvent`](crate::protocol::CommandEvent).
//! Each message has a dedicated structure with its XML definition.
//! Object identifiers, negotiated versions, and object lifetimes are validated
//! against the connection state. Compositor resources and policy remain with the
//! caller. The complete protocol definition is available in the upstream
//! [Wayland XML](https://gitlab.freedesktop.org/wayland/wayland/-/blob/main/protocol/wayland.xml).
//! The package's LICENSE.md preserves the original copyright and permission
//! notices for the XML definitions and documentation excerpts.
mod wl_display_error;
#[doc(inline)]
pub use wl_display_error::WlDisplayError;
mod wl_shm_pool_error;
#[doc(inline)]
pub use wl_shm_pool_error::WlShmPoolError;
mod wl_shm_error;
#[doc(inline)]
pub use wl_shm_error::WlShmError;
mod wl_shm_format;
#[doc(inline)]
pub use wl_shm_format::WlShmFormat;
mod wl_data_offer_error;
#[doc(inline)]
pub use wl_data_offer_error::WlDataOfferError;
mod wl_data_source_error;
#[doc(inline)]
pub use wl_data_source_error::WlDataSourceError;
mod wl_data_device_error;
#[doc(inline)]
pub use wl_data_device_error::WlDataDeviceError;
mod wl_data_device_manager_dnd_action;
#[doc(inline)]
pub use wl_data_device_manager_dnd_action::WlDataDeviceManagerDndAction;
mod wl_shell_error;
#[doc(inline)]
pub use wl_shell_error::WlShellError;
mod wl_shell_surface_resize;
#[doc(inline)]
pub use wl_shell_surface_resize::WlShellSurfaceResize;
mod wl_shell_surface_transient;
#[doc(inline)]
pub use wl_shell_surface_transient::WlShellSurfaceTransient;
mod wl_shell_surface_fullscreen_method;
#[doc(inline)]
pub use wl_shell_surface_fullscreen_method::WlShellSurfaceFullscreenMethod;
mod wl_surface_error;
#[doc(inline)]
pub use wl_surface_error::WlSurfaceError;
mod wl_seat_capability;
#[doc(inline)]
pub use wl_seat_capability::WlSeatCapability;
mod wl_seat_error;
#[doc(inline)]
pub use wl_seat_error::WlSeatError;
mod wl_pointer_error;
#[doc(inline)]
pub use wl_pointer_error::WlPointerError;
mod wl_pointer_button_state;
#[doc(inline)]
pub use wl_pointer_button_state::WlPointerButtonState;
mod wl_pointer_axis;
#[doc(inline)]
pub use wl_pointer_axis::WlPointerAxis;
mod wl_pointer_axis_source;
#[doc(inline)]
pub use wl_pointer_axis_source::WlPointerAxisSource;
mod wl_pointer_axis_relative_direction;
#[doc(inline)]
pub use wl_pointer_axis_relative_direction::WlPointerAxisRelativeDirection;
mod wl_keyboard_keymap_format;
#[doc(inline)]
pub use wl_keyboard_keymap_format::WlKeyboardKeymapFormat;
mod wl_keyboard_key_state;
#[doc(inline)]
pub use wl_keyboard_key_state::WlKeyboardKeyState;
mod wl_output_subpixel;
#[doc(inline)]
pub use wl_output_subpixel::WlOutputSubpixel;
mod wl_output_transform;
#[doc(inline)]
pub use wl_output_transform::WlOutputTransform;
mod wl_output_mode;
#[doc(inline)]
pub use wl_output_mode::WlOutputMode;
mod wl_subcompositor_error;
#[doc(inline)]
pub use wl_subcompositor_error::WlSubcompositorError;
mod wl_subsurface_error;
#[doc(inline)]
pub use wl_subsurface_error::WlSubsurfaceError;
mod wl_fixes_error;
#[doc(inline)]
pub use wl_fixes_error::WlFixesError;
mod wl_registry_bind_id;
#[doc(inline)]
pub use wl_registry_bind_id::WlRegistryBindId;
mod wl_display_sync_request;
#[doc(inline)]
pub use wl_display_sync_request::WlDisplaySyncRequest;
mod wl_display_get_registry_request;
#[doc(inline)]
pub use wl_display_get_registry_request::WlDisplayGetRegistryRequest;
mod wl_display_error_event;
#[doc(inline)]
pub use wl_display_error_event::WlDisplayErrorEvent;
mod wl_display_delete_id_event;
#[doc(inline)]
pub use wl_display_delete_id_event::WlDisplayDeleteIdEvent;
mod wl_registry_bind_request;
#[doc(inline)]
pub use wl_registry_bind_request::WlRegistryBindRequest;
mod wl_registry_global_event;
#[doc(inline)]
pub use wl_registry_global_event::WlRegistryGlobalEvent;
mod wl_registry_global_remove_event;
#[doc(inline)]
pub use wl_registry_global_remove_event::WlRegistryGlobalRemoveEvent;
mod wl_callback_done_event;
#[doc(inline)]
pub use wl_callback_done_event::WlCallbackDoneEvent;
mod wl_compositor_create_surface_request;
#[doc(inline)]
pub use wl_compositor_create_surface_request::WlCompositorCreateSurfaceRequest;
mod wl_compositor_create_region_request;
#[doc(inline)]
pub use wl_compositor_create_region_request::WlCompositorCreateRegionRequest;
mod wl_compositor_release_request;
#[doc(inline)]
pub use wl_compositor_release_request::WlCompositorReleaseRequest;
mod wl_shm_pool_create_buffer_request;
#[doc(inline)]
pub use wl_shm_pool_create_buffer_request::WlShmPoolCreateBufferRequest;
mod wl_shm_pool_destroy_request;
#[doc(inline)]
pub use wl_shm_pool_destroy_request::WlShmPoolDestroyRequest;
mod wl_shm_pool_resize_request;
#[doc(inline)]
pub use wl_shm_pool_resize_request::WlShmPoolResizeRequest;
mod wl_shm_create_pool_request;
#[doc(inline)]
pub use wl_shm_create_pool_request::WlShmCreatePoolRequest;
mod wl_shm_release_request;
#[doc(inline)]
pub use wl_shm_release_request::WlShmReleaseRequest;
mod wl_shm_format_event;
#[doc(inline)]
pub use wl_shm_format_event::WlShmFormatEvent;
mod wl_buffer_destroy_request;
#[doc(inline)]
pub use wl_buffer_destroy_request::WlBufferDestroyRequest;
mod wl_buffer_release_event;
#[doc(inline)]
pub use wl_buffer_release_event::WlBufferReleaseEvent;
mod wl_data_offer_accept_request;
#[doc(inline)]
pub use wl_data_offer_accept_request::WlDataOfferAcceptRequest;
mod wl_data_offer_receive_request;
#[doc(inline)]
pub use wl_data_offer_receive_request::WlDataOfferReceiveRequest;
mod wl_data_offer_destroy_request;
#[doc(inline)]
pub use wl_data_offer_destroy_request::WlDataOfferDestroyRequest;
mod wl_data_offer_finish_request;
#[doc(inline)]
pub use wl_data_offer_finish_request::WlDataOfferFinishRequest;
mod wl_data_offer_set_actions_request;
#[doc(inline)]
pub use wl_data_offer_set_actions_request::WlDataOfferSetActionsRequest;
mod wl_data_offer_offer_event;
#[doc(inline)]
pub use wl_data_offer_offer_event::WlDataOfferOfferEvent;
mod wl_data_offer_source_actions_event;
#[doc(inline)]
pub use wl_data_offer_source_actions_event::WlDataOfferSourceActionsEvent;
mod wl_data_offer_action_event;
#[doc(inline)]
pub use wl_data_offer_action_event::WlDataOfferActionEvent;
mod wl_data_source_offer_request;
#[doc(inline)]
pub use wl_data_source_offer_request::WlDataSourceOfferRequest;
mod wl_data_source_destroy_request;
#[doc(inline)]
pub use wl_data_source_destroy_request::WlDataSourceDestroyRequest;
mod wl_data_source_set_actions_request;
#[doc(inline)]
pub use wl_data_source_set_actions_request::WlDataSourceSetActionsRequest;
mod wl_data_source_target_event;
#[doc(inline)]
pub use wl_data_source_target_event::WlDataSourceTargetEvent;
mod wl_data_source_send_event;
#[doc(inline)]
pub use wl_data_source_send_event::WlDataSourceSendEvent;
mod wl_data_source_cancelled_event;
#[doc(inline)]
pub use wl_data_source_cancelled_event::WlDataSourceCancelledEvent;
mod wl_data_source_dnd_drop_performed_event;
#[doc(inline)]
pub use wl_data_source_dnd_drop_performed_event::WlDataSourceDndDropPerformedEvent;
mod wl_data_source_dnd_finished_event;
#[doc(inline)]
pub use wl_data_source_dnd_finished_event::WlDataSourceDndFinishedEvent;
mod wl_data_source_action_event;
#[doc(inline)]
pub use wl_data_source_action_event::WlDataSourceActionEvent;
mod wl_data_device_start_drag_request;
#[doc(inline)]
pub use wl_data_device_start_drag_request::WlDataDeviceStartDragRequest;
mod wl_data_device_set_selection_request;
#[doc(inline)]
pub use wl_data_device_set_selection_request::WlDataDeviceSetSelectionRequest;
mod wl_data_device_release_request;
#[doc(inline)]
pub use wl_data_device_release_request::WlDataDeviceReleaseRequest;
mod wl_data_device_data_offer_event;
#[doc(inline)]
pub use wl_data_device_data_offer_event::WlDataDeviceDataOfferEvent;
mod wl_data_device_enter_event;
#[doc(inline)]
pub use wl_data_device_enter_event::WlDataDeviceEnterEvent;
mod wl_data_device_leave_event;
#[doc(inline)]
pub use wl_data_device_leave_event::WlDataDeviceLeaveEvent;
mod wl_data_device_motion_event;
#[doc(inline)]
pub use wl_data_device_motion_event::WlDataDeviceMotionEvent;
mod wl_data_device_drop_event;
#[doc(inline)]
pub use wl_data_device_drop_event::WlDataDeviceDropEvent;
mod wl_data_device_selection_event;
#[doc(inline)]
pub use wl_data_device_selection_event::WlDataDeviceSelectionEvent;
mod wl_data_device_manager_create_data_source_request;
#[doc(inline)]
pub use wl_data_device_manager_create_data_source_request::WlDataDeviceManagerCreateDataSourceRequest;
mod wl_data_device_manager_get_data_device_request;
#[doc(inline)]
pub use wl_data_device_manager_get_data_device_request::WlDataDeviceManagerGetDataDeviceRequest;
mod wl_data_device_manager_release_request;
#[doc(inline)]
pub use wl_data_device_manager_release_request::WlDataDeviceManagerReleaseRequest;
mod wl_shell_get_shell_surface_request;
#[doc(inline)]
pub use wl_shell_get_shell_surface_request::WlShellGetShellSurfaceRequest;
mod wl_shell_surface_pong_request;
#[doc(inline)]
pub use wl_shell_surface_pong_request::WlShellSurfacePongRequest;
mod wl_shell_surface_move_request;
#[doc(inline)]
pub use wl_shell_surface_move_request::WlShellSurfaceMoveRequest;
mod wl_shell_surface_resize_request;
#[doc(inline)]
pub use wl_shell_surface_resize_request::WlShellSurfaceResizeRequest;
mod wl_shell_surface_set_toplevel_request;
#[doc(inline)]
pub use wl_shell_surface_set_toplevel_request::WlShellSurfaceSetToplevelRequest;
mod wl_shell_surface_set_transient_request;
#[doc(inline)]
pub use wl_shell_surface_set_transient_request::WlShellSurfaceSetTransientRequest;
mod wl_shell_surface_set_fullscreen_request;
#[doc(inline)]
pub use wl_shell_surface_set_fullscreen_request::WlShellSurfaceSetFullscreenRequest;
mod wl_shell_surface_set_popup_request;
#[doc(inline)]
pub use wl_shell_surface_set_popup_request::WlShellSurfaceSetPopupRequest;
mod wl_shell_surface_set_maximized_request;
#[doc(inline)]
pub use wl_shell_surface_set_maximized_request::WlShellSurfaceSetMaximizedRequest;
mod wl_shell_surface_set_title_request;
#[doc(inline)]
pub use wl_shell_surface_set_title_request::WlShellSurfaceSetTitleRequest;
mod wl_shell_surface_set_class_request;
#[doc(inline)]
pub use wl_shell_surface_set_class_request::WlShellSurfaceSetClassRequest;
mod wl_shell_surface_ping_event;
#[doc(inline)]
pub use wl_shell_surface_ping_event::WlShellSurfacePingEvent;
mod wl_shell_surface_configure_event;
#[doc(inline)]
pub use wl_shell_surface_configure_event::WlShellSurfaceConfigureEvent;
mod wl_shell_surface_popup_done_event;
#[doc(inline)]
pub use wl_shell_surface_popup_done_event::WlShellSurfacePopupDoneEvent;
mod wl_surface_destroy_request;
#[doc(inline)]
pub use wl_surface_destroy_request::WlSurfaceDestroyRequest;
mod wl_surface_attach_request;
#[doc(inline)]
pub use wl_surface_attach_request::WlSurfaceAttachRequest;
mod wl_surface_damage_request;
#[doc(inline)]
pub use wl_surface_damage_request::WlSurfaceDamageRequest;
mod wl_surface_frame_request;
#[doc(inline)]
pub use wl_surface_frame_request::WlSurfaceFrameRequest;
mod wl_surface_set_opaque_region_request;
#[doc(inline)]
pub use wl_surface_set_opaque_region_request::WlSurfaceSetOpaqueRegionRequest;
mod wl_surface_set_input_region_request;
#[doc(inline)]
pub use wl_surface_set_input_region_request::WlSurfaceSetInputRegionRequest;
mod wl_surface_commit_request;
#[doc(inline)]
pub use wl_surface_commit_request::WlSurfaceCommitRequest;
mod wl_surface_set_buffer_transform_request;
#[doc(inline)]
pub use wl_surface_set_buffer_transform_request::WlSurfaceSetBufferTransformRequest;
mod wl_surface_set_buffer_scale_request;
#[doc(inline)]
pub use wl_surface_set_buffer_scale_request::WlSurfaceSetBufferScaleRequest;
mod wl_surface_damage_buffer_request;
#[doc(inline)]
pub use wl_surface_damage_buffer_request::WlSurfaceDamageBufferRequest;
mod wl_surface_offset_request;
#[doc(inline)]
pub use wl_surface_offset_request::WlSurfaceOffsetRequest;
mod wl_surface_get_release_request;
#[doc(inline)]
pub use wl_surface_get_release_request::WlSurfaceGetReleaseRequest;
mod wl_surface_enter_event;
#[doc(inline)]
pub use wl_surface_enter_event::WlSurfaceEnterEvent;
mod wl_surface_leave_event;
#[doc(inline)]
pub use wl_surface_leave_event::WlSurfaceLeaveEvent;
mod wl_surface_preferred_buffer_scale_event;
#[doc(inline)]
pub use wl_surface_preferred_buffer_scale_event::WlSurfacePreferredBufferScaleEvent;
mod wl_surface_preferred_buffer_transform_event;
#[doc(inline)]
pub use wl_surface_preferred_buffer_transform_event::WlSurfacePreferredBufferTransformEvent;
mod wl_seat_get_pointer_request;
#[doc(inline)]
pub use wl_seat_get_pointer_request::WlSeatGetPointerRequest;
mod wl_seat_get_keyboard_request;
#[doc(inline)]
pub use wl_seat_get_keyboard_request::WlSeatGetKeyboardRequest;
mod wl_seat_get_touch_request;
#[doc(inline)]
pub use wl_seat_get_touch_request::WlSeatGetTouchRequest;
mod wl_seat_release_request;
#[doc(inline)]
pub use wl_seat_release_request::WlSeatReleaseRequest;
mod wl_seat_capabilities_event;
#[doc(inline)]
pub use wl_seat_capabilities_event::WlSeatCapabilitiesEvent;
mod wl_seat_name_event;
#[doc(inline)]
pub use wl_seat_name_event::WlSeatNameEvent;
mod wl_pointer_set_cursor_request;
#[doc(inline)]
pub use wl_pointer_set_cursor_request::WlPointerSetCursorRequest;
mod wl_pointer_release_request;
#[doc(inline)]
pub use wl_pointer_release_request::WlPointerReleaseRequest;
mod wl_pointer_enter_event;
#[doc(inline)]
pub use wl_pointer_enter_event::WlPointerEnterEvent;
mod wl_pointer_leave_event;
#[doc(inline)]
pub use wl_pointer_leave_event::WlPointerLeaveEvent;
mod wl_pointer_motion_event;
#[doc(inline)]
pub use wl_pointer_motion_event::WlPointerMotionEvent;
mod wl_pointer_button_event;
#[doc(inline)]
pub use wl_pointer_button_event::WlPointerButtonEvent;
mod wl_pointer_axis_event;
#[doc(inline)]
pub use wl_pointer_axis_event::WlPointerAxisEvent;
mod wl_pointer_frame_event;
#[doc(inline)]
pub use wl_pointer_frame_event::WlPointerFrameEvent;
mod wl_pointer_axis_source_event;
#[doc(inline)]
pub use wl_pointer_axis_source_event::WlPointerAxisSourceEvent;
mod wl_pointer_axis_stop_event;
#[doc(inline)]
pub use wl_pointer_axis_stop_event::WlPointerAxisStopEvent;
mod wl_pointer_axis_discrete_event;
#[doc(inline)]
pub use wl_pointer_axis_discrete_event::WlPointerAxisDiscreteEvent;
mod wl_pointer_axis_value120_event;
#[doc(inline)]
pub use wl_pointer_axis_value120_event::WlPointerAxisValue120Event;
mod wl_pointer_axis_relative_direction_event;
#[doc(inline)]
pub use wl_pointer_axis_relative_direction_event::WlPointerAxisRelativeDirectionEvent;
mod wl_pointer_warp_event;
#[doc(inline)]
pub use wl_pointer_warp_event::WlPointerWarpEvent;
mod wl_keyboard_release_request;
#[doc(inline)]
pub use wl_keyboard_release_request::WlKeyboardReleaseRequest;
mod wl_keyboard_keymap_event;
#[doc(inline)]
pub use wl_keyboard_keymap_event::WlKeyboardKeymapEvent;
mod wl_keyboard_enter_event;
#[doc(inline)]
pub use wl_keyboard_enter_event::WlKeyboardEnterEvent;
mod wl_keyboard_leave_event;
#[doc(inline)]
pub use wl_keyboard_leave_event::WlKeyboardLeaveEvent;
mod wl_keyboard_key_event;
#[doc(inline)]
pub use wl_keyboard_key_event::WlKeyboardKeyEvent;
mod wl_keyboard_modifiers_event;
#[doc(inline)]
pub use wl_keyboard_modifiers_event::WlKeyboardModifiersEvent;
mod wl_keyboard_repeat_info_event;
#[doc(inline)]
pub use wl_keyboard_repeat_info_event::WlKeyboardRepeatInfoEvent;
mod wl_touch_release_request;
#[doc(inline)]
pub use wl_touch_release_request::WlTouchReleaseRequest;
mod wl_touch_down_event;
#[doc(inline)]
pub use wl_touch_down_event::WlTouchDownEvent;
mod wl_touch_up_event;
#[doc(inline)]
pub use wl_touch_up_event::WlTouchUpEvent;
mod wl_touch_motion_event;
#[doc(inline)]
pub use wl_touch_motion_event::WlTouchMotionEvent;
mod wl_touch_frame_event;
#[doc(inline)]
pub use wl_touch_frame_event::WlTouchFrameEvent;
mod wl_touch_cancel_event;
#[doc(inline)]
pub use wl_touch_cancel_event::WlTouchCancelEvent;
mod wl_touch_shape_event;
#[doc(inline)]
pub use wl_touch_shape_event::WlTouchShapeEvent;
mod wl_touch_orientation_event;
#[doc(inline)]
pub use wl_touch_orientation_event::WlTouchOrientationEvent;
mod wl_output_release_request;
#[doc(inline)]
pub use wl_output_release_request::WlOutputReleaseRequest;
mod wl_output_geometry_event;
#[doc(inline)]
pub use wl_output_geometry_event::WlOutputGeometryEvent;
mod wl_output_mode_event;
#[doc(inline)]
pub use wl_output_mode_event::WlOutputModeEvent;
mod wl_output_done_event;
#[doc(inline)]
pub use wl_output_done_event::WlOutputDoneEvent;
mod wl_output_scale_event;
#[doc(inline)]
pub use wl_output_scale_event::WlOutputScaleEvent;
mod wl_output_name_event;
#[doc(inline)]
pub use wl_output_name_event::WlOutputNameEvent;
mod wl_output_description_event;
#[doc(inline)]
pub use wl_output_description_event::WlOutputDescriptionEvent;
mod wl_region_destroy_request;
#[doc(inline)]
pub use wl_region_destroy_request::WlRegionDestroyRequest;
mod wl_region_add_request;
#[doc(inline)]
pub use wl_region_add_request::WlRegionAddRequest;
mod wl_region_subtract_request;
#[doc(inline)]
pub use wl_region_subtract_request::WlRegionSubtractRequest;
mod wl_subcompositor_destroy_request;
#[doc(inline)]
pub use wl_subcompositor_destroy_request::WlSubcompositorDestroyRequest;
mod wl_subcompositor_get_subsurface_request;
#[doc(inline)]
pub use wl_subcompositor_get_subsurface_request::WlSubcompositorGetSubsurfaceRequest;
mod wl_subsurface_destroy_request;
#[doc(inline)]
pub use wl_subsurface_destroy_request::WlSubsurfaceDestroyRequest;
mod wl_subsurface_set_position_request;
#[doc(inline)]
pub use wl_subsurface_set_position_request::WlSubsurfaceSetPositionRequest;
mod wl_subsurface_place_above_request;
#[doc(inline)]
pub use wl_subsurface_place_above_request::WlSubsurfacePlaceAboveRequest;
mod wl_subsurface_place_below_request;
#[doc(inline)]
pub use wl_subsurface_place_below_request::WlSubsurfacePlaceBelowRequest;
mod wl_subsurface_set_sync_request;
#[doc(inline)]
pub use wl_subsurface_set_sync_request::WlSubsurfaceSetSyncRequest;
mod wl_subsurface_set_desync_request;
#[doc(inline)]
pub use wl_subsurface_set_desync_request::WlSubsurfaceSetDesyncRequest;
mod wl_fixes_destroy_request;
#[doc(inline)]
pub use wl_fixes_destroy_request::WlFixesDestroyRequest;
mod wl_fixes_destroy_registry_request;
#[doc(inline)]
pub use wl_fixes_destroy_registry_request::WlFixesDestroyRegistryRequest;
mod wl_fixes_ack_global_remove_request;
#[doc(inline)]
pub use wl_fixes_ack_global_remove_request::WlFixesAckGlobalRemoveRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
