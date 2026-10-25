//! Encodes and decodes the foreign-toplevel-management protocol family.
//!
//! The manager announces server-created handles for windows and destroys itself
//! with finished after stop. Handles advertise properties and native-endian u32
//! state arrays; done groups updates. Closed handles remain inert until destroy.
//! Version 2 adds fullscreen control; version 3 adds parent relationships. The
//! caller enforces window lifetimes, focus policy, rectangles and requested actions.
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
//! Source: [wlr-foreign-toplevel-management-unstable-v1.xml](https://gitlab.freedesktop.org/wlroots/wlr-protocols/-/blob/master/unstable/wlr-foreign-toplevel-management-unstable-v1.xml).

mod zwlr_foreign_toplevel_manager_v1_stop_request;
#[doc(inline)]
pub use zwlr_foreign_toplevel_manager_v1_stop_request::ZwlrForeignToplevelManagerV1StopRequest;

mod zwlr_foreign_toplevel_manager_v1_toplevel_event;
#[doc(inline)]
pub use zwlr_foreign_toplevel_manager_v1_toplevel_event::ZwlrForeignToplevelManagerV1ToplevelEvent;

mod zwlr_foreign_toplevel_manager_v1_finished_event;
#[doc(inline)]
pub use zwlr_foreign_toplevel_manager_v1_finished_event::ZwlrForeignToplevelManagerV1FinishedEvent;

mod zwlr_foreign_toplevel_handle_v1_state;
#[doc(inline)]
pub use zwlr_foreign_toplevel_handle_v1_state::ZwlrForeignToplevelHandleV1State;

mod zwlr_foreign_toplevel_handle_v1_error;
#[doc(inline)]
pub use zwlr_foreign_toplevel_handle_v1_error::ZwlrForeignToplevelHandleV1Error;

mod zwlr_foreign_toplevel_handle_v1_set_maximized_request;
#[doc(inline)]
pub use zwlr_foreign_toplevel_handle_v1_set_maximized_request::ZwlrForeignToplevelHandleV1SetMaximizedRequest;

mod zwlr_foreign_toplevel_handle_v1_unset_maximized_request;
#[doc(inline)]
pub use zwlr_foreign_toplevel_handle_v1_unset_maximized_request::ZwlrForeignToplevelHandleV1UnsetMaximizedRequest;

mod zwlr_foreign_toplevel_handle_v1_set_minimized_request;
#[doc(inline)]
pub use zwlr_foreign_toplevel_handle_v1_set_minimized_request::ZwlrForeignToplevelHandleV1SetMinimizedRequest;

mod zwlr_foreign_toplevel_handle_v1_unset_minimized_request;
#[doc(inline)]
pub use zwlr_foreign_toplevel_handle_v1_unset_minimized_request::ZwlrForeignToplevelHandleV1UnsetMinimizedRequest;

mod zwlr_foreign_toplevel_handle_v1_activate_request;
#[doc(inline)]
pub use zwlr_foreign_toplevel_handle_v1_activate_request::ZwlrForeignToplevelHandleV1ActivateRequest;

mod zwlr_foreign_toplevel_handle_v1_close_request;
#[doc(inline)]
pub use zwlr_foreign_toplevel_handle_v1_close_request::ZwlrForeignToplevelHandleV1CloseRequest;

mod zwlr_foreign_toplevel_handle_v1_set_rectangle_request;
#[doc(inline)]
pub use zwlr_foreign_toplevel_handle_v1_set_rectangle_request::ZwlrForeignToplevelHandleV1SetRectangleRequest;

mod zwlr_foreign_toplevel_handle_v1_destroy_request;
#[doc(inline)]
pub use zwlr_foreign_toplevel_handle_v1_destroy_request::ZwlrForeignToplevelHandleV1DestroyRequest;

mod zwlr_foreign_toplevel_handle_v1_set_fullscreen_request;
#[doc(inline)]
pub use zwlr_foreign_toplevel_handle_v1_set_fullscreen_request::ZwlrForeignToplevelHandleV1SetFullscreenRequest;

mod zwlr_foreign_toplevel_handle_v1_unset_fullscreen_request;
#[doc(inline)]
pub use zwlr_foreign_toplevel_handle_v1_unset_fullscreen_request::ZwlrForeignToplevelHandleV1UnsetFullscreenRequest;

mod zwlr_foreign_toplevel_handle_v1_title_event;
#[doc(inline)]
pub use zwlr_foreign_toplevel_handle_v1_title_event::ZwlrForeignToplevelHandleV1TitleEvent;

mod zwlr_foreign_toplevel_handle_v1_app_id_event;
#[doc(inline)]
pub use zwlr_foreign_toplevel_handle_v1_app_id_event::ZwlrForeignToplevelHandleV1AppIdEvent;

mod zwlr_foreign_toplevel_handle_v1_output_enter_event;
#[doc(inline)]
pub use zwlr_foreign_toplevel_handle_v1_output_enter_event::ZwlrForeignToplevelHandleV1OutputEnterEvent;

mod zwlr_foreign_toplevel_handle_v1_output_leave_event;
#[doc(inline)]
pub use zwlr_foreign_toplevel_handle_v1_output_leave_event::ZwlrForeignToplevelHandleV1OutputLeaveEvent;

mod zwlr_foreign_toplevel_handle_v1_state_event;
#[doc(inline)]
pub use zwlr_foreign_toplevel_handle_v1_state_event::ZwlrForeignToplevelHandleV1StateEvent;

mod zwlr_foreign_toplevel_handle_v1_done_event;
#[doc(inline)]
pub use zwlr_foreign_toplevel_handle_v1_done_event::ZwlrForeignToplevelHandleV1DoneEvent;

mod zwlr_foreign_toplevel_handle_v1_closed_event;
#[doc(inline)]
pub use zwlr_foreign_toplevel_handle_v1_closed_event::ZwlrForeignToplevelHandleV1ClosedEvent;

mod zwlr_foreign_toplevel_handle_v1_parent_event;
#[doc(inline)]
pub use zwlr_foreign_toplevel_handle_v1_parent_event::ZwlrForeignToplevelHandleV1ParentEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
