//! Encodes and decodes the xdg-toplevel-drag extension.
//!
//! Drag objects associate a wl_data_source with an xdg_toplevel and offsets. The
//! caller integrates this with core drag-and-drop, enforces lifetime and attachment
//! rules and moves the toplevel during the drag operation.
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
//! Defined by upstream [xdg-toplevel-drag-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/xdg-toplevel-drag/xdg-toplevel-drag-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod xdg_toplevel_drag_manager_v1_error;
#[doc(inline)]
pub use xdg_toplevel_drag_manager_v1_error::XdgToplevelDragManagerV1Error;

mod xdg_toplevel_drag_manager_v1_destroy_request;
#[doc(inline)]
pub use xdg_toplevel_drag_manager_v1_destroy_request::XdgToplevelDragManagerV1DestroyRequest;

mod xdg_toplevel_drag_manager_v1_get_xdg_toplevel_drag_request;
#[doc(inline)]
pub use xdg_toplevel_drag_manager_v1_get_xdg_toplevel_drag_request::XdgToplevelDragManagerV1GetXdgToplevelDragRequest;

mod xdg_toplevel_drag_v1_error;
#[doc(inline)]
pub use xdg_toplevel_drag_v1_error::XdgToplevelDragV1Error;

mod xdg_toplevel_drag_v1_destroy_request;
#[doc(inline)]
pub use xdg_toplevel_drag_v1_destroy_request::XdgToplevelDragV1DestroyRequest;

mod xdg_toplevel_drag_v1_attach_request;
#[doc(inline)]
pub use xdg_toplevel_drag_v1_attach_request::XdgToplevelDragV1AttachRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
