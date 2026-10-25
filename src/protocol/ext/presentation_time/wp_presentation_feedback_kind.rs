use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Presentation-time value `wp_presentation_feedback.kind`.
///
/// Unknown numeric values are preserved. This value encodes four bytes.
/// Combine flags with `|` and intersect them with `&`.
/// ```xml
/// <enum name="kind" bitfield="true">
///       <description summary="bitmask of flags in presented event">
///         These flags provide information about how the presentation of
///         the related content update was done. The intent is to help
///         clients assess the reliability of the feedback and the visual
///         quality with respect to possible tearing and timings.
///       </description>
///       <entry name="vsync" value="0x1">
///         <description summary="presentation was vsync'd">
///           The presentation was synchronized to the "vertical retrace" by
///           the display hardware such that tearing does not happen.
///           Relying on software scheduling is not acceptable for this
///           flag. If presentation is done by a copy to the active
///           frontbuffer, then it must guarantee that tearing cannot
///           happen.
///         </description>
///       </entry>
///       <entry name="hw_clock" value="0x2">
///         <description summary="hardware provided the presentation timestamp">
///           The display hardware provided measurements that the hardware
///           driver converted into a presentation timestamp. Sampling a
///           clock in software is not acceptable for this flag.
///         </description>
///       </entry>
///       <entry name="hw_completion" value="0x4">
///         <description summary="hardware signalled the start of the presentation">
///           The display hardware signalled that it started using the new
///           image content. The opposite of this is e.g. a timer being used
///           to guess when the display hardware has switched to the new
///           image content.
///         </description>
///       </entry>
///       <entry name="zero_copy" value="0x8">
///         <description summary="presentation was done zero-copy">
///           The presentation of this update was done zero-copy. This means
///           the buffer from the client was given to display hardware as
///           is, without copying it. Compositing with OpenGL counts as
///           copying, even if textured directly from the client buffer.
///           Possible zero-copy cases include direct scanout of a
///           fullscreen surface and a surface on a hardware overlay.
///         </description>
///       </entry>
///       <entry name="fixed_rate" value="0x10" since="3">
///         <description summary="presentation is at a fixed refresh rate">
///           The display hardware is operating at a fixed refresh rate.
///           The presentation was aligned to a multiple of this event's
///           'refresh' argument.
///           If this is set, the refresh argument must not be 0.
///         </description>
///       </entry>
///       <entry name="variable_rate" value="0x20" since="3">
///         <description summary="presentation is at a variable refresh rate">
///           The display hardware is operating at a variable refresh rate, with
///           updates to this surface consistently determining the display refresh
///           timing. The definition of 'consistent' is compositor policy.
///
///           The content update was displayed as soon as possible after it became
///           ready, but no sooner than 'refresh' nanoseconds after the previous
///           presentation on this surface. In other words, the presentation time
///           was determined by when this surface's content became ready, not by
///           a periodic display refresh or by updates to other surfaces.
///
///           If this is set, the refresh argument must not be 0.
///         </description>
///       </entry>
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpPresentationFeedbackKind(pub u32);
impl WpPresentationFeedbackKind {
    /// presentation was vsync'd; XML since=1.
    pub const VSYNC: Self = Self(0x1);
    /// hardware provided the presentation timestamp; XML since=1.
    pub const HW_CLOCK: Self = Self(0x2);
    /// hardware signalled the start of the presentation; XML since=1.
    pub const HW_COMPLETION: Self = Self(0x4);
    /// presentation was done zero-copy; XML since=1.
    pub const ZERO_COPY: Self = Self(0x8);
    /// Presentation is at a fixed refresh rate; XML since=3.
    /// The refresh argument must be nonzero.
    pub const FIXED_RATE: Self = Self(0x10);
    /// Presentation is at a variable refresh rate; XML since=3.
    /// The refresh argument must be nonzero.
    pub const VARIABLE_RATE: Self = Self(0x20);
}
impl std::ops::BitOr for WpPresentationFeedbackKind {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl std::ops::BitAnd for WpPresentationFeedbackKind {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl CommandReader<Self> for WpPresentationFeedbackKind {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpPresentationFeedbackKind {
    fn object_id(&self) -> u32 {
        0
    }
    fn opcode(&self) -> u16 {
        0
    }
    fn payload_size(&self) -> usize {
        4
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        writer.next_u32(self.0).await
    }
}
