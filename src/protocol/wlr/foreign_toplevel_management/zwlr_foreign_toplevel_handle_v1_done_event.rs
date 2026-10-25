use super::ObjectInteface as WlrForeignToplevelManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_foreign_toplevel_handle_v1.done`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="done">
///       <description summary="all information about the toplevel has been sent">
///         This event is sent after all changes in the toplevel state have been
///         sent.
///
///         This allows changes to the zwlr_foreign_toplevel_handle_v1 properties
///         to be seen as atomic, even if they happen via multiple events.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrForeignToplevelHandleV1DoneEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl ZwlrForeignToplevelHandleV1DoneEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 5;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrForeignToplevelHandleV1DoneEvent {
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
impl InterfaceReader for ZwlrForeignToplevelHandleV1DoneEvent {
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
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id })
    }
}
impl CommandWriter for ZwlrForeignToplevelHandleV1DoneEvent {
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
        Ok(())
    }
}
