use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `ext_session_lock_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="invalid_destroy" value="0" summary="attempted to destroy session lock while locked" />
///       <entry name="invalid_unlock" value="1" summary="unlock requested but locked event was never sent" />
///       <entry name="role" value="2" summary="given wl_surface already has a role" />
///       <entry name="duplicate_output" value="3" summary="given output already has a lock surface" />
///       <entry name="already_constructed" value="4" summary="given wl_surface has a buffer attached or committed" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExtSessionLockV1Error(pub u32);
impl ExtSessionLockV1Error {
    /// attempted to destroy session lock while locked; available since version 1.
    pub const INVALID_DESTROY: Self = Self(0);
    /// unlock requested but locked event was never sent; available since version 1.
    pub const INVALID_UNLOCK: Self = Self(1);
    /// given wl_surface already has a role; available since version 1.
    pub const ROLE: Self = Self(2);
    /// given output already has a lock surface; available since version 1.
    pub const DUPLICATE_OUTPUT: Self = Self(3);
    /// given wl_surface has a buffer attached or committed; available since version 1.
    pub const ALREADY_CONSTRUCTED: Self = Self(4);
}
impl CommandReader<Self> for ExtSessionLockV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ExtSessionLockV1Error {
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
