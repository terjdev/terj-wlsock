//! Encodes and decodes the tearing-control extension.
//!
//! Per-surface presentation hints select vsync or asynchronous presentation.
//! The caller enforces one control object per surface, handles pending commit
//! state and decides whether asynchronous presentation is possible.
//!
//! Codecs validate framing, receiver metadata, message versions and new object IDs.
//! Child versions inherit the parent version, capped at the target maximum; core
//! buffers use version 1. New IDs are registered before writes. A failed or
//! cancelled write after registration requires closing the connection. Client IDs
//! remain reserved until `wl_display.delete_id`; destructors release server IDs.
//! Strings and fixed values are checked for wire encoding. Nullable arguments use
//! `Option`; arrays retain raw bytes. Unknown enum values and flag bits are preserved.
//! The caller enforces protocol semantics, object relationships and compositor policy.
//! This extension defines no events.
//!
//! Defined by upstream [tearing-control-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/tearing-control/tearing-control-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod wp_tearing_control_manager_v1_error;
#[doc(inline)]
pub use wp_tearing_control_manager_v1_error::WpTearingControlManagerV1Error;

mod wp_tearing_control_manager_v1_destroy_request;
#[doc(inline)]
pub use wp_tearing_control_manager_v1_destroy_request::WpTearingControlManagerV1DestroyRequest;

mod wp_tearing_control_manager_v1_get_tearing_control_request;
#[doc(inline)]
pub use wp_tearing_control_manager_v1_get_tearing_control_request::WpTearingControlManagerV1GetTearingControlRequest;

mod wp_tearing_control_v1_presentation_hint;
#[doc(inline)]
pub use wp_tearing_control_v1_presentation_hint::WpTearingControlV1PresentationHint;

mod wp_tearing_control_v1_set_presentation_hint_request;
#[doc(inline)]
pub use wp_tearing_control_v1_set_presentation_hint_request::WpTearingControlV1SetPresentationHintRequest;

mod wp_tearing_control_v1_destroy_request;
#[doc(inline)]
pub use wp_tearing_control_v1_destroy_request::WpTearingControlV1DestroyRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
