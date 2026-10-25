use super::ObjectInteface as XxInputMethodObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xx_input_method_v1.surrounding_text`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="surrounding_text">
///       <description summary="surrounding text event">
///         Updates the surrounding plain text around the cursor, excluding the
///         preedit text.
///
///         If any preedit text is present, it is replaced with the cursor for the
///         purpose of this event.
///
///         The argument text is a buffer containing the preedit string, and must
///         include the cursor position, and the complete selection. It should
///         contain additional characters before and after these. There is a
///         maximum length of wayland messages, so text can not be longer than 4000
///         bytes.
///
///         cursor is the byte offset of the cursor within the text buffer.
///
///         anchor is the byte offset of the selection anchor within the text
///         buffer. If there is no selected text, anchor must be the same as
///         cursor.
///
///         If this event does not arrive before the first done event, the input
///         method may assume that the text input does not support this
///         functionality and ignore following surrounding_text events.
///
///         Values set with this event are double-buffered. They will get applied
///         and set to initial values on the next xx_input_method_v1.done
///         event.
///
///         The initial state for affected fields is empty, meaning that the text
///         input does not support sending surrounding text. If the empty values
///         get applied, subsequent attempts to change them may have no effect.
///       </description>
///       <arg name="text" type="string" />
///       <arg name="cursor" type="uint" />
///       <arg name="anchor" type="uint" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XxInputMethodV1SurroundingTextEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `text`.
    pub text: String,
    /// Wire argument `cursor`.
    pub cursor: u32,
    /// Wire argument `anchor`.
    pub anchor: u32,
}
impl XxInputMethodV1SurroundingTextEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxInputMethodV1SurroundingTextEvent {
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
impl InterfaceReader for XxInputMethodV1SurroundingTextEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxInputMethod(XxInputMethodObjectInteface::XxInputMethodV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let text = reader.next_string().await?;
        let cursor = reader.next_u32().await?;
        let anchor = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            text,
            cursor,
            anchor,
        })
    }
}
impl CommandWriter for XxInputMethodV1SurroundingTextEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[
            FdWriter::encoded_string_size(Some(&self.text)),
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
            Self::writer_event_interface(state, self.object_id, "xx_input_method_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XxInputMethod(XxInputMethodObjectInteface::XxInputMethodV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.text) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.text).await?;
        writer.next_u32(self.cursor).await?;
        writer.next_u32(self.anchor).await?;
        Ok(())
    }
}
