use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_image_description_creator_icc_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <description summary="protocol errors" />
///
///       <entry name="incomplete_set" value="0" summary="incomplete parameter set" />
///       <entry name="already_set" value="1" summary="property already set" />
///       <entry name="bad_fd" value="2" summary="fd not seekable and readable" />
///       <entry name="bad_size" value="3" summary="no or too much data" />
///       <entry name="out_of_file" value="4" summary="offset + length exceeds file size" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpImageDescriptionCreatorIccV1Error(pub u32);
impl WpImageDescriptionCreatorIccV1Error {
    /// incomplete parameter set; available since version 1.
    pub const INCOMPLETE_SET: Self = Self(0);
    /// property already set; available since version 1.
    pub const ALREADY_SET: Self = Self(1);
    /// fd not seekable and readable; available since version 1.
    pub const BAD_FD: Self = Self(2);
    /// no or too much data; available since version 1.
    pub const BAD_SIZE: Self = Self(3);
    /// offset + length exceeds file size; available since version 1.
    pub const OUT_OF_FILE: Self = Self(4);
}
impl CommandReader<Self> for WpImageDescriptionCreatorIccV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpImageDescriptionCreatorIccV1Error {
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
