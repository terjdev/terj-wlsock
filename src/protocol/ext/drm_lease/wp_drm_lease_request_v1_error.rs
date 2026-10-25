use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_drm_lease_request_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="wrong_device" value="0" summary="requested a connector from a different lease device" />
///       <entry name="duplicate_connector" value="1" summary="requested a connector twice" />
///       <entry name="empty_lease" value="2" summary="requested a lease without requesting a connector" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpDrmLeaseRequestV1Error(pub u32);
impl WpDrmLeaseRequestV1Error {
    /// requested a connector from a different lease device; available since version 1.
    pub const WRONG_DEVICE: Self = Self(0);
    /// requested a connector twice; available since version 1.
    pub const DUPLICATE_CONNECTOR: Self = Self(1);
    /// requested a lease without requesting a connector; available since version 1.
    pub const EMPTY_LEASE: Self = Self(2);
}
impl CommandReader<Self> for WpDrmLeaseRequestV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpDrmLeaseRequestV1Error {
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
