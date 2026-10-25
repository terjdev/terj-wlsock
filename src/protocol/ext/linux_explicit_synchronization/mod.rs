//! Encodes and decodes the linux-explicit-synchronization protocol family.
//!
//! Surface synchronization accepts acquire sync-file FDs and creates buffer-release
//! objects. Release events are destructors and may carry a release fence FD. The
//! caller validates fences, commit state and buffer support and performs synchronization.
//!
//! Codecs validate receiver protocol, opcode, minimum version and complete framing.
//! New child IDs inherit the receiver version capped at the target maximum; core
//! keyboard objects use version 1. Registration precedes writes; failed or cancelled
//! writes after registration require closing the connection. Client IDs remain
//! reserved until `wl_display.delete_id`; destructors release server IDs.
//! Fixed values use `f64` with signed 24.8 encoding. Strings and fixed values are
//! validated before writes. Nullable values use `Option`; arrays retain raw bytes.
//! Unknown enum values and flag bits are preserved. Protocol semantics belong to
//! the caller. Original copyright and permission terms are in LICENSE.md.
//! Source: [linux-explicit-synchronization-unstable-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/unstable/linux-explicit-synchronization/linux-explicit-synchronization-unstable-v1.xml).

mod zwp_linux_explicit_synchronization_v1_error;
#[doc(inline)]
pub use zwp_linux_explicit_synchronization_v1_error::ZwpLinuxExplicitSynchronizationV1Error;

mod zwp_linux_explicit_synchronization_v1_destroy_request;
#[doc(inline)]
pub use zwp_linux_explicit_synchronization_v1_destroy_request::ZwpLinuxExplicitSynchronizationV1DestroyRequest;

mod zwp_linux_explicit_synchronization_v1_get_synchronization_request;
#[doc(inline)]
pub use zwp_linux_explicit_synchronization_v1_get_synchronization_request::ZwpLinuxExplicitSynchronizationV1GetSynchronizationRequest;

mod zwp_linux_surface_synchronization_v1_error;
#[doc(inline)]
pub use zwp_linux_surface_synchronization_v1_error::ZwpLinuxSurfaceSynchronizationV1Error;

mod zwp_linux_surface_synchronization_v1_destroy_request;
#[doc(inline)]
pub use zwp_linux_surface_synchronization_v1_destroy_request::ZwpLinuxSurfaceSynchronizationV1DestroyRequest;

mod zwp_linux_surface_synchronization_v1_set_acquire_fence_request;
#[doc(inline)]
pub use zwp_linux_surface_synchronization_v1_set_acquire_fence_request::ZwpLinuxSurfaceSynchronizationV1SetAcquireFenceRequest;

mod zwp_linux_surface_synchronization_v1_get_release_request;
#[doc(inline)]
pub use zwp_linux_surface_synchronization_v1_get_release_request::ZwpLinuxSurfaceSynchronizationV1GetReleaseRequest;

mod zwp_linux_buffer_release_v1_fenced_release_event;
#[doc(inline)]
pub use zwp_linux_buffer_release_v1_fenced_release_event::ZwpLinuxBufferReleaseV1FencedReleaseEvent;

mod zwp_linux_buffer_release_v1_immediate_release_event;
#[doc(inline)]
pub use zwp_linux_buffer_release_v1_immediate_release_event::ZwpLinuxBufferReleaseV1ImmediateReleaseEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
