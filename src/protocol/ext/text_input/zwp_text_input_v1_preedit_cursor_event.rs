use super::ObjectInteface as TextInputObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_text_input_v1.preedit_cursor`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="preedit_cursor">
///       <description summary="pre-edit cursor">
///     Sets the cursor position inside the composing text (as byte
///     offset) relative to the start of the composing text. When index is a
///     negative number no cursor is shown.
///
///     This event is handled as part of a following preedit_string event.
///       </description>
///       <arg name="index" type="int" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTextInputV1PreeditCursorEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `index`.
    pub index: i32,
}
impl ZwpTextInputV1PreeditCursorEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 6;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTextInputV1PreeditCursorEvent {
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
impl InterfaceReader for ZwpTextInputV1PreeditCursorEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::TextInput(TextInputObjectInteface::ZwpTextInputV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let index = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, index })
    }
}
impl CommandWriter for ZwpTextInputV1PreeditCursorEvent {
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
        let interface =
            Self::writer_event_interface(state, self.object_id, "zwp_text_input_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::TextInput(TextInputObjectInteface::ZwpTextInputV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.index).await?;
        Ok(())
    }
}
