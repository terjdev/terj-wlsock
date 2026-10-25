//! Encodes and decodes the xdg-dialog extension.
//!
//! Dialog objects annotate xdg_toplevel relationships with modal hints. The caller
//! handles parent relationships, toplevel lifetimes, uniqueness and modal policy.
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
//! Defined by upstream [xdg-dialog-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/xdg-dialog/xdg-dialog-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod xdg_wm_dialog_v1_error;
#[doc(inline)]
pub use xdg_wm_dialog_v1_error::XdgWmDialogV1Error;

mod xdg_wm_dialog_v1_destroy_request;
#[doc(inline)]
pub use xdg_wm_dialog_v1_destroy_request::XdgWmDialogV1DestroyRequest;

mod xdg_wm_dialog_v1_get_xdg_dialog_request;
#[doc(inline)]
pub use xdg_wm_dialog_v1_get_xdg_dialog_request::XdgWmDialogV1GetXdgDialogRequest;

mod xdg_dialog_v1_destroy_request;
#[doc(inline)]
pub use xdg_dialog_v1_destroy_request::XdgDialogV1DestroyRequest;

mod xdg_dialog_v1_set_modal_request;
#[doc(inline)]
pub use xdg_dialog_v1_set_modal_request::XdgDialogV1SetModalRequest;

mod xdg_dialog_v1_unset_modal_request;
#[doc(inline)]
pub use xdg_dialog_v1_unset_modal_request::XdgDialogV1UnsetModalRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
