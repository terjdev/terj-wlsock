//! Encodes and decodes the commit-timing extension through interface version 1.
//!
//! A manager creates one timer per wl_surface. Each timestamp constrains the next
//! content update to be presented as closely as possible to, but not before, the
//! requested time in the compositor's presentation-clock domain. Seconds are split
//! into high and low u32 words; nanoseconds must be below one billion. Destroying
//! a timer leaves existing constraints intact. The caller validates timestamps,
//! enforces one pending timestamp and one timer per surface, tracks surface
//! lifetimes and preserves commit order while scheduling presentation.
//!
//! Codecs validate framing, receiver metadata, message versions and new object IDs.
//! Child versions inherit the receiver version, capped at the target maximum.
//! New IDs are registered before writes; close the connection after failed or
//! cancelled writes following registration. Client IDs remain reserved until
//! wl_display.delete_id. Protocol semantics and compositor policy belong to the
//! caller. This extension defines no events.
//!
//! Defined by upstream [commit-timing-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/commit-timing/commit-timing-v1.xml).
//! Original copyright and MIT permission terms are preserved in LICENSE.md.

mod wp_commit_timing_manager_v1_error;
#[doc(inline)]
pub use wp_commit_timing_manager_v1_error::WpCommitTimingManagerV1Error;

mod wp_commit_timing_manager_v1_destroy_request;
#[doc(inline)]
pub use wp_commit_timing_manager_v1_destroy_request::WpCommitTimingManagerV1DestroyRequest;

mod wp_commit_timing_manager_v1_get_timer_request;
#[doc(inline)]
pub use wp_commit_timing_manager_v1_get_timer_request::WpCommitTimingManagerV1GetTimerRequest;

mod wp_commit_timer_v1_error;
#[doc(inline)]
pub use wp_commit_timer_v1_error::WpCommitTimerV1Error;

mod wp_commit_timer_v1_set_timestamp_request;
#[doc(inline)]
pub use wp_commit_timer_v1_set_timestamp_request::WpCommitTimerV1SetTimestampRequest;

mod wp_commit_timer_v1_destroy_request;
#[doc(inline)]
pub use wp_commit_timer_v1_destroy_request::WpCommitTimerV1DestroyRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
