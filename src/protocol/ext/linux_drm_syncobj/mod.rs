//! Encodes and decodes the linux-drm-syncobj extension.
//!
//! A manager imports DRM syncobj timeline FDs and creates surface synchronization
//! objects. Acquire and release points use split high/low u32 values. The caller
//! validates DRM objects, manages timeline synchronization, enforces commit-time
//! point and buffer constraints, and signals release points when buffers are free.
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
//! Defined by upstream [linux-drm-syncobj-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/linux-drm-syncobj/linux-drm-syncobj-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod wp_linux_drm_syncobj_manager_v1_error;
#[doc(inline)]
pub use wp_linux_drm_syncobj_manager_v1_error::WpLinuxDrmSyncobjManagerV1Error;

mod wp_linux_drm_syncobj_manager_v1_destroy_request;
#[doc(inline)]
pub use wp_linux_drm_syncobj_manager_v1_destroy_request::WpLinuxDrmSyncobjManagerV1DestroyRequest;

mod wp_linux_drm_syncobj_manager_v1_get_surface_request;
#[doc(inline)]
pub use wp_linux_drm_syncobj_manager_v1_get_surface_request::WpLinuxDrmSyncobjManagerV1GetSurfaceRequest;

mod wp_linux_drm_syncobj_manager_v1_import_timeline_request;
#[doc(inline)]
pub use wp_linux_drm_syncobj_manager_v1_import_timeline_request::WpLinuxDrmSyncobjManagerV1ImportTimelineRequest;

mod wp_linux_drm_syncobj_timeline_v1_destroy_request;
#[doc(inline)]
pub use wp_linux_drm_syncobj_timeline_v1_destroy_request::WpLinuxDrmSyncobjTimelineV1DestroyRequest;

mod wp_linux_drm_syncobj_surface_v1_error;
#[doc(inline)]
pub use wp_linux_drm_syncobj_surface_v1_error::WpLinuxDrmSyncobjSurfaceV1Error;

mod wp_linux_drm_syncobj_surface_v1_destroy_request;
#[doc(inline)]
pub use wp_linux_drm_syncobj_surface_v1_destroy_request::WpLinuxDrmSyncobjSurfaceV1DestroyRequest;

mod wp_linux_drm_syncobj_surface_v1_set_acquire_point_request;
#[doc(inline)]
pub use wp_linux_drm_syncobj_surface_v1_set_acquire_point_request::WpLinuxDrmSyncobjSurfaceV1SetAcquirePointRequest;

mod wp_linux_drm_syncobj_surface_v1_set_release_point_request;
#[doc(inline)]
pub use wp_linux_drm_syncobj_surface_v1_set_release_point_request::WpLinuxDrmSyncobjSurfaceV1SetReleasePointRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
