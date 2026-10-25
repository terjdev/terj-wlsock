//! Encodes and decodes the fifo extension.
//!
//! Per-surface FIFO barriers and waits add display-refresh readiness constraints.
//! The caller enforces unique surface associations and applies barriers and waits
//! at commit time while integrating them into the presentation scheduler.
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
//! Defined by upstream [fifo-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/fifo/fifo-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod wp_fifo_manager_v1_error;
#[doc(inline)]
pub use wp_fifo_manager_v1_error::WpFifoManagerV1Error;

mod wp_fifo_manager_v1_destroy_request;
#[doc(inline)]
pub use wp_fifo_manager_v1_destroy_request::WpFifoManagerV1DestroyRequest;

mod wp_fifo_manager_v1_get_fifo_request;
#[doc(inline)]
pub use wp_fifo_manager_v1_get_fifo_request::WpFifoManagerV1GetFifoRequest;

mod wp_fifo_v1_error;
#[doc(inline)]
pub use wp_fifo_v1_error::WpFifoV1Error;

mod wp_fifo_v1_set_barrier_request;
#[doc(inline)]
pub use wp_fifo_v1_set_barrier_request::WpFifoV1SetBarrierRequest;

mod wp_fifo_v1_wait_barrier_request;
#[doc(inline)]
pub use wp_fifo_v1_wait_barrier_request::WpFifoV1WaitBarrierRequest;

mod wp_fifo_v1_destroy_request;
#[doc(inline)]
pub use wp_fifo_v1_destroy_request::WpFifoV1DestroyRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
