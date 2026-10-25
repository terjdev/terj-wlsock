use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_content_type_v1.type`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="type">
///       <description summary="possible content types">
///         These values describe the available content types for a surface.
///       </description>
///       <entry name="none" value="0">
///         <description summary="no content type applies">
///           The content type none means that either the application has no data
///           about the content type, or that the content doesn't fit into one of
///           the other categories.
///         </description>
///       </entry>
///       <entry name="photo" value="1">
///         <description summary="photo content type">
///           The content type photo describes content derived from digital still
///           pictures and may be presented with minimal processing.
///         </description>
///       </entry>
///       <entry name="video" value="2">
///         <description summary="video content type">
///           The content type video describes a video or animation and may be
///           presented with more accurate timing to avoid stutter. Where scaling
///           is needed, scaling methods more appropriate for video may be used.
///         </description>
///       </entry>
///       <entry name="game" value="3">
///         <description summary="game content type">
///           The content type game describes a running game. Its content may be
///           presented with reduced latency.
///         </description>
///       </entry>
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpContentTypeV1Type(pub u32);
impl WpContentTypeV1Type {
    /// no content type applies; available since version 1.
    pub const NONE: Self = Self(0);
    /// photo content type; available since version 1.
    pub const PHOTO: Self = Self(1);
    /// video content type; available since version 1.
    pub const VIDEO: Self = Self(2);
    /// game content type; available since version 1.
    pub const GAME: Self = Self(3);
}
impl CommandReader<Self> for WpContentTypeV1Type {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpContentTypeV1Type {
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
