use super::ObjectInteface as TextInputObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_text_input_v1.cursor_position`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="cursor_position">
///       <description summary="set cursor to new position">
///     Notify when the cursor or anchor position should be modified.
///
///     This event should be handled as part of a following commit_string
///     event.
///       </description>
///       <arg name="index" type="int" />
///       <arg name="anchor" type="int" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTextInputV1CursorPositionEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `index`.
    pub index: i32,
    /// Wire argument `anchor`.
    pub anchor: i32,
}
impl ZwpTextInputV1CursorPositionEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 8;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTextInputV1CursorPositionEvent {
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
impl InterfaceReader for ZwpTextInputV1CursorPositionEvent {
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
        let anchor = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            index,
            anchor,
        })
    }
}
impl CommandWriter for ZwpTextInputV1CursorPositionEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4)])
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
        writer.next_i32(self.anchor).await?;
        Ok(())
    }
}
