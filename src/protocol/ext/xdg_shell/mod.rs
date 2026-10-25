//! Encodes and decodes the stable xdg-shell extension, through interface version 7.
//!
//! `xdg_wm_base` creates positioners and shell surfaces. Shell surfaces receive
//! either a toplevel or popup role, with configure events synchronized by
//! `xdg_surface.configure` and acknowledged by `ack_configure` before committing
//! new content. Initial mapping requires a bufferless commit and an acknowledged
//! initial configuration. Positioner rules control popup placement and repositioning.
//!
//! Codecs check framing, receiver metadata, message versions and new object IDs.
//! Child objects inherit their parent's negotiated version. New IDs are registered
//! before writes; failed or cancelled writes after registration require closing
//! the connection. Client IDs remain reserved until `wl_display.delete_id`.
//! Nullable object arguments use `Option<u32>`; arrays contain raw native-endian
//! `u32` values for toplevel states and window-manager capabilities. Unknown enum
//! values and flag bits are preserved. Strings are validated for wire encoding.
//!
//! The caller enforces surface roles and lifetimes, configure serial ordering,
//! mapping and commit rules, positioner completeness, popup stacks and grabs,
//! interactive-operation serials, geometry constraints, and compositor policy.
//! Ping events require a corresponding pong request from the client.
//!
//! Derived from upstream [xdg-shell.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/stable/xdg-shell/xdg-shell.xml).
//! Original copyright notices and MIT permission terms are preserved in LICENSE.md.

mod xdg_wm_base_error;
#[doc(inline)]
pub use xdg_wm_base_error::XdgWmBaseError;

mod xdg_wm_base_destroy_request;
#[doc(inline)]
pub use xdg_wm_base_destroy_request::XdgWmBaseDestroyRequest;

mod xdg_wm_base_create_positioner_request;
#[doc(inline)]
pub use xdg_wm_base_create_positioner_request::XdgWmBaseCreatePositionerRequest;

mod xdg_wm_base_get_xdg_surface_request;
#[doc(inline)]
pub use xdg_wm_base_get_xdg_surface_request::XdgWmBaseGetXdgSurfaceRequest;

mod xdg_wm_base_pong_request;
#[doc(inline)]
pub use xdg_wm_base_pong_request::XdgWmBasePongRequest;

mod xdg_wm_base_ping_event;
#[doc(inline)]
pub use xdg_wm_base_ping_event::XdgWmBasePingEvent;

mod xdg_positioner_error;
#[doc(inline)]
pub use xdg_positioner_error::XdgPositionerError;

mod xdg_positioner_anchor;
#[doc(inline)]
pub use xdg_positioner_anchor::XdgPositionerAnchor;

mod xdg_positioner_gravity;
#[doc(inline)]
pub use xdg_positioner_gravity::XdgPositionerGravity;

mod xdg_positioner_constraint_adjustment;
#[doc(inline)]
pub use xdg_positioner_constraint_adjustment::XdgPositionerConstraintAdjustment;

mod xdg_positioner_destroy_request;
#[doc(inline)]
pub use xdg_positioner_destroy_request::XdgPositionerDestroyRequest;

mod xdg_positioner_set_size_request;
#[doc(inline)]
pub use xdg_positioner_set_size_request::XdgPositionerSetSizeRequest;

mod xdg_positioner_set_anchor_rect_request;
#[doc(inline)]
pub use xdg_positioner_set_anchor_rect_request::XdgPositionerSetAnchorRectRequest;

mod xdg_positioner_set_anchor_request;
#[doc(inline)]
pub use xdg_positioner_set_anchor_request::XdgPositionerSetAnchorRequest;

mod xdg_positioner_set_gravity_request;
#[doc(inline)]
pub use xdg_positioner_set_gravity_request::XdgPositionerSetGravityRequest;

mod xdg_positioner_set_constraint_adjustment_request;
#[doc(inline)]
pub use xdg_positioner_set_constraint_adjustment_request::XdgPositionerSetConstraintAdjustmentRequest;

mod xdg_positioner_set_offset_request;
#[doc(inline)]
pub use xdg_positioner_set_offset_request::XdgPositionerSetOffsetRequest;

mod xdg_positioner_set_reactive_request;
#[doc(inline)]
pub use xdg_positioner_set_reactive_request::XdgPositionerSetReactiveRequest;

mod xdg_positioner_set_parent_size_request;
#[doc(inline)]
pub use xdg_positioner_set_parent_size_request::XdgPositionerSetParentSizeRequest;

mod xdg_positioner_set_parent_configure_request;
#[doc(inline)]
pub use xdg_positioner_set_parent_configure_request::XdgPositionerSetParentConfigureRequest;

mod xdg_surface_error;
#[doc(inline)]
pub use xdg_surface_error::XdgSurfaceError;

mod xdg_surface_destroy_request;
#[doc(inline)]
pub use xdg_surface_destroy_request::XdgSurfaceDestroyRequest;

mod xdg_surface_get_toplevel_request;
#[doc(inline)]
pub use xdg_surface_get_toplevel_request::XdgSurfaceGetToplevelRequest;

mod xdg_surface_get_popup_request;
#[doc(inline)]
pub use xdg_surface_get_popup_request::XdgSurfaceGetPopupRequest;

mod xdg_surface_set_window_geometry_request;
#[doc(inline)]
pub use xdg_surface_set_window_geometry_request::XdgSurfaceSetWindowGeometryRequest;

mod xdg_surface_ack_configure_request;
#[doc(inline)]
pub use xdg_surface_ack_configure_request::XdgSurfaceAckConfigureRequest;

mod xdg_surface_configure_event;
#[doc(inline)]
pub use xdg_surface_configure_event::XdgSurfaceConfigureEvent;

mod xdg_toplevel_error;
#[doc(inline)]
pub use xdg_toplevel_error::XdgToplevelError;

mod xdg_toplevel_resize_edge;
#[doc(inline)]
pub use xdg_toplevel_resize_edge::XdgToplevelResizeEdge;

mod xdg_toplevel_state;
#[doc(inline)]
pub use xdg_toplevel_state::XdgToplevelState;

mod xdg_toplevel_wm_capabilities;
#[doc(inline)]
pub use xdg_toplevel_wm_capabilities::XdgToplevelWmCapabilities;

mod xdg_toplevel_destroy_request;
#[doc(inline)]
pub use xdg_toplevel_destroy_request::XdgToplevelDestroyRequest;

mod xdg_toplevel_set_parent_request;
#[doc(inline)]
pub use xdg_toplevel_set_parent_request::XdgToplevelSetParentRequest;

mod xdg_toplevel_set_title_request;
#[doc(inline)]
pub use xdg_toplevel_set_title_request::XdgToplevelSetTitleRequest;

mod xdg_toplevel_set_app_id_request;
#[doc(inline)]
pub use xdg_toplevel_set_app_id_request::XdgToplevelSetAppIdRequest;

mod xdg_toplevel_show_window_menu_request;
#[doc(inline)]
pub use xdg_toplevel_show_window_menu_request::XdgToplevelShowWindowMenuRequest;

mod xdg_toplevel_move_request;
#[doc(inline)]
pub use xdg_toplevel_move_request::XdgToplevelMoveRequest;

mod xdg_toplevel_resize_request;
#[doc(inline)]
pub use xdg_toplevel_resize_request::XdgToplevelResizeRequest;

mod xdg_toplevel_set_max_size_request;
#[doc(inline)]
pub use xdg_toplevel_set_max_size_request::XdgToplevelSetMaxSizeRequest;

mod xdg_toplevel_set_min_size_request;
#[doc(inline)]
pub use xdg_toplevel_set_min_size_request::XdgToplevelSetMinSizeRequest;

mod xdg_toplevel_set_maximized_request;
#[doc(inline)]
pub use xdg_toplevel_set_maximized_request::XdgToplevelSetMaximizedRequest;

mod xdg_toplevel_unset_maximized_request;
#[doc(inline)]
pub use xdg_toplevel_unset_maximized_request::XdgToplevelUnsetMaximizedRequest;

mod xdg_toplevel_set_fullscreen_request;
#[doc(inline)]
pub use xdg_toplevel_set_fullscreen_request::XdgToplevelSetFullscreenRequest;

mod xdg_toplevel_unset_fullscreen_request;
#[doc(inline)]
pub use xdg_toplevel_unset_fullscreen_request::XdgToplevelUnsetFullscreenRequest;

mod xdg_toplevel_set_minimized_request;
#[doc(inline)]
pub use xdg_toplevel_set_minimized_request::XdgToplevelSetMinimizedRequest;

mod xdg_toplevel_configure_event;
#[doc(inline)]
pub use xdg_toplevel_configure_event::XdgToplevelConfigureEvent;

mod xdg_toplevel_close_event;
#[doc(inline)]
pub use xdg_toplevel_close_event::XdgToplevelCloseEvent;

mod xdg_toplevel_configure_bounds_event;
#[doc(inline)]
pub use xdg_toplevel_configure_bounds_event::XdgToplevelConfigureBoundsEvent;

mod xdg_toplevel_wm_capabilities_event;
#[doc(inline)]
pub use xdg_toplevel_wm_capabilities_event::XdgToplevelWmCapabilitiesEvent;

mod xdg_popup_error;
#[doc(inline)]
pub use xdg_popup_error::XdgPopupError;

mod xdg_popup_destroy_request;
#[doc(inline)]
pub use xdg_popup_destroy_request::XdgPopupDestroyRequest;

mod xdg_popup_grab_request;
#[doc(inline)]
pub use xdg_popup_grab_request::XdgPopupGrabRequest;

mod xdg_popup_reposition_request;
#[doc(inline)]
pub use xdg_popup_reposition_request::XdgPopupRepositionRequest;

mod xdg_popup_configure_event;
#[doc(inline)]
pub use xdg_popup_configure_event::XdgPopupConfigureEvent;

mod xdg_popup_popup_done_event;
#[doc(inline)]
pub use xdg_popup_popup_done_event::XdgPopupPopupDoneEvent;

mod xdg_popup_repositioned_event;
#[doc(inline)]
pub use xdg_popup_repositioned_event::XdgPopupRepositionedEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
