use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_color_representation_surface_v1.chroma_location`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="chroma_location">
///       <description summary="Chroma sample location for 4:2:0 YCbCr">
///         Chroma sample location as defined by H.273 Chroma420SampleLocType.
///
///         A value of 0 is invalid and will never be present in the list of enums.
///
///         The descriptions list the matching Vulkan VkChromaLocation combinations
///         for convenience.
///       </description>
///       <entry name="type_0" value="1">
///         <description summary="Horizontal offset of 0, vertical offset of 0.5">
///           Corresponding to VkChromaLocations:
///           - xChromaOffset: VK_CHROMA_LOCATION_COSITED_EVEN
///           - yChromaOffset: VK_CHROMA_LOCATION_MIDPOINT
///
///           Equivalent to H.273 Chroma420SampleLocType 0.
///         </description>
///       </entry>
///       <entry name="type_1" value="2">
///         <description summary="Horizontal offset of 0.5, vertical offset of 0.5">
///           Corresponding to VkChromaLocations:
///           - xChromaOffset: VK_CHROMA_LOCATION_MIDPOINT
///           - yChromaOffset: VK_CHROMA_LOCATION_MIDPOINT
///
///           Equivalent to H.273 Chroma420SampleLocType 1.
///         </description>
///       </entry>
///       <entry name="type_2" value="3">
///         <description summary="Horizontal offset of 0, vertical offset of 0">
///           Corresponding to VkChromaLocations:
///           - xChromaOffset: VK_CHROMA_LOCATION_COSITED_EVEN
///           - yChromaOffset: VK_CHROMA_LOCATION_COSITED_EVEN
///
///           Equivalent to H.273 Chroma420SampleLocType 2.
///         </description>
///       </entry>
///       <entry name="type_3" value="4">
///         <description summary="Horizontal offset of 0.5, vertical offset of 0">
///           Corresponding to VkChromaLocations:
///           - xChromaOffset: VK_CHROMA_LOCATION_MIDPOINT
///           - yChromaOffset: VK_CHROMA_LOCATION_COSITED_EVEN
///
///           Equivalent to H.273 Chroma420SampleLocType 3.
///         </description>
///       </entry>
///       <entry name="type_4" value="5">
///         <description summary="Horizontal offset of 0, vertical offset of 1">
///           Equivalent to H.273 Chroma420SampleLocType 4.
///         </description>
///       </entry>
///       <entry name="type_5" value="6">
///         <description summary="Horizontal offset of 0.5, vertical offset of 1">
///           Equivalent to H.273 Chroma420SampleLocType 5.
///         </description>
///       </entry>
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpColorRepresentationSurfaceV1ChromaLocation(pub u32);
impl WpColorRepresentationSurfaceV1ChromaLocation {
    /// Horizontal offset of 0, vertical offset of 0.5; available since version 1.
    pub const TYPE_0: Self = Self(1);
    /// Horizontal offset of 0.5, vertical offset of 0.5; available since version 1.
    pub const TYPE_1: Self = Self(2);
    /// Horizontal offset of 0, vertical offset of 0; available since version 1.
    pub const TYPE_2: Self = Self(3);
    /// Horizontal offset of 0.5, vertical offset of 0; available since version 1.
    pub const TYPE_3: Self = Self(4);
    /// Horizontal offset of 0, vertical offset of 1; available since version 1.
    pub const TYPE_4: Self = Self(5);
    /// Horizontal offset of 0.5, vertical offset of 1; available since version 1.
    pub const TYPE_5: Self = Self(6);
}
impl CommandReader<Self> for WpColorRepresentationSurfaceV1ChromaLocation {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpColorRepresentationSurfaceV1ChromaLocation {
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
