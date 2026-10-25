//! Encodes and decodes the ext-session-lock extension.
//!
//! Lock objects request exclusive session locking and create per-output surfaces.
//! locked confirms the lock; finished reports refusal or completion without
//! destroying the object. unlock_and_destroy is a destructor. The caller enforces
//! lock state, authentication policy, output coverage, configure acknowledgements
//! and buffer presentation before confirming that the session is locked.
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
//! Defined by upstream [ext-session-lock-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/ext-session-lock/ext-session-lock-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod ext_session_lock_manager_v1_destroy_request;
#[doc(inline)]
pub use ext_session_lock_manager_v1_destroy_request::ExtSessionLockManagerV1DestroyRequest;

mod ext_session_lock_manager_v1_lock_request;
#[doc(inline)]
pub use ext_session_lock_manager_v1_lock_request::ExtSessionLockManagerV1LockRequest;

mod ext_session_lock_v1_error;
#[doc(inline)]
pub use ext_session_lock_v1_error::ExtSessionLockV1Error;

mod ext_session_lock_v1_destroy_request;
#[doc(inline)]
pub use ext_session_lock_v1_destroy_request::ExtSessionLockV1DestroyRequest;

mod ext_session_lock_v1_get_lock_surface_request;
#[doc(inline)]
pub use ext_session_lock_v1_get_lock_surface_request::ExtSessionLockV1GetLockSurfaceRequest;

mod ext_session_lock_v1_unlock_and_destroy_request;
#[doc(inline)]
pub use ext_session_lock_v1_unlock_and_destroy_request::ExtSessionLockV1UnlockAndDestroyRequest;

mod ext_session_lock_v1_locked_event;
#[doc(inline)]
pub use ext_session_lock_v1_locked_event::ExtSessionLockV1LockedEvent;

mod ext_session_lock_v1_finished_event;
#[doc(inline)]
pub use ext_session_lock_v1_finished_event::ExtSessionLockV1FinishedEvent;

mod ext_session_lock_surface_v1_error;
#[doc(inline)]
pub use ext_session_lock_surface_v1_error::ExtSessionLockSurfaceV1Error;

mod ext_session_lock_surface_v1_destroy_request;
#[doc(inline)]
pub use ext_session_lock_surface_v1_destroy_request::ExtSessionLockSurfaceV1DestroyRequest;

mod ext_session_lock_surface_v1_ack_configure_request;
#[doc(inline)]
pub use ext_session_lock_surface_v1_ack_configure_request::ExtSessionLockSurfaceV1AckConfigureRequest;

mod ext_session_lock_surface_v1_configure_event;
#[doc(inline)]
pub use ext_session_lock_surface_v1_configure_event::ExtSessionLockSurfaceV1ConfigureEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
