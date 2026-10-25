use super::ObjectInteface as TextInputObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_text_input_v3.preedit_string`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="preedit_string">
///       <description summary="pre-edit">
///         Notify when a new composing text (pre-edit) should be set at the
///         current cursor position. Any previously set composing text must be
///         removed. Any previously existing selected text must be removed.
///
///         The argument text contains the pre-edit string buffer.
///
///         The parameters cursor_begin and cursor_end are counted in bytes
///         relative to the beginning of the submitted text buffer. Cursor should
///         be hidden when both are equal to -1.
///
///         They could be represented by the client as a line if both values are
///         the same, or as a text highlight otherwise.
///
///         Values set with this event are double-buffered. They must be applied
///         and reset to initial on the next zwp_text_input_v3.done event.
///
///         The initial value of text is an empty string, and cursor_begin,
///         cursor_end and cursor_hidden are all 0.
///       </description>
///       <arg name="text" type="string" allow-null="true" />
///       <arg name="cursor_begin" type="int" />
///       <arg name="cursor_end" type="int" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTextInputV3PreeditStringEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `text`.
    pub text: Option<String>,
    /// Wire argument `cursor_begin`.
    pub cursor_begin: i32,
    /// Wire argument `cursor_end`.
    pub cursor_end: i32,
}
impl ZwpTextInputV3PreeditStringEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTextInputV3PreeditStringEvent {
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
impl InterfaceReader for ZwpTextInputV3PreeditStringEvent {
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
        let text = reader.next_nullable_string().await?;
        let cursor_begin = reader.next_i32().await?;
        let cursor_end = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            text,
            cursor_begin,
            cursor_end,
        })
    }
}
impl CommandWriter for ZwpTextInputV3PreeditStringEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[
            FdWriter::encoded_string_size(self.text.as_deref()),
            Some(4),
            Some(4),
        ])
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
        if self
            .text
            .as_deref()
            .is_some_and(|v| !FdWriter::is_valid_string(v))
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_nullable_string(self.text.as_deref()).await?;
        writer.next_i32(self.cursor_begin).await?;
        writer.next_i32(self.cursor_end).await?;
        Ok(())
    }
}
