use super::ObjectInteface as XxTextInputObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xx_text_input_v3.move_cursor`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="move_cursor" since="2">
///       <description summary="move cursor and change selection">
///         Unselects text, moves the cursor and selects text.
///
///         This is equivalent to dragging the mouse over some text: it deselects whatever might be currently selected and selects a new range of text.
///
///         The offsets used in arguments are in bytes relative to the current cursor position. Cursor is the new position of the cursor, and anchor is the opposite end of selection. If there's no selection, anchor should be equal to cursor.
///         In terms of dragging the mouse, the anchor is the start, and cursor the end.
///
///         The offsets do not take preedit contents into account, nor is preedit changed in any way with this request.
///
///         Both cursor and anchor must fall on code point boundaries, otherwise text input client may ignore the request. It is therefore not recommended for an input method to move any of them beyond the text received in surrounding_text.
///
///
///         When surrounding_text is not supported, the offsets must not be interpreted as bytes, but as some human-readable unit at least as big as a code point, for example a grapheme.
///
///         The cursor and anchor arguments can also take the following special values:
///         BEGINNING := 0x8000_0000 = i32::MIN
///         END := 0x7fff_ffff = i32::MAX
///         meaning, respectively, the beginning and the end of all text in the input field.
///
///         Values set with this event are double-buffered. They must be applied
///         and reset to initial on the next commit request.
///
///         The initial values of both cursor and anchor are 0.
///       </description>
///       <arg name="cursor" type="int" />
///       <arg name="anchor" type="int" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XxTextInputV3MoveCursorEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `cursor`.
    pub cursor: i32,
    /// Wire argument `anchor`.
    pub anchor: i32,
}
impl XxTextInputV3MoveCursorEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 5;
    /// Minimum interface version.
    pub const SINCE: u32 = 2;
}
impl CommandReader<Self> for XxTextInputV3MoveCursorEvent {
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
impl InterfaceReader for XxTextInputV3MoveCursorEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxTextInput(XxTextInputObjectInteface::XxTextInputV3 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let cursor = reader.next_i32().await?;
        let anchor = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            cursor,
            anchor,
        })
    }
}
impl CommandWriter for XxTextInputV3MoveCursorEvent {
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
            Self::writer_event_interface(state, self.object_id, "xx_text_input_v3", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XxTextInput(XxTextInputObjectInteface::XxTextInputV3 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.cursor).await?;
        writer.next_i32(self.anchor).await?;
        Ok(())
    }
}
