//! Encodes and decodes the xx-zones protocol family.
//!
//! Managers create zone descriptors and items from xdg_toplevels. XxZones advertise
//! sizes and exported handles. Item events report frame extents, placement and
//! failed positioning; zone events report membership. The caller applies pending
//! state on surface commits, validates handles, manages output and toplevel
//! lifetimes, bounds and authorization for placement.
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
//! Source: [xx-zones-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/experimental/xx-zones/xx-zones-v1.xml).

mod xx_zone_manager_v1_destroy_request;
#[doc(inline)]
pub use xx_zone_manager_v1_destroy_request::XxZoneManagerV1DestroyRequest;

mod xx_zone_manager_v1_get_zone_item_request;
#[doc(inline)]
pub use xx_zone_manager_v1_get_zone_item_request::XxZoneManagerV1GetZoneItemRequest;

mod xx_zone_manager_v1_get_zone_request;
#[doc(inline)]
pub use xx_zone_manager_v1_get_zone_request::XxZoneManagerV1GetZoneRequest;

mod xx_zone_manager_v1_get_zone_from_handle_request;
#[doc(inline)]
pub use xx_zone_manager_v1_get_zone_from_handle_request::XxZoneManagerV1GetZoneFromHandleRequest;

mod xx_zone_item_v1_destroy_request;
#[doc(inline)]
pub use xx_zone_item_v1_destroy_request::XxZoneItemV1DestroyRequest;

mod xx_zone_item_v1_set_position_request;
#[doc(inline)]
pub use xx_zone_item_v1_set_position_request::XxZoneItemV1SetPositionRequest;

mod xx_zone_item_v1_frame_extents_event;
#[doc(inline)]
pub use xx_zone_item_v1_frame_extents_event::XxZoneItemV1FrameExtentsEvent;

mod xx_zone_item_v1_position_event;
#[doc(inline)]
pub use xx_zone_item_v1_position_event::XxZoneItemV1PositionEvent;

mod xx_zone_item_v1_position_failed_event;
#[doc(inline)]
pub use xx_zone_item_v1_position_failed_event::XxZoneItemV1PositionFailedEvent;

mod xx_zone_item_v1_closed_event;
#[doc(inline)]
pub use xx_zone_item_v1_closed_event::XxZoneItemV1ClosedEvent;

mod xx_zone_v1_error;
#[doc(inline)]
pub use xx_zone_v1_error::XxZoneV1Error;

mod xx_zone_v1_destroy_request;
#[doc(inline)]
pub use xx_zone_v1_destroy_request::XxZoneV1DestroyRequest;

mod xx_zone_v1_add_item_request;
#[doc(inline)]
pub use xx_zone_v1_add_item_request::XxZoneV1AddItemRequest;

mod xx_zone_v1_remove_item_request;
#[doc(inline)]
pub use xx_zone_v1_remove_item_request::XxZoneV1RemoveItemRequest;

mod xx_zone_v1_size_event;
#[doc(inline)]
pub use xx_zone_v1_size_event::XxZoneV1SizeEvent;

mod xx_zone_v1_handle_event;
#[doc(inline)]
pub use xx_zone_v1_handle_event::XxZoneV1HandleEvent;

mod xx_zone_v1_done_event;
#[doc(inline)]
pub use xx_zone_v1_done_event::XxZoneV1DoneEvent;

mod xx_zone_v1_item_blocked_event;
#[doc(inline)]
pub use xx_zone_v1_item_blocked_event::XxZoneV1ItemBlockedEvent;

mod xx_zone_v1_item_entered_event;
#[doc(inline)]
pub use xx_zone_v1_item_entered_event::XxZoneV1ItemEnteredEvent;

mod xx_zone_v1_item_left_event;
#[doc(inline)]
pub use xx_zone_v1_item_left_event::XxZoneV1ItemLeftEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
