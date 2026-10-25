//! Encodes and decodes the ext-foreign-toplevel-list extension.
//!
//! The list introduces server-created handles for mapped toplevels. Handle updates
//! are grouped by done; closed and finished do not destroy their objects. The
//! caller controls window visibility, identifiers, stop ordering and handle lifetimes.
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
//! Defined by upstream [ext-foreign-toplevel-list-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/ext-foreign-toplevel-list/ext-foreign-toplevel-list-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod ext_foreign_toplevel_list_v1_stop_request;
#[doc(inline)]
pub use ext_foreign_toplevel_list_v1_stop_request::ExtForeignToplevelListV1StopRequest;

mod ext_foreign_toplevel_list_v1_destroy_request;
#[doc(inline)]
pub use ext_foreign_toplevel_list_v1_destroy_request::ExtForeignToplevelListV1DestroyRequest;

mod ext_foreign_toplevel_list_v1_toplevel_event;
#[doc(inline)]
pub use ext_foreign_toplevel_list_v1_toplevel_event::ExtForeignToplevelListV1ToplevelEvent;

mod ext_foreign_toplevel_list_v1_finished_event;
#[doc(inline)]
pub use ext_foreign_toplevel_list_v1_finished_event::ExtForeignToplevelListV1FinishedEvent;

mod ext_foreign_toplevel_handle_v1_destroy_request;
#[doc(inline)]
pub use ext_foreign_toplevel_handle_v1_destroy_request::ExtForeignToplevelHandleV1DestroyRequest;

mod ext_foreign_toplevel_handle_v1_closed_event;
#[doc(inline)]
pub use ext_foreign_toplevel_handle_v1_closed_event::ExtForeignToplevelHandleV1ClosedEvent;

mod ext_foreign_toplevel_handle_v1_done_event;
#[doc(inline)]
pub use ext_foreign_toplevel_handle_v1_done_event::ExtForeignToplevelHandleV1DoneEvent;

mod ext_foreign_toplevel_handle_v1_title_event;
#[doc(inline)]
pub use ext_foreign_toplevel_handle_v1_title_event::ExtForeignToplevelHandleV1TitleEvent;

mod ext_foreign_toplevel_handle_v1_app_id_event;
#[doc(inline)]
pub use ext_foreign_toplevel_handle_v1_app_id_event::ExtForeignToplevelHandleV1AppIdEvent;

mod ext_foreign_toplevel_handle_v1_identifier_event;
#[doc(inline)]
pub use ext_foreign_toplevel_handle_v1_identifier_event::ExtForeignToplevelHandleV1IdentifierEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
