//! Encodes and decodes the ext-background-effect extension.
//!
//! The manager advertises capabilities and creates one effects object per surface.
//! Blur regions are nullable wl_region IDs with copy semantics. Changes and effect
//! removal on destruction apply on the next wl_surface.commit. Losing a capability
//! disables that effect regardless of the configured region. The caller enforces
//! unique surface associations, lifetimes, region copying/clipping, pending-state
//! application and the blur algorithm. Unknown capability bits are preserved.
//!
//! Codecs validate framing, receiver metadata, message versions and new object IDs.
//! Child versions inherit the receiver version, capped at the target maximum.
//! New IDs are registered before writes; close the connection after failed or
//! cancelled writes following registration. Client IDs remain reserved until
//! wl_display.delete_id; destructors release server IDs. Strings are checked for
//! wire encoding, and unknown enum values are preserved. Protocol semantics and
//! compositor policy belong to the caller.
//!
//! Defined by upstream [ext-background-effect-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/ext-background-effect/ext-background-effect-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod ext_background_effect_manager_v1_error;
#[doc(inline)]
pub use ext_background_effect_manager_v1_error::ExtBackgroundEffectManagerV1Error;

mod ext_background_effect_manager_v1_capability;
#[doc(inline)]
pub use ext_background_effect_manager_v1_capability::ExtBackgroundEffectManagerV1Capability;

mod ext_background_effect_manager_v1_destroy_request;
#[doc(inline)]
pub use ext_background_effect_manager_v1_destroy_request::ExtBackgroundEffectManagerV1DestroyRequest;

mod ext_background_effect_manager_v1_get_background_effect_request;
#[doc(inline)]
pub use ext_background_effect_manager_v1_get_background_effect_request::ExtBackgroundEffectManagerV1GetBackgroundEffectRequest;

mod ext_background_effect_manager_v1_capabilities_event;
#[doc(inline)]
pub use ext_background_effect_manager_v1_capabilities_event::ExtBackgroundEffectManagerV1CapabilitiesEvent;

mod ext_background_effect_surface_v1_error;
#[doc(inline)]
pub use ext_background_effect_surface_v1_error::ExtBackgroundEffectSurfaceV1Error;

mod ext_background_effect_surface_v1_destroy_request;
#[doc(inline)]
pub use ext_background_effect_surface_v1_destroy_request::ExtBackgroundEffectSurfaceV1DestroyRequest;

mod ext_background_effect_surface_v1_set_blur_region_request;
#[doc(inline)]
pub use ext_background_effect_surface_v1_set_blur_region_request::ExtBackgroundEffectSurfaceV1SetBlurRegionRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
