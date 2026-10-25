//! Encodes and decodes the ext-idle-notify extension.
//!
//! Per-seat notifications report idle and resumed state. Version 2 adds input-idle
//! notifications independent of idle inhibitors. The caller tracks elapsed time,
//! seat activity, inhibitors and notification state.
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
//! Defined by upstream [ext-idle-notify-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/ext-idle-notify/ext-idle-notify-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod ext_idle_notifier_v1_destroy_request;
#[doc(inline)]
pub use ext_idle_notifier_v1_destroy_request::ExtIdleNotifierV1DestroyRequest;

mod ext_idle_notifier_v1_get_idle_notification_request;
#[doc(inline)]
pub use ext_idle_notifier_v1_get_idle_notification_request::ExtIdleNotifierV1GetIdleNotificationRequest;

mod ext_idle_notifier_v1_get_input_idle_notification_request;
#[doc(inline)]
pub use ext_idle_notifier_v1_get_input_idle_notification_request::ExtIdleNotifierV1GetInputIdleNotificationRequest;

mod ext_idle_notification_v1_destroy_request;
#[doc(inline)]
pub use ext_idle_notification_v1_destroy_request::ExtIdleNotificationV1DestroyRequest;

mod ext_idle_notification_v1_idled_event;
#[doc(inline)]
pub use ext_idle_notification_v1_idled_event::ExtIdleNotificationV1IdledEvent;

mod ext_idle_notification_v1_resumed_event;
#[doc(inline)]
pub use ext_idle_notification_v1_resumed_event::ExtIdleNotificationV1ResumedEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
pub(crate) use object_interface::ObjectInteface;
