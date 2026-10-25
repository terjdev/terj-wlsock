use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_color_manager_v1.render_intent`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="render_intent">
///       <description summary="rendering intents">
///         See the ICC.1:2022 specification from the International Color Consortium
///         for more details about rendering intents.
///
///         The principles of ICC defined rendering intents apply with all types of
///         image descriptions, not only those with ICC file profiles.
///
///         Compositors must support the perceptual rendering intent. Other
///         rendering intents are optional.
///       </description>
///
///       <entry name="perceptual" value="0" summary="perceptual" />
///       <entry name="relative" value="1" summary="media-relative colorimetric" />
///       <entry name="saturation" value="2" summary="saturation" />
///       <entry name="absolute" value="3" summary="ICC-absolute colorimetric" />
///       <entry name="relative_bpc" value="4" summary="media-relative colorimetric + black point compensation" />
///       <entry name="absolute_no_adaptation" value="5" since="2">
///         <description summary="ICC-absolute colorimetric without adaptation">
///           This rendering intent is a modified absolute rendering intent that
///           assumes the viewer is not adapted to the display white point, so no
///           chromatic adaptation between surface and display is done.
///           This can be useful for color proofing applications.
///         </description>
///       </entry>
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpColorManagerV1RenderIntent(pub u32);
impl WpColorManagerV1RenderIntent {
    /// perceptual; available since version 1.
    pub const PERCEPTUAL: Self = Self(0);
    /// media-relative colorimetric; available since version 1.
    pub const RELATIVE: Self = Self(1);
    /// saturation; available since version 1.
    pub const SATURATION: Self = Self(2);
    /// ICC-absolute colorimetric; available since version 1.
    pub const ABSOLUTE: Self = Self(3);
    /// media-relative colorimetric + black point compensation; available since version 1.
    pub const RELATIVE_BPC: Self = Self(4);
    /// ICC-absolute colorimetric without adaptation; available since version 2.
    pub const ABSOLUTE_NO_ADAPTATION: Self = Self(5);
}
impl CommandReader<Self> for WpColorManagerV1RenderIntent {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpColorManagerV1RenderIntent {
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
