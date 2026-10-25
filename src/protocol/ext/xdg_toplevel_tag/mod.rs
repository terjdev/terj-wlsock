//! Encodes and decodes the xdg-toplevel-tag extension.
//!
//! Clients attach persistent tags and translated descriptions to xdg_toplevels.
//! The caller stores tags, applies window rules and resolves toplevel lifetimes.
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
//! Defined by upstream [xdg-toplevel-tag-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/xdg-toplevel-tag/xdg-toplevel-tag-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod xdg_toplevel_tag_manager_v1_destroy_request;
#[doc(inline)]
pub use xdg_toplevel_tag_manager_v1_destroy_request::XdgToplevelTagManagerV1DestroyRequest;

mod xdg_toplevel_tag_manager_v1_set_toplevel_tag_request;
#[doc(inline)]
pub use xdg_toplevel_tag_manager_v1_set_toplevel_tag_request::XdgToplevelTagManagerV1SetToplevelTagRequest;

mod xdg_toplevel_tag_manager_v1_set_toplevel_description_request;
#[doc(inline)]
pub use xdg_toplevel_tag_manager_v1_set_toplevel_description_request::XdgToplevelTagManagerV1SetToplevelDescriptionRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
