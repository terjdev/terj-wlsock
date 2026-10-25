use crate::protocol::wlr::screencopy::ObjectInteface as WlrScreencopyObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_screencopy_frame_v1.copy_with_damage`.
///
/// Opcode 2; available since version 2.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="copy_with_damage" since="2">
///       <description summary="copy the frame when it's damaged">
///         Same as copy, except it waits until there is damage to copy.
///       </description>
///       <arg name="buffer" type="object" interface="wl_buffer" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrScreencopyFrameV1CopyWithDamageRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `buffer`.
    pub buffer: u32,
}
impl ZwlrScreencopyFrameV1CopyWithDamageRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 2;
}
impl CommandReader<Self> for ZwlrScreencopyFrameV1CopyWithDamageRequest {
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
impl InterfaceReader for ZwlrScreencopyFrameV1CopyWithDamageRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrScreencopy(
                WlrScreencopyObjectInteface::ZwlrScreencopyFrameV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let buffer = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, buffer })
    }
}
impl CommandWriter for ZwlrScreencopyFrameV1CopyWithDamageRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(
            state,
            self.object_id,
            "zwlr_screencopy_frame_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.buffer).await?;
        Ok(())
    }
}
