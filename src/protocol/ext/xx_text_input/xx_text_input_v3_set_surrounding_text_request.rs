use super::ObjectInteface as XxTextInputObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_text_input_v3.set_surrounding_text`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_surrounding_text">
///       <description summary="sets the surrounding text">
///         Sets the surrounding plain text around the input, excluding the preedit
///         text.
///
///         The client should notify the compositor of any changes in any of the
///         values carried with this request, including changes caused by handling
///         incoming text-input events as well as changes caused by other
///         mechanisms like keyboard typing.
///
///         If the client is unaware of the text around the cursor, it should not
///         issue this request, to signify lack of support to the compositor.
///
///         Text is UTF-8 encoded, and should include the cursor position, the
///         complete selection and additional characters before and after them.
///         There is a maximum length of wayland messages, so text can not be
///         longer than 4000 bytes.
///
///         Cursor is the byte offset of the cursor within text buffer.
///
///         Anchor is the byte offset of the selection anchor within text buffer.
///         If there is no selected text, anchor is the same as cursor.
///
///         If any preedit text is present, it is replaced with a cursor for the
///         purpose of this event.
///
///         Values set with this request are double-buffered. They will get applied
///         on the next xx_text_input_v3.commit request, and stay valid until the
///         next committed enable or disable request.
///
///         The initial state for affected fields is empty, meaning that the text
///         input does not support sending surrounding text. If the empty values
///         get applied, subsequent attempts to change them may have no effect.
///       </description>
///       <arg name="text" type="string" />
///       <arg name="cursor" type="int" />
///       <arg name="anchor" type="int" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxTextInputV3SetSurroundingTextRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `text`.
    pub text: String,
    /// Wire argument `cursor`.
    pub cursor: i32,
    /// Wire argument `anchor`.
    pub anchor: i32,
}
impl XxTextInputV3SetSurroundingTextRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxTextInputV3SetSurroundingTextRequest {
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
impl InterfaceReader for XxTextInputV3SetSurroundingTextRequest {
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
        let text = reader.next_string().await?;
        let cursor = reader.next_i32().await?;
        let anchor = reader.next_i32().await?;
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
impl CommandWriter for XxTextInputV3SetSurroundingTextRequest {
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
            Self::writer_request_interface(state, self.object_id, "xx_text_input_v3", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XxTextInput(XxTextInputObjectInteface::XxTextInputV3 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.text) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.text).await?;
        writer.next_i32(self.cursor).await?;
        writer.next_i32(self.anchor).await?;
        Ok(())
    }
}
