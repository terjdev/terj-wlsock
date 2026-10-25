//! Encodes and decodes the ext-data-control extension.
//!
//! A manager creates per-seat data devices and sources. Devices announce server
//! offers for clipboard and primary selections; sources advertise MIME types.
//! receive requests and send events transfer owned FDs through ancillary data,
//! without payload bytes. The source writes and closes its FD; the receiver reads
//! until EOF. Nullable sources and offers use `Option<u32>`. The caller enforces
//! privileged access, seat lifetimes, single-use sources, offer ordering, selection
//! replacement and actual transfers. Primary selection is available at version 1.
//! Finished devices and cancelled sources remain registered until destruction.
//!
//! Codecs validate framing, receiver metadata, message versions and new object IDs.
//! Child versions inherit the receiver version, capped at the target maximum.
//! New IDs are registered before writes; close the connection after failed or
//! cancelled writes following registration. Client IDs remain reserved until
//! wl_display.delete_id; destructors release server IDs. Strings are checked for
//! wire encoding, and unknown enum values are preserved. Protocol semantics and
//! compositor policy belong to the caller.
//!
//! Defined by upstream [ext-data-control-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/ext-data-control/ext-data-control-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod ext_data_control_manager_v1_create_data_source_request;
#[doc(inline)]
pub use ext_data_control_manager_v1_create_data_source_request::ExtDataControlManagerV1CreateDataSourceRequest;

mod ext_data_control_manager_v1_get_data_device_request;
#[doc(inline)]
pub use ext_data_control_manager_v1_get_data_device_request::ExtDataControlManagerV1GetDataDeviceRequest;

mod ext_data_control_manager_v1_destroy_request;
#[doc(inline)]
pub use ext_data_control_manager_v1_destroy_request::ExtDataControlManagerV1DestroyRequest;

mod ext_data_control_device_v1_error;
#[doc(inline)]
pub use ext_data_control_device_v1_error::ExtDataControlDeviceV1Error;

mod ext_data_control_device_v1_set_selection_request;
#[doc(inline)]
pub use ext_data_control_device_v1_set_selection_request::ExtDataControlDeviceV1SetSelectionRequest;

mod ext_data_control_device_v1_destroy_request;
#[doc(inline)]
pub use ext_data_control_device_v1_destroy_request::ExtDataControlDeviceV1DestroyRequest;

mod ext_data_control_device_v1_set_primary_selection_request;
#[doc(inline)]
pub use ext_data_control_device_v1_set_primary_selection_request::ExtDataControlDeviceV1SetPrimarySelectionRequest;

mod ext_data_control_device_v1_data_offer_event;
#[doc(inline)]
pub use ext_data_control_device_v1_data_offer_event::ExtDataControlDeviceV1DataOfferEvent;

mod ext_data_control_device_v1_selection_event;
#[doc(inline)]
pub use ext_data_control_device_v1_selection_event::ExtDataControlDeviceV1SelectionEvent;

mod ext_data_control_device_v1_finished_event;
#[doc(inline)]
pub use ext_data_control_device_v1_finished_event::ExtDataControlDeviceV1FinishedEvent;

mod ext_data_control_device_v1_primary_selection_event;
#[doc(inline)]
pub use ext_data_control_device_v1_primary_selection_event::ExtDataControlDeviceV1PrimarySelectionEvent;

mod ext_data_control_source_v1_error;
#[doc(inline)]
pub use ext_data_control_source_v1_error::ExtDataControlSourceV1Error;

mod ext_data_control_source_v1_offer_request;
#[doc(inline)]
pub use ext_data_control_source_v1_offer_request::ExtDataControlSourceV1OfferRequest;

mod ext_data_control_source_v1_destroy_request;
#[doc(inline)]
pub use ext_data_control_source_v1_destroy_request::ExtDataControlSourceV1DestroyRequest;

mod ext_data_control_source_v1_send_event;
#[doc(inline)]
pub use ext_data_control_source_v1_send_event::ExtDataControlSourceV1SendEvent;

mod ext_data_control_source_v1_cancelled_event;
#[doc(inline)]
pub use ext_data_control_source_v1_cancelled_event::ExtDataControlSourceV1CancelledEvent;

mod ext_data_control_offer_v1_receive_request;
#[doc(inline)]
pub use ext_data_control_offer_v1_receive_request::ExtDataControlOfferV1ReceiveRequest;

mod ext_data_control_offer_v1_destroy_request;
#[doc(inline)]
pub use ext_data_control_offer_v1_destroy_request::ExtDataControlOfferV1DestroyRequest;

mod ext_data_control_offer_v1_offer_event;
#[doc(inline)]
pub use ext_data_control_offer_v1_offer_event::ExtDataControlOfferV1OfferEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
