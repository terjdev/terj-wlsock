use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_image_description_v1.cause`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="cause">
///       <description summary="generic reason for failure" />
///
///       <entry name="low_version" value="0" summary="interface version too low" />
///       <entry name="unsupported" value="1" summary="unsupported image description data" />
///       <entry name="operating_system" value="2" summary="error independent of the client" />
///       <entry name="no_output" value="3" summary="the relevant output no longer exists" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpImageDescriptionV1Cause(pub u32);
impl WpImageDescriptionV1Cause {
    /// interface version too low; available since version 1.
    pub const LOW_VERSION: Self = Self(0);
    /// unsupported image description data; available since version 1.
    pub const UNSUPPORTED: Self = Self(1);
    /// error independent of the client; available since version 1.
    pub const OPERATING_SYSTEM: Self = Self(2);
    /// the relevant output no longer exists; available since version 1.
    pub const NO_OUTPUT: Self = Self(3);
}
impl CommandReader<Self> for WpImageDescriptionV1Cause {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpImageDescriptionV1Cause {
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
