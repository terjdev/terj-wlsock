//! Encodes and decodes WLR layer-shell through interface version 5.
//!
//! `zwlr_layer_shell_v1` creates layer surfaces for panels, backgrounds,
//! notifications and other desktop components. A layer surface chooses a layer,
//! anchors, size, margins, exclusive zone and keyboard interaction policy.
//! State changes are double-buffered until `wl_surface.commit`. Initial mapping
//! requires a bufferless commit, a configure event and its acknowledgement before
//! attaching a buffer. A null buffer unmaps the surface and resets its state.
//! Popups are existing `xdg_popup` objects created with a null parent; `get_popup`
//! assigns their layer-surface parent before the popup's initial commit.
//!
//! Codecs validate framing, receiver metadata, message versions and new object IDs.
//! New layer surfaces inherit their parent's negotiated version. IDs are registered
//! before writes; close the connection after failed or cancelled writes following
//! registration. Client IDs remain reserved until `wl_display.delete_id`.
//! Nullable outputs use `Option<u32>`. Strings are validated for wire encoding;
//! unknown enum values and anchor bits are preserved. Version 2 adds `set_layer`,
//! version 3 adds manager destruction, version 4 adds on-demand keyboard focus,
//! and version 5 adds explicit exclusive edges.
//!
//! The caller enforces surface roles and lifetimes, configure serials, mapping and
//! commit rules, valid sizes and anchors, keyboard focus, exclusive edge semantics,
//! popup parenting and compositor policy. Popup arguments are wire object IDs;
//! this codec can be enabled independently of the xdg-shell codec.
//!
//! Defined by upstream [wlr-layer-shell-unstable-v1.xml](https://gitlab.freedesktop.org/wlroots/wlr-protocols/-/blob/master/unstable/wlr-layer-shell-unstable-v1.xml).
//! Original copyright and permissive license terms are preserved in LICENSE.md.

mod zwlr_layer_shell_v1_error;
#[doc(inline)]
pub use zwlr_layer_shell_v1_error::ZwlrLayerShellV1Error;

mod zwlr_layer_shell_v1_layer;
#[doc(inline)]
pub use zwlr_layer_shell_v1_layer::ZwlrLayerShellV1Layer;

mod zwlr_layer_shell_v1_get_layer_surface_request;
#[doc(inline)]
pub use zwlr_layer_shell_v1_get_layer_surface_request::ZwlrLayerShellV1GetLayerSurfaceRequest;

mod zwlr_layer_shell_v1_destroy_request;
#[doc(inline)]
pub use zwlr_layer_shell_v1_destroy_request::ZwlrLayerShellV1DestroyRequest;

mod zwlr_layer_surface_v1_keyboard_interactivity;
#[doc(inline)]
pub use zwlr_layer_surface_v1_keyboard_interactivity::ZwlrLayerSurfaceV1KeyboardInteractivity;

mod zwlr_layer_surface_v1_error;
#[doc(inline)]
pub use zwlr_layer_surface_v1_error::ZwlrLayerSurfaceV1Error;

mod zwlr_layer_surface_v1_anchor;
#[doc(inline)]
pub use zwlr_layer_surface_v1_anchor::ZwlrLayerSurfaceV1Anchor;

mod zwlr_layer_surface_v1_set_size_request;
#[doc(inline)]
pub use zwlr_layer_surface_v1_set_size_request::ZwlrLayerSurfaceV1SetSizeRequest;

mod zwlr_layer_surface_v1_set_anchor_request;
#[doc(inline)]
pub use zwlr_layer_surface_v1_set_anchor_request::ZwlrLayerSurfaceV1SetAnchorRequest;

mod zwlr_layer_surface_v1_set_exclusive_zone_request;
#[doc(inline)]
pub use zwlr_layer_surface_v1_set_exclusive_zone_request::ZwlrLayerSurfaceV1SetExclusiveZoneRequest;

mod zwlr_layer_surface_v1_set_margin_request;
#[doc(inline)]
pub use zwlr_layer_surface_v1_set_margin_request::ZwlrLayerSurfaceV1SetMarginRequest;

mod zwlr_layer_surface_v1_set_keyboard_interactivity_request;
#[doc(inline)]
pub use zwlr_layer_surface_v1_set_keyboard_interactivity_request::ZwlrLayerSurfaceV1SetKeyboardInteractivityRequest;

mod zwlr_layer_surface_v1_get_popup_request;
#[doc(inline)]
pub use zwlr_layer_surface_v1_get_popup_request::ZwlrLayerSurfaceV1GetPopupRequest;

mod zwlr_layer_surface_v1_ack_configure_request;
#[doc(inline)]
pub use zwlr_layer_surface_v1_ack_configure_request::ZwlrLayerSurfaceV1AckConfigureRequest;

mod zwlr_layer_surface_v1_destroy_request;
#[doc(inline)]
pub use zwlr_layer_surface_v1_destroy_request::ZwlrLayerSurfaceV1DestroyRequest;

mod zwlr_layer_surface_v1_set_layer_request;
#[doc(inline)]
pub use zwlr_layer_surface_v1_set_layer_request::ZwlrLayerSurfaceV1SetLayerRequest;

mod zwlr_layer_surface_v1_set_exclusive_edge_request;
#[doc(inline)]
pub use zwlr_layer_surface_v1_set_exclusive_edge_request::ZwlrLayerSurfaceV1SetExclusiveEdgeRequest;

mod zwlr_layer_surface_v1_configure_event;
#[doc(inline)]
pub use zwlr_layer_surface_v1_configure_event::ZwlrLayerSurfaceV1ConfigureEvent;

mod zwlr_layer_surface_v1_closed_event;
#[doc(inline)]
pub use zwlr_layer_surface_v1_closed_event::ZwlrLayerSurfaceV1ClosedEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
