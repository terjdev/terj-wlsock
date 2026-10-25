use super::ObjectInteface as XxInputMethodObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_input_method_v1.set_preedit_string`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_preedit_string">
///       <description summary="pre-edit string">
///         Send the pre-edit string text to the application text input.
///
///         Place a new composing text (pre-edit) at the current cursor position.
///         Any previously set composing text must be removed. Any previously
///         existing selected text must be removed. The cursor is moved to a new
///         position within the preedit string.
///
///         The argument text is a buffer containing the preedit string. There is
///         a maximum length of wayland messages, so text can not be longer than
///         4000 bytes.
///
///         The arguments cursor_begin and cursor_end are counted in bytes relative
///         to the beginning of the submitted string buffer. Cursor should be
///         hidden by the text input when both are equal to -1.
///
///         cursor_begin indicates the beginning of the cursor. cursor_end
///         indicates the end of the cursor. It may be equal or different than
///         cursor_begin.
///
///         Values set with this request are double-buffered. They must be applied on
///         the next xx_input_method_v1.commit request.
///         They must be reset to initial on the next committed .deactivate event.
///
///         The initial value of text is an empty string. The initial value of
///         cursor_begin, and cursor_end are both 0.
///       </description>
///       <arg name="text" type="string" />
///       <arg name="cursor_begin" type="int" />
///       <arg name="cursor_end" type="int" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxInputMethodV1SetPreeditStringRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `text`.
    pub text: String,
    /// Wire argument `cursor_begin`.
    pub cursor_begin: i32,
    /// Wire argument `cursor_end`.
    pub cursor_end: i32,
}
impl XxInputMethodV1SetPreeditStringRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxInputMethodV1SetPreeditStringRequest {
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
impl InterfaceReader for XxInputMethodV1SetPreeditStringRequest {
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
impl CommandWriter for XxInputMethodV1SetPreeditStringRequest {
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
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "xx_input_method_v1",
            Self::SINCE,
        )?;
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
        writer.next_i32(self.cursor_begin).await?;
        writer.next_i32(self.cursor_end).await?;
        Ok(())
    }
}
