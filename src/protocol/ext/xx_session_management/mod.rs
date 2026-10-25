//! Encodes and decodes the xx-session-management protocol family.
//!
//! Experimental xx sessions store and restore named xdg_toplevel state. Nullable
//! session strings request a new session. Session and toplevel remove requests are
//! destructors. The caller manages persistent storage, identifiers, ownership,
//! replacement, window restoration and ordering. These wire interfaces are distinct
//! from the separately supported xdg-session-management protocol.
//!
//! Codecs validate receiver protocol, opcode, minimum version and complete framing.
//! New child IDs inherit the receiver version capped at the target maximum.
//! Registration precedes writes; failed or cancelled
//! writes after registration require closing the connection. Client IDs remain
//! reserved until `wl_display.delete_id`; destructors release server IDs.
//! Fixed values use `f64` with signed 24.8 encoding. Strings and fixed values are
//! validated before writes. Nullable values use `Option`; arrays retain raw bytes.
//! Unknown enum values and flag bits are preserved. Protocol semantics belong to
//! the caller. Original copyright and permission terms are in LICENSE.md.
//! Source: [xx-session-management-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/experimental/xx-session-management/xx-session-management-v1.xml).

mod xx_session_manager_v1_error;
#[doc(inline)]
pub use xx_session_manager_v1_error::XxSessionManagerV1Error;

mod xx_session_manager_v1_reason;
#[doc(inline)]
pub use xx_session_manager_v1_reason::XxSessionManagerV1Reason;

mod xx_session_manager_v1_destroy_request;
#[doc(inline)]
pub use xx_session_manager_v1_destroy_request::XxSessionManagerV1DestroyRequest;

mod xx_session_manager_v1_get_session_request;
#[doc(inline)]
pub use xx_session_manager_v1_get_session_request::XxSessionManagerV1GetSessionRequest;

mod xx_session_v1_error;
#[doc(inline)]
pub use xx_session_v1_error::XxSessionV1Error;

mod xx_session_v1_destroy_request;
#[doc(inline)]
pub use xx_session_v1_destroy_request::XxSessionV1DestroyRequest;

mod xx_session_v1_remove_request;
#[doc(inline)]
pub use xx_session_v1_remove_request::XxSessionV1RemoveRequest;

mod xx_session_v1_add_toplevel_request;
#[doc(inline)]
pub use xx_session_v1_add_toplevel_request::XxSessionV1AddToplevelRequest;

mod xx_session_v1_restore_toplevel_request;
#[doc(inline)]
pub use xx_session_v1_restore_toplevel_request::XxSessionV1RestoreToplevelRequest;

mod xx_session_v1_created_event;
#[doc(inline)]
pub use xx_session_v1_created_event::XxSessionV1CreatedEvent;

mod xx_session_v1_restored_event;
#[doc(inline)]
pub use xx_session_v1_restored_event::XxSessionV1RestoredEvent;

mod xx_session_v1_replaced_event;
#[doc(inline)]
pub use xx_session_v1_replaced_event::XxSessionV1ReplacedEvent;

mod xx_toplevel_session_v1_destroy_request;
#[doc(inline)]
pub use xx_toplevel_session_v1_destroy_request::XxToplevelSessionV1DestroyRequest;

mod xx_toplevel_session_v1_remove_request;
#[doc(inline)]
pub use xx_toplevel_session_v1_remove_request::XxToplevelSessionV1RemoveRequest;

mod xx_toplevel_session_v1_restored_event;
#[doc(inline)]
pub use xx_toplevel_session_v1_restored_event::XxToplevelSessionV1RestoredEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
