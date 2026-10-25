use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_color_representation_surface_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <description summary="protocol errors" />
///       <entry name="alpha_mode" value="1" summary="unsupported alpha mode" />
///       <entry name="coefficients" value="2" summary="unsupported coefficients" />
///       <entry name="pixel_format" value="3" summary="the pixel format and a set value are incompatible" />
///       <entry name="inert" value="4" summary="forbidden request on inert object" />
///       <entry name="chroma_location" value="5" summary="invalid chroma location" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpColorRepresentationSurfaceV1Error(pub u32);
impl WpColorRepresentationSurfaceV1Error {
    /// unsupported alpha mode; available since version 1.
    pub const ALPHA_MODE: Self = Self(1);
    /// unsupported coefficients; available since version 1.
    pub const COEFFICIENTS: Self = Self(2);
    /// the pixel format and a set value are incompatible; available since version 1.
    pub const PIXEL_FORMAT: Self = Self(3);
    /// forbidden request on inert object; available since version 1.
    pub const INERT: Self = Self(4);
    /// invalid chroma location; available since version 1.
    pub const CHROMA_LOCATION: Self = Self(5);
}
impl CommandReader<Self> for WpColorRepresentationSurfaceV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpColorRepresentationSurfaceV1Error {
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
