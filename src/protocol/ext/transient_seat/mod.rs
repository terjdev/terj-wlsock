//! Encodes and decodes the ext-transient-seat extension.
//!
//! A manager creates transient-seat objects; ready announces a wl_registry global
//! name rather than a new wire object. The caller allocates seats, manages input
//! capability changes and removes the seat when the transient object is destroyed.
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
//! Defined by upstream [ext-transient-seat-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/ext-transient-seat/ext-transient-seat-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod ext_transient_seat_manager_v1_create_request;
#[doc(inline)]
pub use ext_transient_seat_manager_v1_create_request::ExtTransientSeatManagerV1CreateRequest;

mod ext_transient_seat_manager_v1_destroy_request;
#[doc(inline)]
pub use ext_transient_seat_manager_v1_destroy_request::ExtTransientSeatManagerV1DestroyRequest;

mod ext_transient_seat_v1_destroy_request;
#[doc(inline)]
pub use ext_transient_seat_v1_destroy_request::ExtTransientSeatV1DestroyRequest;

mod ext_transient_seat_v1_ready_event;
#[doc(inline)]
pub use ext_transient_seat_v1_ready_event::ExtTransientSeatV1ReadyEvent;

mod ext_transient_seat_v1_denied_event;
#[doc(inline)]
pub use ext_transient_seat_v1_denied_event::ExtTransientSeatV1DeniedEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
