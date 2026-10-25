use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xx_cutouts_v1.type`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="type">
///       <description summary="Cutout type">
///         These values indicate the type of cutout. The information is
///         meant to help clients to decide whether they can possibly
///         ignore the element.
///       </description>
///
///       <entry name="cutout" value="0">
///         <description summary="A generic cutout">
///           This element type can be used by the compositor if it
///           doesn't want to provide a more specific type.
///         </description>
///       </entry>
///       <entry name="notch" value="1">
///         <description summary="Small functional cutout area">
///           A functional, irregular shape on one of the device's
///           edges. It often contains a camera.
///         </description>
///       </entry>
///       <entry name="waterfall" value="2">
///         <description summary="A curved display edge">
///           A curved display edge intended to make the device appear
///           like not having any bezel.
///         </description>
///       </entry>
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XxCutoutsV1Type(pub u32);
impl XxCutoutsV1Type {
    /// A generic cutout; available since version 1.
    pub const CUTOUT: Self = Self(0);
    /// Small functional cutout area; available since version 1.
    pub const NOTCH: Self = Self(1);
    /// A curved display edge; available since version 1.
    pub const WATERFALL: Self = Self(2);
}
impl CommandReader<Self> for XxCutoutsV1Type {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XxCutoutsV1Type {
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
