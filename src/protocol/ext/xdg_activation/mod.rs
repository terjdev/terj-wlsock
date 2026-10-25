//! Encodes and decodes the xdg-activation extension.
//!
//! Activation-token objects collect a seat serial, app ID and requesting surface
//! before commit. done returns a token string; it does not destroy the object.
//! The caller issues and validates tokens and enforces focus-stealing policy.
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
//! Defined by upstream [xdg-activation-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/xdg-activation/xdg-activation-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod xdg_activation_v1_destroy_request;
#[doc(inline)]
pub use xdg_activation_v1_destroy_request::XdgActivationV1DestroyRequest;

mod xdg_activation_v1_get_activation_token_request;
#[doc(inline)]
pub use xdg_activation_v1_get_activation_token_request::XdgActivationV1GetActivationTokenRequest;

mod xdg_activation_v1_activate_request;
#[doc(inline)]
pub use xdg_activation_v1_activate_request::XdgActivationV1ActivateRequest;

mod xdg_activation_token_v1_error;
#[doc(inline)]
pub use xdg_activation_token_v1_error::XdgActivationTokenV1Error;

mod xdg_activation_token_v1_set_serial_request;
#[doc(inline)]
pub use xdg_activation_token_v1_set_serial_request::XdgActivationTokenV1SetSerialRequest;

mod xdg_activation_token_v1_set_app_id_request;
#[doc(inline)]
pub use xdg_activation_token_v1_set_app_id_request::XdgActivationTokenV1SetAppIdRequest;

mod xdg_activation_token_v1_set_surface_request;
#[doc(inline)]
pub use xdg_activation_token_v1_set_surface_request::XdgActivationTokenV1SetSurfaceRequest;

mod xdg_activation_token_v1_commit_request;
#[doc(inline)]
pub use xdg_activation_token_v1_commit_request::XdgActivationTokenV1CommitRequest;

mod xdg_activation_token_v1_destroy_request;
#[doc(inline)]
pub use xdg_activation_token_v1_destroy_request::XdgActivationTokenV1DestroyRequest;

mod xdg_activation_token_v1_done_event;
#[doc(inline)]
pub use xdg_activation_token_v1_done_event::XdgActivationTokenV1DoneEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
