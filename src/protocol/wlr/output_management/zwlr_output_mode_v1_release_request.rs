use super::ObjectInteface as WlrOutputManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_output_mode_v1.release`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="release" type="destructor" since="3">
///       <description summary="destroy the mode object">
///         This request indicates that the client will no longer use this mode
///         object.
///       </description>
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrOutputModeV1ReleaseRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl ZwlrOutputModeV1ReleaseRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 3;
}
impl CommandReader<Self> for ZwlrOutputModeV1ReleaseRequest {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        let interface = Self::reader_state(state)?
            .object_interface(object_id)
            .ok_or(FdReaderError::InvalidProtocol)?;
        Self::read_with_interface(object_id, opcode, interface, reader, state).await
    }
}
impl InterfaceReader for ZwlrOutputModeV1ReleaseRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputModeV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Self::reader_state(state)?.release_server_id(object_id);
        Ok(Self { object_id })
    }
}
impl CommandWriter for ZwlrOutputModeV1ReleaseRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "zwlr_output_mode_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputModeV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        Self::writer_state(state)?.release_server_id(self.object_id);
        Ok(())
    }
}
