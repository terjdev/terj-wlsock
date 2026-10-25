use super::ObjectInteface as WlrForeignToplevelManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_foreign_toplevel_handle_v1.output_leave`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="output_leave">
///       <description summary="toplevel left an output">
///         This event is emitted whenever the toplevel stops being visible on
///         the given output. It is guaranteed that an entered-output event
///         with the same output has been emitted before this event.
///       </description>
///       <arg name="output" type="object" interface="wl_output" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrForeignToplevelHandleV1OutputLeaveEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `output`.
    pub output: u32,
}
impl ZwlrForeignToplevelHandleV1OutputLeaveEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrForeignToplevelHandleV1OutputLeaveEvent {
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
impl InterfaceReader for ZwlrForeignToplevelHandleV1OutputLeaveEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrForeignToplevelManagement(
                WlrForeignToplevelManagementObjectInteface::ZwlrForeignToplevelHandleV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let output = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, output })
    }
}
impl CommandWriter for ZwlrForeignToplevelHandleV1OutputLeaveEvent {
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
            "zwlr_foreign_toplevel_handle_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::WlrForeignToplevelManagement(
                WlrForeignToplevelManagementObjectInteface::ZwlrForeignToplevelHandleV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.output).await?;
        Ok(())
    }
}
