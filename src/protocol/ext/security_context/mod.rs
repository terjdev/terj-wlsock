//! Encodes and decodes the security-context extension.
//!
//! Clients provide listening and close-notification FDs, attach sandbox metadata
//! and commit it. The caller accepts connections, monitors FDs, restricts nested
//! contexts, validates metadata and applies access policy to the new connections.
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
//! Defined by upstream [security-context-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/security-context/security-context-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod wp_security_context_manager_v1_error;
#[doc(inline)]
pub use wp_security_context_manager_v1_error::WpSecurityContextManagerV1Error;

mod wp_security_context_manager_v1_destroy_request;
#[doc(inline)]
pub use wp_security_context_manager_v1_destroy_request::WpSecurityContextManagerV1DestroyRequest;

mod wp_security_context_manager_v1_create_listener_request;
#[doc(inline)]
pub use wp_security_context_manager_v1_create_listener_request::WpSecurityContextManagerV1CreateListenerRequest;

mod wp_security_context_v1_error;
#[doc(inline)]
pub use wp_security_context_v1_error::WpSecurityContextV1Error;

mod wp_security_context_v1_destroy_request;
#[doc(inline)]
pub use wp_security_context_v1_destroy_request::WpSecurityContextV1DestroyRequest;

mod wp_security_context_v1_set_sandbox_engine_request;
#[doc(inline)]
pub use wp_security_context_v1_set_sandbox_engine_request::WpSecurityContextV1SetSandboxEngineRequest;

mod wp_security_context_v1_set_app_id_request;
#[doc(inline)]
pub use wp_security_context_v1_set_app_id_request::WpSecurityContextV1SetAppIdRequest;

mod wp_security_context_v1_set_instance_id_request;
#[doc(inline)]
pub use wp_security_context_v1_set_instance_id_request::WpSecurityContextV1SetInstanceIdRequest;

mod wp_security_context_v1_commit_request;
#[doc(inline)]
pub use wp_security_context_v1_commit_request::WpSecurityContextV1CommitRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
