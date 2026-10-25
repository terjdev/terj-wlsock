//! Encodes and decodes the primary-selection protocol family.
//!
//! Sources advertise MIME types and offers transfer data using owned FDs. Nullable
//! selections represent clearing the selection. The caller handles focus, source
//! ownership, serials, offer ordering, selection replacement and data transfer.
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
//! Source: [primary-selection-unstable-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/unstable/primary-selection/primary-selection-unstable-v1.xml).

mod zwp_primary_selection_device_manager_v1_create_source_request;
#[doc(inline)]
pub use zwp_primary_selection_device_manager_v1_create_source_request::ZwpPrimarySelectionDeviceManagerV1CreateSourceRequest;

mod zwp_primary_selection_device_manager_v1_get_device_request;
#[doc(inline)]
pub use zwp_primary_selection_device_manager_v1_get_device_request::ZwpPrimarySelectionDeviceManagerV1GetDeviceRequest;

mod zwp_primary_selection_device_manager_v1_destroy_request;
#[doc(inline)]
pub use zwp_primary_selection_device_manager_v1_destroy_request::ZwpPrimarySelectionDeviceManagerV1DestroyRequest;

mod zwp_primary_selection_device_v1_set_selection_request;
#[doc(inline)]
pub use zwp_primary_selection_device_v1_set_selection_request::ZwpPrimarySelectionDeviceV1SetSelectionRequest;

mod zwp_primary_selection_device_v1_destroy_request;
#[doc(inline)]
pub use zwp_primary_selection_device_v1_destroy_request::ZwpPrimarySelectionDeviceV1DestroyRequest;

mod zwp_primary_selection_device_v1_data_offer_event;
#[doc(inline)]
pub use zwp_primary_selection_device_v1_data_offer_event::ZwpPrimarySelectionDeviceV1DataOfferEvent;

mod zwp_primary_selection_device_v1_selection_event;
#[doc(inline)]
pub use zwp_primary_selection_device_v1_selection_event::ZwpPrimarySelectionDeviceV1SelectionEvent;

mod zwp_primary_selection_offer_v1_receive_request;
#[doc(inline)]
pub use zwp_primary_selection_offer_v1_receive_request::ZwpPrimarySelectionOfferV1ReceiveRequest;

mod zwp_primary_selection_offer_v1_destroy_request;
#[doc(inline)]
pub use zwp_primary_selection_offer_v1_destroy_request::ZwpPrimarySelectionOfferV1DestroyRequest;

mod zwp_primary_selection_offer_v1_offer_event;
#[doc(inline)]
pub use zwp_primary_selection_offer_v1_offer_event::ZwpPrimarySelectionOfferV1OfferEvent;

mod zwp_primary_selection_source_v1_offer_request;
#[doc(inline)]
pub use zwp_primary_selection_source_v1_offer_request::ZwpPrimarySelectionSourceV1OfferRequest;

mod zwp_primary_selection_source_v1_destroy_request;
#[doc(inline)]
pub use zwp_primary_selection_source_v1_destroy_request::ZwpPrimarySelectionSourceV1DestroyRequest;

mod zwp_primary_selection_source_v1_send_event;
#[doc(inline)]
pub use zwp_primary_selection_source_v1_send_event::ZwpPrimarySelectionSourceV1SendEvent;

mod zwp_primary_selection_source_v1_cancelled_event;
#[doc(inline)]
pub use zwp_primary_selection_source_v1_cancelled_event::ZwpPrimarySelectionSourceV1CancelledEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
