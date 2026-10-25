//! Encodes and decodes the output-management protocol family.
//!
//! Managers announce server-created heads and modes and group updates with done.
//! Configurations enable or disable every head, then test or apply the result.
//! Configuration-head objects live until their parent configuration is destroyed;
//! their client IDs remain reserved until delete_id. Manager finished is a
//! destructor; head/mode finished notifications make objects inert. Version 3
//! adds head/mode release requests and version 4 adds adaptive sync. Mode versions
//! are capped at 3, while other interfaces reach 4. The caller enforces configuration
//! serials, completeness, property uniqueness, object lifetimes and backend changes.
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
//! Source: [wlr-output-management-unstable-v1.xml](https://gitlab.freedesktop.org/wlroots/wlr-protocols/-/blob/master/unstable/wlr-output-management-unstable-v1.xml).

mod zwlr_output_manager_v1_create_configuration_request;
#[doc(inline)]
pub use zwlr_output_manager_v1_create_configuration_request::ZwlrOutputManagerV1CreateConfigurationRequest;

mod zwlr_output_manager_v1_stop_request;
#[doc(inline)]
pub use zwlr_output_manager_v1_stop_request::ZwlrOutputManagerV1StopRequest;

mod zwlr_output_manager_v1_head_event;
#[doc(inline)]
pub use zwlr_output_manager_v1_head_event::ZwlrOutputManagerV1HeadEvent;

mod zwlr_output_manager_v1_done_event;
#[doc(inline)]
pub use zwlr_output_manager_v1_done_event::ZwlrOutputManagerV1DoneEvent;

mod zwlr_output_manager_v1_finished_event;
#[doc(inline)]
pub use zwlr_output_manager_v1_finished_event::ZwlrOutputManagerV1FinishedEvent;

mod zwlr_output_head_v1_adaptive_sync_state;
#[doc(inline)]
pub use zwlr_output_head_v1_adaptive_sync_state::ZwlrOutputHeadV1AdaptiveSyncState;

mod zwlr_output_head_v1_release_request;
#[doc(inline)]
pub use zwlr_output_head_v1_release_request::ZwlrOutputHeadV1ReleaseRequest;

mod zwlr_output_head_v1_name_event;
#[doc(inline)]
pub use zwlr_output_head_v1_name_event::ZwlrOutputHeadV1NameEvent;

mod zwlr_output_head_v1_description_event;
#[doc(inline)]
pub use zwlr_output_head_v1_description_event::ZwlrOutputHeadV1DescriptionEvent;

mod zwlr_output_head_v1_physical_size_event;
#[doc(inline)]
pub use zwlr_output_head_v1_physical_size_event::ZwlrOutputHeadV1PhysicalSizeEvent;

mod zwlr_output_head_v1_mode_event;
#[doc(inline)]
pub use zwlr_output_head_v1_mode_event::ZwlrOutputHeadV1ModeEvent;

mod zwlr_output_head_v1_enabled_event;
#[doc(inline)]
pub use zwlr_output_head_v1_enabled_event::ZwlrOutputHeadV1EnabledEvent;

mod zwlr_output_head_v1_current_mode_event;
#[doc(inline)]
pub use zwlr_output_head_v1_current_mode_event::ZwlrOutputHeadV1CurrentModeEvent;

mod zwlr_output_head_v1_position_event;
#[doc(inline)]
pub use zwlr_output_head_v1_position_event::ZwlrOutputHeadV1PositionEvent;

mod zwlr_output_head_v1_transform_event;
#[doc(inline)]
pub use zwlr_output_head_v1_transform_event::ZwlrOutputHeadV1TransformEvent;

mod zwlr_output_head_v1_scale_event;
#[doc(inline)]
pub use zwlr_output_head_v1_scale_event::ZwlrOutputHeadV1ScaleEvent;

mod zwlr_output_head_v1_finished_event;
#[doc(inline)]
pub use zwlr_output_head_v1_finished_event::ZwlrOutputHeadV1FinishedEvent;

mod zwlr_output_head_v1_make_event;
#[doc(inline)]
pub use zwlr_output_head_v1_make_event::ZwlrOutputHeadV1MakeEvent;

mod zwlr_output_head_v1_model_event;
#[doc(inline)]
pub use zwlr_output_head_v1_model_event::ZwlrOutputHeadV1ModelEvent;

mod zwlr_output_head_v1_serial_number_event;
#[doc(inline)]
pub use zwlr_output_head_v1_serial_number_event::ZwlrOutputHeadV1SerialNumberEvent;

mod zwlr_output_head_v1_adaptive_sync_event;
#[doc(inline)]
pub use zwlr_output_head_v1_adaptive_sync_event::ZwlrOutputHeadV1AdaptiveSyncEvent;

mod zwlr_output_mode_v1_release_request;
#[doc(inline)]
pub use zwlr_output_mode_v1_release_request::ZwlrOutputModeV1ReleaseRequest;

mod zwlr_output_mode_v1_size_event;
#[doc(inline)]
pub use zwlr_output_mode_v1_size_event::ZwlrOutputModeV1SizeEvent;

mod zwlr_output_mode_v1_refresh_event;
#[doc(inline)]
pub use zwlr_output_mode_v1_refresh_event::ZwlrOutputModeV1RefreshEvent;

mod zwlr_output_mode_v1_preferred_event;
#[doc(inline)]
pub use zwlr_output_mode_v1_preferred_event::ZwlrOutputModeV1PreferredEvent;

mod zwlr_output_mode_v1_finished_event;
#[doc(inline)]
pub use zwlr_output_mode_v1_finished_event::ZwlrOutputModeV1FinishedEvent;

mod zwlr_output_configuration_v1_error;
#[doc(inline)]
pub use zwlr_output_configuration_v1_error::ZwlrOutputConfigurationV1Error;

mod zwlr_output_configuration_v1_enable_head_request;
#[doc(inline)]
pub use zwlr_output_configuration_v1_enable_head_request::ZwlrOutputConfigurationV1EnableHeadRequest;

mod zwlr_output_configuration_v1_disable_head_request;
#[doc(inline)]
pub use zwlr_output_configuration_v1_disable_head_request::ZwlrOutputConfigurationV1DisableHeadRequest;

mod zwlr_output_configuration_v1_apply_request;
#[doc(inline)]
pub use zwlr_output_configuration_v1_apply_request::ZwlrOutputConfigurationV1ApplyRequest;

mod zwlr_output_configuration_v1_test_request;
#[doc(inline)]
pub use zwlr_output_configuration_v1_test_request::ZwlrOutputConfigurationV1TestRequest;

mod zwlr_output_configuration_v1_destroy_request;
#[doc(inline)]
pub use zwlr_output_configuration_v1_destroy_request::ZwlrOutputConfigurationV1DestroyRequest;

mod zwlr_output_configuration_v1_succeeded_event;
#[doc(inline)]
pub use zwlr_output_configuration_v1_succeeded_event::ZwlrOutputConfigurationV1SucceededEvent;

mod zwlr_output_configuration_v1_failed_event;
#[doc(inline)]
pub use zwlr_output_configuration_v1_failed_event::ZwlrOutputConfigurationV1FailedEvent;

mod zwlr_output_configuration_v1_cancelled_event;
#[doc(inline)]
pub use zwlr_output_configuration_v1_cancelled_event::ZwlrOutputConfigurationV1CancelledEvent;

mod zwlr_output_configuration_head_v1_error;
#[doc(inline)]
pub use zwlr_output_configuration_head_v1_error::ZwlrOutputConfigurationHeadV1Error;

mod zwlr_output_configuration_head_v1_set_mode_request;
#[doc(inline)]
pub use zwlr_output_configuration_head_v1_set_mode_request::ZwlrOutputConfigurationHeadV1SetModeRequest;

mod zwlr_output_configuration_head_v1_set_custom_mode_request;
#[doc(inline)]
pub use zwlr_output_configuration_head_v1_set_custom_mode_request::ZwlrOutputConfigurationHeadV1SetCustomModeRequest;

mod zwlr_output_configuration_head_v1_set_position_request;
#[doc(inline)]
pub use zwlr_output_configuration_head_v1_set_position_request::ZwlrOutputConfigurationHeadV1SetPositionRequest;

mod zwlr_output_configuration_head_v1_set_transform_request;
#[doc(inline)]
pub use zwlr_output_configuration_head_v1_set_transform_request::ZwlrOutputConfigurationHeadV1SetTransformRequest;

mod zwlr_output_configuration_head_v1_set_scale_request;
#[doc(inline)]
pub use zwlr_output_configuration_head_v1_set_scale_request::ZwlrOutputConfigurationHeadV1SetScaleRequest;

mod zwlr_output_configuration_head_v1_set_adaptive_sync_request;
#[doc(inline)]
pub use zwlr_output_configuration_head_v1_set_adaptive_sync_request::ZwlrOutputConfigurationHeadV1SetAdaptiveSyncRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
