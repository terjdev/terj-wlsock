//! Encodes and decodes the pointer-constraints protocol family.
//!
//! Locked and confined pointers use nullable regions with copy semantics and
//! persistent or one-shot lifetimes. The caller enforces focus, region state,
//! activation, exclusivity and commit-time cursor-position hints.
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
//! Source: [pointer-constraints-unstable-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/unstable/pointer-constraints/pointer-constraints-unstable-v1.xml).

mod zwp_pointer_constraints_v1_error;
#[doc(inline)]
pub use zwp_pointer_constraints_v1_error::ZwpPointerConstraintsV1Error;

mod zwp_pointer_constraints_v1_lifetime;
#[doc(inline)]
pub use zwp_pointer_constraints_v1_lifetime::ZwpPointerConstraintsV1Lifetime;

mod zwp_pointer_constraints_v1_destroy_request;
#[doc(inline)]
pub use zwp_pointer_constraints_v1_destroy_request::ZwpPointerConstraintsV1DestroyRequest;

mod zwp_pointer_constraints_v1_lock_pointer_request;
#[doc(inline)]
pub use zwp_pointer_constraints_v1_lock_pointer_request::ZwpPointerConstraintsV1LockPointerRequest;

mod zwp_pointer_constraints_v1_confine_pointer_request;
#[doc(inline)]
pub use zwp_pointer_constraints_v1_confine_pointer_request::ZwpPointerConstraintsV1ConfinePointerRequest;

mod zwp_locked_pointer_v1_destroy_request;
#[doc(inline)]
pub use zwp_locked_pointer_v1_destroy_request::ZwpLockedPointerV1DestroyRequest;

mod zwp_locked_pointer_v1_set_cursor_position_hint_request;
#[doc(inline)]
pub use zwp_locked_pointer_v1_set_cursor_position_hint_request::ZwpLockedPointerV1SetCursorPositionHintRequest;

mod zwp_locked_pointer_v1_set_region_request;
#[doc(inline)]
pub use zwp_locked_pointer_v1_set_region_request::ZwpLockedPointerV1SetRegionRequest;

mod zwp_locked_pointer_v1_locked_event;
#[doc(inline)]
pub use zwp_locked_pointer_v1_locked_event::ZwpLockedPointerV1LockedEvent;

mod zwp_locked_pointer_v1_unlocked_event;
#[doc(inline)]
pub use zwp_locked_pointer_v1_unlocked_event::ZwpLockedPointerV1UnlockedEvent;

mod zwp_confined_pointer_v1_destroy_request;
#[doc(inline)]
pub use zwp_confined_pointer_v1_destroy_request::ZwpConfinedPointerV1DestroyRequest;

mod zwp_confined_pointer_v1_set_region_request;
#[doc(inline)]
pub use zwp_confined_pointer_v1_set_region_request::ZwpConfinedPointerV1SetRegionRequest;

mod zwp_confined_pointer_v1_confined_event;
#[doc(inline)]
pub use zwp_confined_pointer_v1_confined_event::ZwpConfinedPointerV1ConfinedEvent;

mod zwp_confined_pointer_v1_unconfined_event;
#[doc(inline)]
pub use zwp_confined_pointer_v1_unconfined_event::ZwpConfinedPointerV1UnconfinedEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
