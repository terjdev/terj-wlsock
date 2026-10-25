//! Encodes and decodes the alpha-modifier extension through interface version 1.
//!
//! A manager creates one alpha modifier per wl_surface. Multipliers are raw
//! `u32` values: zero is fully transparent and `u32::MAX` is the default full
//! multiplier. State is double-buffered until wl_surface.commit, including the
//! reset caused by destroying the modifier. Multiplication occurs in the
//! compositor's blending space after per-pixel alpha processing, even for buffers
//! without an alpha channel. The caller enforces unique surface associations,
//! surface lifetimes and application of pending state.
//!
//! Codecs validate framing, receiver metadata, message versions and new object IDs.
//! Child versions inherit the receiver version, capped at the target maximum.
//! New IDs are registered before writes; close the connection after failed or
//! cancelled writes following registration. Client IDs remain reserved until
//! wl_display.delete_id. Protocol semantics and compositor policy belong to the
//! caller. This extension defines no events.
//!
//! Defined by upstream [alpha-modifier-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/alpha-modifier/alpha-modifier-v1.xml).
//! Original copyright and MIT permission terms are preserved in LICENSE.md.

mod wp_alpha_modifier_v1_error;
#[doc(inline)]
pub use wp_alpha_modifier_v1_error::WpAlphaModifierV1Error;

mod wp_alpha_modifier_v1_destroy_request;
#[doc(inline)]
pub use wp_alpha_modifier_v1_destroy_request::WpAlphaModifierV1DestroyRequest;

mod wp_alpha_modifier_v1_get_surface_request;
#[doc(inline)]
pub use wp_alpha_modifier_v1_get_surface_request::WpAlphaModifierV1GetSurfaceRequest;

mod wp_alpha_modifier_surface_v1_error;
#[doc(inline)]
pub use wp_alpha_modifier_surface_v1_error::WpAlphaModifierSurfaceV1Error;

mod wp_alpha_modifier_surface_v1_destroy_request;
#[doc(inline)]
pub use wp_alpha_modifier_surface_v1_destroy_request::WpAlphaModifierSurfaceV1DestroyRequest;

mod wp_alpha_modifier_surface_v1_set_multiplier_request;
#[doc(inline)]
pub use wp_alpha_modifier_surface_v1_set_multiplier_request::WpAlphaModifierSurfaceV1SetMultiplierRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
