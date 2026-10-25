//! Encodes and decodes the Presentation time extension, interface versions 1 through 3.
//!
//! Defined by the upstream [Presentation time XML](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/stable/presentation-time/presentation-time.xml).
//! Compositor policy and resource lifetimes remain with the caller. Version 2 changes
//! the meaning of `refresh` on variable refresh rate outputs without changing the wire format.
//! Version 3 adds fixed and variable refresh rate flags to presentation feedback.
//! Original copyright and MIT permission terms are preserved in this directory's LICENSE.md.

mod wp_presentation_error;
#[doc(inline)]
pub use wp_presentation_error::WpPresentationError;

mod wp_presentation_destroy_request;
#[doc(inline)]
pub use wp_presentation_destroy_request::WpPresentationDestroyRequest;

mod wp_presentation_feedback_request;
#[doc(inline)]
pub use wp_presentation_feedback_request::WpPresentationFeedbackRequest;

mod wp_presentation_clock_id_event;
#[doc(inline)]
pub use wp_presentation_clock_id_event::WpPresentationClockIdEvent;

mod wp_presentation_feedback_kind;
#[doc(inline)]
pub use wp_presentation_feedback_kind::WpPresentationFeedbackKind;

mod wp_presentation_feedback_sync_output_event;
#[doc(inline)]
pub use wp_presentation_feedback_sync_output_event::WpPresentationFeedbackSyncOutputEvent;

mod wp_presentation_feedback_presented_event;
#[doc(inline)]
pub use wp_presentation_feedback_presented_event::WpPresentationFeedbackPresentedEvent;

mod wp_presentation_feedback_discarded_event;
#[doc(inline)]
pub use wp_presentation_feedback_discarded_event::WpPresentationFeedbackDiscardedEvent;
