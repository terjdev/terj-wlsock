//! Encodes and decodes the color-management extension.
//!
//! Color descriptions support ICC and parametric metadata, HDR properties, output
//! descriptions and preferred surface feedback. Creator requests are one-shot;
//! information objects destroy themselves on done. Version 2 adds 64-bit identities
//! and version 3 adds BT.2100 creation. Reference objects are frozen at version 1.
//! The caller validates color parameters, supported features, ICC files, readiness,
//! identities, and performs the actual color conversion and rendering.
//!
//! Codecs validate framing, receiver metadata, message versions and new object IDs.
//! Child versions inherit the parent version, capped at the target maximum; core
//! buffers use version 1. New IDs are registered before writes. A failed or
//! cancelled write after registration requires closing the connection. Client IDs
//! remain reserved until `wl_display.delete_id`; destructors release server IDs.
//! Strings and fixed values are checked for wire encoding. Nullable arguments use
//! `Option`; arrays retain raw bytes. Unknown enum values and flag bits are preserved.
//! The caller enforces protocol semantics, object relationships and compositor policy.
//!
//! Defined by upstream [color-management-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/color-management/color-management-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod wp_color_manager_v1_error;
#[doc(inline)]
pub use wp_color_manager_v1_error::WpColorManagerV1Error;

mod wp_color_manager_v1_render_intent;
#[doc(inline)]
pub use wp_color_manager_v1_render_intent::WpColorManagerV1RenderIntent;

mod wp_color_manager_v1_feature;
#[doc(inline)]
pub use wp_color_manager_v1_feature::WpColorManagerV1Feature;

mod wp_color_manager_v1_primaries;
#[doc(inline)]
pub use wp_color_manager_v1_primaries::WpColorManagerV1Primaries;

mod wp_color_manager_v1_transfer_function;
#[doc(inline)]
pub use wp_color_manager_v1_transfer_function::WpColorManagerV1TransferFunction;

mod wp_color_manager_v1_destroy_request;
#[doc(inline)]
pub use wp_color_manager_v1_destroy_request::WpColorManagerV1DestroyRequest;

mod wp_color_manager_v1_get_output_request;
#[doc(inline)]
pub use wp_color_manager_v1_get_output_request::WpColorManagerV1GetOutputRequest;

mod wp_color_manager_v1_get_surface_request;
#[doc(inline)]
pub use wp_color_manager_v1_get_surface_request::WpColorManagerV1GetSurfaceRequest;

mod wp_color_manager_v1_get_surface_feedback_request;
#[doc(inline)]
pub use wp_color_manager_v1_get_surface_feedback_request::WpColorManagerV1GetSurfaceFeedbackRequest;

mod wp_color_manager_v1_create_icc_creator_request;
#[doc(inline)]
pub use wp_color_manager_v1_create_icc_creator_request::WpColorManagerV1CreateIccCreatorRequest;

mod wp_color_manager_v1_create_parametric_creator_request;
#[doc(inline)]
pub use wp_color_manager_v1_create_parametric_creator_request::WpColorManagerV1CreateParametricCreatorRequest;

mod wp_color_manager_v1_create_windows_scrgb_request;
#[doc(inline)]
pub use wp_color_manager_v1_create_windows_scrgb_request::WpColorManagerV1CreateWindowsScrgbRequest;

mod wp_color_manager_v1_get_image_description_request;
#[doc(inline)]
pub use wp_color_manager_v1_get_image_description_request::WpColorManagerV1GetImageDescriptionRequest;

mod wp_color_manager_v1_create_windows_bt2100_request;
#[doc(inline)]
pub use wp_color_manager_v1_create_windows_bt2100_request::WpColorManagerV1CreateWindowsBt2100Request;

mod wp_color_manager_v1_supported_intent_event;
#[doc(inline)]
pub use wp_color_manager_v1_supported_intent_event::WpColorManagerV1SupportedIntentEvent;

mod wp_color_manager_v1_supported_feature_event;
#[doc(inline)]
pub use wp_color_manager_v1_supported_feature_event::WpColorManagerV1SupportedFeatureEvent;

mod wp_color_manager_v1_supported_tf_named_event;
#[doc(inline)]
pub use wp_color_manager_v1_supported_tf_named_event::WpColorManagerV1SupportedTfNamedEvent;

mod wp_color_manager_v1_supported_primaries_named_event;
#[doc(inline)]
pub use wp_color_manager_v1_supported_primaries_named_event::WpColorManagerV1SupportedPrimariesNamedEvent;

mod wp_color_manager_v1_done_event;
#[doc(inline)]
pub use wp_color_manager_v1_done_event::WpColorManagerV1DoneEvent;

mod wp_color_management_output_v1_destroy_request;
#[doc(inline)]
pub use wp_color_management_output_v1_destroy_request::WpColorManagementOutputV1DestroyRequest;

mod wp_color_management_output_v1_get_image_description_request;
#[doc(inline)]
pub use wp_color_management_output_v1_get_image_description_request::WpColorManagementOutputV1GetImageDescriptionRequest;

mod wp_color_management_output_v1_image_description_changed_event;
#[doc(inline)]
pub use wp_color_management_output_v1_image_description_changed_event::WpColorManagementOutputV1ImageDescriptionChangedEvent;

mod wp_color_management_surface_v1_error;
#[doc(inline)]
pub use wp_color_management_surface_v1_error::WpColorManagementSurfaceV1Error;

mod wp_color_management_surface_v1_destroy_request;
#[doc(inline)]
pub use wp_color_management_surface_v1_destroy_request::WpColorManagementSurfaceV1DestroyRequest;

mod wp_color_management_surface_v1_set_image_description_request;
#[doc(inline)]
pub use wp_color_management_surface_v1_set_image_description_request::WpColorManagementSurfaceV1SetImageDescriptionRequest;

mod wp_color_management_surface_v1_unset_image_description_request;
#[doc(inline)]
pub use wp_color_management_surface_v1_unset_image_description_request::WpColorManagementSurfaceV1UnsetImageDescriptionRequest;

mod wp_color_management_surface_feedback_v1_error;
#[doc(inline)]
pub use wp_color_management_surface_feedback_v1_error::WpColorManagementSurfaceFeedbackV1Error;

mod wp_color_management_surface_feedback_v1_destroy_request;
#[doc(inline)]
pub use wp_color_management_surface_feedback_v1_destroy_request::WpColorManagementSurfaceFeedbackV1DestroyRequest;

mod wp_color_management_surface_feedback_v1_get_preferred_request;
#[doc(inline)]
pub use wp_color_management_surface_feedback_v1_get_preferred_request::WpColorManagementSurfaceFeedbackV1GetPreferredRequest;

mod wp_color_management_surface_feedback_v1_get_preferred_parametric_request;
#[doc(inline)]
pub use wp_color_management_surface_feedback_v1_get_preferred_parametric_request::WpColorManagementSurfaceFeedbackV1GetPreferredParametricRequest;

mod wp_color_management_surface_feedback_v1_preferred_changed_event;
#[doc(inline)]
pub use wp_color_management_surface_feedback_v1_preferred_changed_event::WpColorManagementSurfaceFeedbackV1PreferredChangedEvent;

mod wp_color_management_surface_feedback_v1_preferred_changed2_event;
#[doc(inline)]
pub use wp_color_management_surface_feedback_v1_preferred_changed2_event::WpColorManagementSurfaceFeedbackV1PreferredChanged2Event;

mod wp_image_description_creator_icc_v1_error;
#[doc(inline)]
pub use wp_image_description_creator_icc_v1_error::WpImageDescriptionCreatorIccV1Error;

mod wp_image_description_creator_icc_v1_create_request;
#[doc(inline)]
pub use wp_image_description_creator_icc_v1_create_request::WpImageDescriptionCreatorIccV1CreateRequest;

mod wp_image_description_creator_icc_v1_set_icc_file_request;
#[doc(inline)]
pub use wp_image_description_creator_icc_v1_set_icc_file_request::WpImageDescriptionCreatorIccV1SetIccFileRequest;

mod wp_image_description_creator_params_v1_error;
#[doc(inline)]
pub use wp_image_description_creator_params_v1_error::WpImageDescriptionCreatorParamsV1Error;

mod wp_image_description_creator_params_v1_create_request;
#[doc(inline)]
pub use wp_image_description_creator_params_v1_create_request::WpImageDescriptionCreatorParamsV1CreateRequest;

mod wp_image_description_creator_params_v1_set_tf_named_request;
#[doc(inline)]
pub use wp_image_description_creator_params_v1_set_tf_named_request::WpImageDescriptionCreatorParamsV1SetTfNamedRequest;

mod wp_image_description_creator_params_v1_set_tf_power_request;
#[doc(inline)]
pub use wp_image_description_creator_params_v1_set_tf_power_request::WpImageDescriptionCreatorParamsV1SetTfPowerRequest;

mod wp_image_description_creator_params_v1_set_primaries_named_request;
#[doc(inline)]
pub use wp_image_description_creator_params_v1_set_primaries_named_request::WpImageDescriptionCreatorParamsV1SetPrimariesNamedRequest;

mod wp_image_description_creator_params_v1_set_primaries_request;
#[doc(inline)]
pub use wp_image_description_creator_params_v1_set_primaries_request::WpImageDescriptionCreatorParamsV1SetPrimariesRequest;

mod wp_image_description_creator_params_v1_set_luminances_request;
#[doc(inline)]
pub use wp_image_description_creator_params_v1_set_luminances_request::WpImageDescriptionCreatorParamsV1SetLuminancesRequest;

mod wp_image_description_creator_params_v1_set_mastering_display_primaries_request;
#[doc(inline)]
pub use wp_image_description_creator_params_v1_set_mastering_display_primaries_request::WpImageDescriptionCreatorParamsV1SetMasteringDisplayPrimariesRequest;

mod wp_image_description_creator_params_v1_set_mastering_luminance_request;
#[doc(inline)]
pub use wp_image_description_creator_params_v1_set_mastering_luminance_request::WpImageDescriptionCreatorParamsV1SetMasteringLuminanceRequest;

mod wp_image_description_creator_params_v1_set_max_cll_request;
#[doc(inline)]
pub use wp_image_description_creator_params_v1_set_max_cll_request::WpImageDescriptionCreatorParamsV1SetMaxCllRequest;

mod wp_image_description_creator_params_v1_set_max_fall_request;
#[doc(inline)]
pub use wp_image_description_creator_params_v1_set_max_fall_request::WpImageDescriptionCreatorParamsV1SetMaxFallRequest;

mod wp_image_description_v1_error;
#[doc(inline)]
pub use wp_image_description_v1_error::WpImageDescriptionV1Error;

mod wp_image_description_v1_cause;
#[doc(inline)]
pub use wp_image_description_v1_cause::WpImageDescriptionV1Cause;

mod wp_image_description_v1_destroy_request;
#[doc(inline)]
pub use wp_image_description_v1_destroy_request::WpImageDescriptionV1DestroyRequest;

mod wp_image_description_v1_get_information_request;
#[doc(inline)]
pub use wp_image_description_v1_get_information_request::WpImageDescriptionV1GetInformationRequest;

mod wp_image_description_v1_failed_event;
#[doc(inline)]
pub use wp_image_description_v1_failed_event::WpImageDescriptionV1FailedEvent;

mod wp_image_description_v1_ready_event;
#[doc(inline)]
pub use wp_image_description_v1_ready_event::WpImageDescriptionV1ReadyEvent;

mod wp_image_description_v1_ready2_event;
#[doc(inline)]
pub use wp_image_description_v1_ready2_event::WpImageDescriptionV1Ready2Event;

mod wp_image_description_info_v1_done_event;
#[doc(inline)]
pub use wp_image_description_info_v1_done_event::WpImageDescriptionInfoV1DoneEvent;

mod wp_image_description_info_v1_icc_file_event;
#[doc(inline)]
pub use wp_image_description_info_v1_icc_file_event::WpImageDescriptionInfoV1IccFileEvent;

mod wp_image_description_info_v1_primaries_event;
#[doc(inline)]
pub use wp_image_description_info_v1_primaries_event::WpImageDescriptionInfoV1PrimariesEvent;

mod wp_image_description_info_v1_primaries_named_event;
#[doc(inline)]
pub use wp_image_description_info_v1_primaries_named_event::WpImageDescriptionInfoV1PrimariesNamedEvent;

mod wp_image_description_info_v1_tf_power_event;
#[doc(inline)]
pub use wp_image_description_info_v1_tf_power_event::WpImageDescriptionInfoV1TfPowerEvent;

mod wp_image_description_info_v1_tf_named_event;
#[doc(inline)]
pub use wp_image_description_info_v1_tf_named_event::WpImageDescriptionInfoV1TfNamedEvent;

mod wp_image_description_info_v1_luminances_event;
#[doc(inline)]
pub use wp_image_description_info_v1_luminances_event::WpImageDescriptionInfoV1LuminancesEvent;

mod wp_image_description_info_v1_target_primaries_event;
#[doc(inline)]
pub use wp_image_description_info_v1_target_primaries_event::WpImageDescriptionInfoV1TargetPrimariesEvent;

mod wp_image_description_info_v1_target_luminance_event;
#[doc(inline)]
pub use wp_image_description_info_v1_target_luminance_event::WpImageDescriptionInfoV1TargetLuminanceEvent;

mod wp_image_description_info_v1_target_max_cll_event;
#[doc(inline)]
pub use wp_image_description_info_v1_target_max_cll_event::WpImageDescriptionInfoV1TargetMaxCllEvent;

mod wp_image_description_info_v1_target_max_fall_event;
#[doc(inline)]
pub use wp_image_description_info_v1_target_max_fall_event::WpImageDescriptionInfoV1TargetMaxFallEvent;

mod wp_image_description_reference_v1_destroy_request;
#[doc(inline)]
pub use wp_image_description_reference_v1_destroy_request::WpImageDescriptionReferenceV1DestroyRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
