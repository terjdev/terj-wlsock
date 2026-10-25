use super::ObjectInteface as XxInputMethodObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_input_method_v1.commit_string`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="commit_string">
///       <description summary="commit string">
///         Send the commit string text for insertion to the application.
///
///         Inserts a string at current cursor position (see commit event
///         sequence). The string to commit could be either just a single character
///         after a key press or the result of some composing.
///
///         The argument text is a buffer containing the string to insert. There is
///         a maximum length of wayland messages, so text can not be longer than
///         4000 bytes.
///
///         Values set with this request are double-buffered. They must be applied
///         and reset to initial on the next .commit request.
///
///         The initial value of text is an empty string.
///       </description>
///       <arg name="text" type="string" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxInputMethodV1CommitStringRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `text`.
    pub text: String,
}
impl XxInputMethodV1CommitStringRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxInputMethodV1CommitStringRequest {
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
impl InterfaceReader for XxInputMethodV1CommitStringRequest {
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
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, text })
    }
}
impl CommandWriter for XxInputMethodV1CommitStringRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_string_size(Some(&self.text))])
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
        Ok(())
    }
}
