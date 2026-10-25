//! Encodes and decodes WLR data-control through manager and device version 2.
//!
//! A manager creates per-seat data devices and data sources. Devices announce
//! server-created offers and regular clipboard or primary selections. Sources
//! advertise MIME types; transfers use owned file descriptors sent through the
//! ancillary-data channel, without payload bytes. The source writes the requested
//! representation and closes its descriptor; the receiver reads until EOF.
//!
//! Codecs check framing, receiver metadata, message versions and new object IDs.
//! Child versions are capped at the target interface maximum: sources and offers
//! use version 1, while devices can use version 2. New IDs are registered before
//! writes; close the connection after failed or cancelled writes following
//! registration. Client IDs remain reserved until `wl_display.delete_id`;
//! destructors release server IDs. Nullable sources and offers use `Option<u32>`.
//! Unknown error codes are preserved, and strings are checked for wire encoding.
//!
//! The caller controls privileged access, seat lifetimes, source reuse, MIME offer
//! ordering, selection replacement, primary-selection availability and actual data
//! transfers. Finished devices and cancelled sources remain registered until their
//! destroy requests. This upstream protocol is deprecated in favor of
//! `ext-data-control-v1`; it is provided here for client compatibility.
//!
//! Defined by upstream [wlr-data-control-unstable-v1.xml](https://gitlab.freedesktop.org/wlroots/wlr-protocols/-/blob/master/unstable/wlr-data-control-unstable-v1.xml).
//! Original copyright and permissive license terms are preserved in LICENSE.md.

mod zwlr_data_control_manager_v1_create_data_source_request;
#[doc(inline)]
pub use zwlr_data_control_manager_v1_create_data_source_request::ZwlrDataControlManagerV1CreateDataSourceRequest;

mod zwlr_data_control_manager_v1_get_data_device_request;
#[doc(inline)]
pub use zwlr_data_control_manager_v1_get_data_device_request::ZwlrDataControlManagerV1GetDataDeviceRequest;

mod zwlr_data_control_manager_v1_destroy_request;
#[doc(inline)]
pub use zwlr_data_control_manager_v1_destroy_request::ZwlrDataControlManagerV1DestroyRequest;

mod zwlr_data_control_device_v1_error;
#[doc(inline)]
pub use zwlr_data_control_device_v1_error::ZwlrDataControlDeviceV1Error;

mod zwlr_data_control_device_v1_set_selection_request;
#[doc(inline)]
pub use zwlr_data_control_device_v1_set_selection_request::ZwlrDataControlDeviceV1SetSelectionRequest;

mod zwlr_data_control_device_v1_destroy_request;
#[doc(inline)]
pub use zwlr_data_control_device_v1_destroy_request::ZwlrDataControlDeviceV1DestroyRequest;

mod zwlr_data_control_device_v1_set_primary_selection_request;
#[doc(inline)]
pub use zwlr_data_control_device_v1_set_primary_selection_request::ZwlrDataControlDeviceV1SetPrimarySelectionRequest;

mod zwlr_data_control_device_v1_data_offer_event;
#[doc(inline)]
pub use zwlr_data_control_device_v1_data_offer_event::ZwlrDataControlDeviceV1DataOfferEvent;

mod zwlr_data_control_device_v1_selection_event;
#[doc(inline)]
pub use zwlr_data_control_device_v1_selection_event::ZwlrDataControlDeviceV1SelectionEvent;

mod zwlr_data_control_device_v1_finished_event;
#[doc(inline)]
pub use zwlr_data_control_device_v1_finished_event::ZwlrDataControlDeviceV1FinishedEvent;

mod zwlr_data_control_device_v1_primary_selection_event;
#[doc(inline)]
pub use zwlr_data_control_device_v1_primary_selection_event::ZwlrDataControlDeviceV1PrimarySelectionEvent;

mod zwlr_data_control_source_v1_error;
#[doc(inline)]
pub use zwlr_data_control_source_v1_error::ZwlrDataControlSourceV1Error;

mod zwlr_data_control_source_v1_offer_request;
#[doc(inline)]
pub use zwlr_data_control_source_v1_offer_request::ZwlrDataControlSourceV1OfferRequest;

mod zwlr_data_control_source_v1_destroy_request;
#[doc(inline)]
pub use zwlr_data_control_source_v1_destroy_request::ZwlrDataControlSourceV1DestroyRequest;

mod zwlr_data_control_source_v1_send_event;
#[doc(inline)]
pub use zwlr_data_control_source_v1_send_event::ZwlrDataControlSourceV1SendEvent;

mod zwlr_data_control_source_v1_cancelled_event;
#[doc(inline)]
pub use zwlr_data_control_source_v1_cancelled_event::ZwlrDataControlSourceV1CancelledEvent;

mod zwlr_data_control_offer_v1_receive_request;
#[doc(inline)]
pub use zwlr_data_control_offer_v1_receive_request::ZwlrDataControlOfferV1ReceiveRequest;

mod zwlr_data_control_offer_v1_destroy_request;
#[doc(inline)]
pub use zwlr_data_control_offer_v1_destroy_request::ZwlrDataControlOfferV1DestroyRequest;

mod zwlr_data_control_offer_v1_offer_event;
#[doc(inline)]
pub use zwlr_data_control_offer_v1_offer_event::ZwlrDataControlOfferV1OfferEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
