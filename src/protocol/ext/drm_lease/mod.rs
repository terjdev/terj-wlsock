//! Encodes and decodes the drm-lease extension.
//!
//! A device advertises a non-master DRM FD and server-created connectors. Clients
//! collect connectors in a lease request and submit it to create a lease, destroying
//! the request. Granted leases receive a lease FD; finished reports rejection or
//! revocation. Device release is a handshake: release is not a destructor, but the
//! released event is. Withdrawn connectors and finished leases remain registered
//! until their destroy requests. The caller manages DRM resources, authorization,
//! connector ownership, duplicate/empty requests, hotplug, release ordering and
//! lease revocation. This codec does not perform DRM ioctls.
//!
//! Codecs validate framing, receiver metadata, message versions and new object IDs.
//! Child versions inherit the receiver version, capped at the target maximum.
//! New IDs are registered before writes; close the connection after failed or
//! cancelled writes following registration. Client IDs remain reserved until
//! wl_display.delete_id; destructors release server IDs. Strings are checked for
//! wire encoding, and unknown enum values are preserved. Protocol semantics and
//! compositor policy belong to the caller.
//!
//! Defined by upstream [drm-lease-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/drm-lease/drm-lease-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod wp_drm_lease_device_v1_create_lease_request_request;
#[doc(inline)]
pub use wp_drm_lease_device_v1_create_lease_request_request::WpDrmLeaseDeviceV1CreateLeaseRequestRequest;

mod wp_drm_lease_device_v1_release_request;
#[doc(inline)]
pub use wp_drm_lease_device_v1_release_request::WpDrmLeaseDeviceV1ReleaseRequest;

mod wp_drm_lease_device_v1_drm_fd_event;
#[doc(inline)]
pub use wp_drm_lease_device_v1_drm_fd_event::WpDrmLeaseDeviceV1DrmFdEvent;

mod wp_drm_lease_device_v1_connector_event;
#[doc(inline)]
pub use wp_drm_lease_device_v1_connector_event::WpDrmLeaseDeviceV1ConnectorEvent;

mod wp_drm_lease_device_v1_done_event;
#[doc(inline)]
pub use wp_drm_lease_device_v1_done_event::WpDrmLeaseDeviceV1DoneEvent;

mod wp_drm_lease_device_v1_released_event;
#[doc(inline)]
pub use wp_drm_lease_device_v1_released_event::WpDrmLeaseDeviceV1ReleasedEvent;

mod wp_drm_lease_connector_v1_destroy_request;
#[doc(inline)]
pub use wp_drm_lease_connector_v1_destroy_request::WpDrmLeaseConnectorV1DestroyRequest;

mod wp_drm_lease_connector_v1_name_event;
#[doc(inline)]
pub use wp_drm_lease_connector_v1_name_event::WpDrmLeaseConnectorV1NameEvent;

mod wp_drm_lease_connector_v1_description_event;
#[doc(inline)]
pub use wp_drm_lease_connector_v1_description_event::WpDrmLeaseConnectorV1DescriptionEvent;

mod wp_drm_lease_connector_v1_connector_id_event;
#[doc(inline)]
pub use wp_drm_lease_connector_v1_connector_id_event::WpDrmLeaseConnectorV1ConnectorIdEvent;

mod wp_drm_lease_connector_v1_done_event;
#[doc(inline)]
pub use wp_drm_lease_connector_v1_done_event::WpDrmLeaseConnectorV1DoneEvent;

mod wp_drm_lease_connector_v1_withdrawn_event;
#[doc(inline)]
pub use wp_drm_lease_connector_v1_withdrawn_event::WpDrmLeaseConnectorV1WithdrawnEvent;

mod wp_drm_lease_request_v1_error;
#[doc(inline)]
pub use wp_drm_lease_request_v1_error::WpDrmLeaseRequestV1Error;

mod wp_drm_lease_request_v1_request_connector_request;
#[doc(inline)]
pub use wp_drm_lease_request_v1_request_connector_request::WpDrmLeaseRequestV1RequestConnectorRequest;

mod wp_drm_lease_request_v1_submit_request;
#[doc(inline)]
pub use wp_drm_lease_request_v1_submit_request::WpDrmLeaseRequestV1SubmitRequest;

mod wp_drm_lease_v1_destroy_request;
#[doc(inline)]
pub use wp_drm_lease_v1_destroy_request::WpDrmLeaseV1DestroyRequest;

mod wp_drm_lease_v1_lease_fd_event;
#[doc(inline)]
pub use wp_drm_lease_v1_lease_fd_event::WpDrmLeaseV1LeaseFdEvent;

mod wp_drm_lease_v1_finished_event;
#[doc(inline)]
pub use wp_drm_lease_v1_finished_event::WpDrmLeaseV1FinishedEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
