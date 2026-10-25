//! Encodes and decodes the xdg-session-management extension.
//!
//! Application sessions store toplevel state across restarts. Nullable session IDs
//! distinguish creation from restoration. Toplevel-session objects support naming
//! and restoration; remove destroys a session and its persisted state. The caller
//! manages storage, ownership, identifiers, replacement and restoration policy.
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
//! Defined by upstream [xdg-session-management-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/xdg-session-management/xdg-session-management-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod xdg_session_manager_v1_error;
#[doc(inline)]
pub use xdg_session_manager_v1_error::XdgSessionManagerV1Error;

mod xdg_session_manager_v1_reason;
#[doc(inline)]
pub use xdg_session_manager_v1_reason::XdgSessionManagerV1Reason;

mod xdg_session_manager_v1_destroy_request;
#[doc(inline)]
pub use xdg_session_manager_v1_destroy_request::XdgSessionManagerV1DestroyRequest;

mod xdg_session_manager_v1_get_session_request;
#[doc(inline)]
pub use xdg_session_manager_v1_get_session_request::XdgSessionManagerV1GetSessionRequest;

mod xdg_session_v1_error;
#[doc(inline)]
pub use xdg_session_v1_error::XdgSessionV1Error;

mod xdg_session_v1_destroy_request;
#[doc(inline)]
pub use xdg_session_v1_destroy_request::XdgSessionV1DestroyRequest;

mod xdg_session_v1_remove_request;
#[doc(inline)]
pub use xdg_session_v1_remove_request::XdgSessionV1RemoveRequest;

mod xdg_session_v1_add_toplevel_request;
#[doc(inline)]
pub use xdg_session_v1_add_toplevel_request::XdgSessionV1AddToplevelRequest;

mod xdg_session_v1_restore_toplevel_request;
#[doc(inline)]
pub use xdg_session_v1_restore_toplevel_request::XdgSessionV1RestoreToplevelRequest;

mod xdg_session_v1_remove_toplevel_request;
#[doc(inline)]
pub use xdg_session_v1_remove_toplevel_request::XdgSessionV1RemoveToplevelRequest;

mod xdg_session_v1_created_event;
#[doc(inline)]
pub use xdg_session_v1_created_event::XdgSessionV1CreatedEvent;

mod xdg_session_v1_restored_event;
#[doc(inline)]
pub use xdg_session_v1_restored_event::XdgSessionV1RestoredEvent;

mod xdg_session_v1_replaced_event;
#[doc(inline)]
pub use xdg_session_v1_replaced_event::XdgSessionV1ReplacedEvent;

mod xdg_toplevel_session_v1_destroy_request;
#[doc(inline)]
pub use xdg_toplevel_session_v1_destroy_request::XdgToplevelSessionV1DestroyRequest;

mod xdg_toplevel_session_v1_rename_request;
#[doc(inline)]
pub use xdg_toplevel_session_v1_rename_request::XdgToplevelSessionV1RenameRequest;

mod xdg_toplevel_session_v1_restored_event;
#[doc(inline)]
pub use xdg_toplevel_session_v1_restored_event::XdgToplevelSessionV1RestoredEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
