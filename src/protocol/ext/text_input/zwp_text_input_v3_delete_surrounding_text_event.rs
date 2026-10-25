use super::ObjectInteface as TextInputObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_text_input_v3.delete_surrounding_text`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="delete_surrounding_text">
///       <description summary="delete surrounding text">
///         Notify when the text around the current cursor position should be
///         deleted.
///
///         Before_length and after_length are the number of bytes before and after
///         the current cursor index (excluding the selection) to delete.
///
///         If a preedit text is present, in effect before_length is counted from
///         the beginning of it, and after_length from its end (see done event
///         sequence).
///
///         Values set with this event are double-buffered. They must be applied
///         and reset to initial on the next zwp_text_input_v3.done event.
///
///         The initial values of both before_length and after_length are 0.
///       </description>
///       <arg name="before_length" type="uint" summary="length of text before current cursor position" />
///       <arg name="after_length" type="uint" summary="length of text after current cursor position" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTextInputV3DeleteSurroundingTextEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// length of text before current cursor position.
    pub before_length: u32,
    /// length of text after current cursor position.
    pub after_length: u32,
}
impl ZwpTextInputV3DeleteSurroundingTextEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 4;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTextInputV3DeleteSurroundingTextEvent {
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
impl InterfaceReader for ZwpTextInputV3DeleteSurroundingTextEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::TextInput(TextInputObjectInteface::ZwpTextInputV3 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let before_length = reader.next_u32().await?;
        let after_length = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            before_length,
            after_length,
        })
    }
}
impl CommandWriter for ZwpTextInputV3DeleteSurroundingTextEvent {
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
            Self::writer_event_interface(state, self.object_id, "zwp_text_input_v3", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::TextInput(TextInputObjectInteface::ZwpTextInputV3 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.before_length).await?;
        writer.next_u32(self.after_length).await?;
        Ok(())
    }
}
