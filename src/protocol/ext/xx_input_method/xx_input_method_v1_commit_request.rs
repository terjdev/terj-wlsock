use super::ObjectInteface as XxInputMethodObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_input_method_v1.commit`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="commit">
///       <description summary="apply state">
///         Apply state changes from commit_string, set_preedit_string and
///         delete_surrounding_text requests.
///
///         The state relating to these events is double-buffered, and each one
///         modifies the pending state. This request replaces the current state
///         with the pending state.
///
///         The connected text input is expected to proceed by evaluating the
///         changes in the following order:
///
///         1. Replace existing preedit string with the cursor.
///         2. Delete requested surrounding text.
///         3. Insert commit string with the cursor at its end.
///         4. Move the cursor and selection.
///         5. Calculate surrounding text to send.
///         6. Insert new preedit text in cursor position.
///         7. Place cursor inside preedit text.
///         8. Perform the requested action.
///
///         Note that the input method can not receive more than 4000 bytes of selection text, which might be the case for example when the entire document is selected. Nevertheless, the text input must delete the entire selected range before inserting the commit string.
///
///         Serial handling with protocol_compat == xx_text_input
///
///         The serial number should be set to 0.
///
///         Serial handling with protocol_compat == text_input_v3
///
///         The serial number reflects the last state of the xx_input_method_v1
///         object known to the client. The value of the serial argument must be
///         equal to the number of done events already issued by that object. When
///         the compositor receives a commit request with a serial different than
///         the number of past done events, it must proceed as normal, except it
///         should not change the current state of the xx_input_method_v1 object.
///       </description>
///       <arg name="serial" type="uint" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxInputMethodV1CommitRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `serial`.
    pub serial: u32,
}
impl XxInputMethodV1CommitRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 5;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxInputMethodV1CommitRequest {
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
impl InterfaceReader for XxInputMethodV1CommitRequest {
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
        let serial = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, serial })
    }
}
impl CommandWriter for XxInputMethodV1CommitRequest {
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
        writer.next_u32(self.serial).await?;
        Ok(())
    }
}
