use super::ObjectInteface as TextInputObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_text_input_v1.commit_string`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="commit_string">
///       <description summary="commit">
///     Notify when text should be inserted into the editor widget. The text to
///     commit could be either just a single character after a key press or the
///     result of some composing (pre-edit). It could also be an empty text
///     when some text should be removed (see delete_surrounding_text) or when
///     the input cursor should be moved (see cursor_position).
///
///     Any previously set composing text should be removed.
///       </description>
///       <arg name="serial" type="uint" summary="serial of the latest known text input state" />
///       <arg name="text" type="string" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTextInputV1CommitStringEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// serial of the latest known text input state.
    pub serial: u32,
    /// Wire argument `text`.
    pub text: String,
}
impl ZwpTextInputV1CommitStringEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 7;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTextInputV1CommitStringEvent {
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
impl InterfaceReader for ZwpTextInputV1CommitStringEvent {
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
        let serial = reader.next_u32().await?;
        let text = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            serial,
            text,
        })
    }
}
impl CommandWriter for ZwpTextInputV1CommitStringEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), FdWriter::encoded_string_size(Some(&self.text))])
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
        if !FdWriter::is_valid_string(&self.text) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        writer.next_string(&self.text).await?;
        Ok(())
    }
}
