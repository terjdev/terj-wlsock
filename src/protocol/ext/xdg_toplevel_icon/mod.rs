//! Encodes and decodes the xdg-toplevel-icon extension.
//!
//! The manager advertises icon sizes and accepts icons for xdg_toplevel objects.
//! Icons use names and/or wl_buffer content at specified scales. Nullable icon
//! arguments reset the icon. The caller validates buffers, freezes applied icons
//! when required and selects how to load and display them.
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
//! Defined by upstream [xdg-toplevel-icon-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/xdg-toplevel-icon/xdg-toplevel-icon-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod xdg_toplevel_icon_manager_v1_destroy_request;
#[doc(inline)]
pub use xdg_toplevel_icon_manager_v1_destroy_request::XdgToplevelIconManagerV1DestroyRequest;

mod xdg_toplevel_icon_manager_v1_create_icon_request;
#[doc(inline)]
pub use xdg_toplevel_icon_manager_v1_create_icon_request::XdgToplevelIconManagerV1CreateIconRequest;

mod xdg_toplevel_icon_manager_v1_set_icon_request;
#[doc(inline)]
pub use xdg_toplevel_icon_manager_v1_set_icon_request::XdgToplevelIconManagerV1SetIconRequest;

mod xdg_toplevel_icon_manager_v1_icon_size_event;
#[doc(inline)]
pub use xdg_toplevel_icon_manager_v1_icon_size_event::XdgToplevelIconManagerV1IconSizeEvent;

mod xdg_toplevel_icon_manager_v1_done_event;
#[doc(inline)]
pub use xdg_toplevel_icon_manager_v1_done_event::XdgToplevelIconManagerV1DoneEvent;

mod xdg_toplevel_icon_v1_error;
#[doc(inline)]
pub use xdg_toplevel_icon_v1_error::XdgToplevelIconV1Error;

mod xdg_toplevel_icon_v1_destroy_request;
#[doc(inline)]
pub use xdg_toplevel_icon_v1_destroy_request::XdgToplevelIconV1DestroyRequest;

mod xdg_toplevel_icon_v1_set_name_request;
#[doc(inline)]
pub use xdg_toplevel_icon_v1_set_name_request::XdgToplevelIconV1SetNameRequest;

mod xdg_toplevel_icon_v1_add_buffer_request;
#[doc(inline)]
pub use xdg_toplevel_icon_v1_add_buffer_request::XdgToplevelIconV1AddBufferRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
