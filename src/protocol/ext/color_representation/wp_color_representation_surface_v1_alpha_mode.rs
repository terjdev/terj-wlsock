use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_color_representation_surface_v1.alpha_mode`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="alpha_mode">
///       <description summary="alpha mode">
///         Specifies how the alpha channel affects the color channels.
///       </description>
///       <entry name="premultiplied_electrical" value="0">
///         <description summary="premultiplied alpha in electrical values">
///           Electrical color channel values (after transfer function encoding)
///           are already multiplied with the alpha channel value.
///         </description>
///       </entry>
///       <entry name="premultiplied_optical" value="1">
///         <description summary="premultiplied alpha in optical values">
///           Optical color channel values (before transfer function encoding)
///           are already multiplied with the alpha channel value.
///         </description>
///       </entry>
///       <entry name="straight" value="2">
///         <description summary="straight alpha">
///           Alpha channel has not been pre-multiplied into color channels.
///         </description>
///       </entry>
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpColorRepresentationSurfaceV1AlphaMode(pub u32);
impl WpColorRepresentationSurfaceV1AlphaMode {
    /// premultiplied alpha in electrical values; available since version 1.
    pub const PREMULTIPLIED_ELECTRICAL: Self = Self(0);
    /// premultiplied alpha in optical values; available since version 1.
    pub const PREMULTIPLIED_OPTICAL: Self = Self(1);
    /// straight alpha; available since version 1.
    pub const STRAIGHT: Self = Self(2);
}
impl CommandReader<Self> for WpColorRepresentationSurfaceV1AlphaMode {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpColorRepresentationSurfaceV1AlphaMode {
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
