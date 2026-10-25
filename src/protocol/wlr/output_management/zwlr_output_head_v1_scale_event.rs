use super::ObjectInteface as WlrOutputManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_output_head_v1.scale`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="scale">
///       <description summary="current scale">
///         This events describes the scale of the head in the global compositor
///         space. It is only sent if the output is enabled.
///       </description>
///       <arg name="scale" type="fixed" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrOutputHeadV1ScaleEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `scale`.
    pub scale: f64,
}
impl ZwlrOutputHeadV1ScaleEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 8;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrOutputHeadV1ScaleEvent {
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
impl InterfaceReader for ZwlrOutputHeadV1ScaleEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputHeadV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let scale = reader.next_fixed().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, scale })
    }
}
impl CommandWriter for ZwlrOutputHeadV1ScaleEvent {
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
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "zwlr_output_head_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputHeadV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.scale) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_fixed(self.scale).await?;
        Ok(())
    }
}
