//! Encodes and decodes the xx-keyboard-filter protocol family.
//!
//! Filters bind a keyboard, an experimental xx_input_method_v1 object and a surface.
//! The caller controls binding, event serials, filtering decisions and object
//! lifetimes. Object arguments are wire IDs; enable ext-xx-input-method separately
//! when using the experimental input-method codec.
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
//! This protocol family defines no events.
//! Source: [xx-keyboard-filter-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/experimental/xx-keyboard-filter/xx-keyboard-filter-v1.xml).

mod xx_keyboard_filter_v1_error;
#[doc(inline)]
pub use xx_keyboard_filter_v1_error::XxKeyboardFilterV1Error;

mod xx_keyboard_filter_v1_filter_action;
#[doc(inline)]
pub use xx_keyboard_filter_v1_filter_action::XxKeyboardFilterV1FilterAction;

mod xx_keyboard_filter_v1_unbind_request;
#[doc(inline)]
pub use xx_keyboard_filter_v1_unbind_request::XxKeyboardFilterV1UnbindRequest;

mod xx_keyboard_filter_v1_filter_request;
#[doc(inline)]
pub use xx_keyboard_filter_v1_filter_request::XxKeyboardFilterV1FilterRequest;

mod xx_keyboard_filter_v1_destroy_request;
#[doc(inline)]
pub use xx_keyboard_filter_v1_destroy_request::XxKeyboardFilterV1DestroyRequest;

mod xx_keyboard_filter_manager_v1_error;
#[doc(inline)]
pub use xx_keyboard_filter_manager_v1_error::XxKeyboardFilterManagerV1Error;

mod xx_keyboard_filter_manager_v1_bind_to_input_method_request;
#[doc(inline)]
pub use xx_keyboard_filter_manager_v1_bind_to_input_method_request::XxKeyboardFilterManagerV1BindToInputMethodRequest;

mod xx_keyboard_filter_manager_v1_destroy_request;
#[doc(inline)]
pub use xx_keyboard_filter_manager_v1_destroy_request::XxKeyboardFilterManagerV1DestroyRequest;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
