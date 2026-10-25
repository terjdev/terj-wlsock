use super::ObjectInteface as TextInputObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_text_input_v3.done`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="done">
///       <description summary="apply changes">
///         Instruct the application to apply changes to state requested by the
///         preedit_string, commit_string delete_surrounding_text, and action
///         events.
///
///         The state relating to these events is double-buffered, and each one
///         modifies the pending state. This event replaces the current state with
///         the pending state.
///
///         The application must proceed by evaluating the changes in the following
///         order:
///
///         1. Replace existing preedit string with the cursor.
///         2. Delete requested surrounding text.
///         3. Insert commit string with the cursor at its end.
///         4. Calculate surrounding text to send.
///         5. Insert new preedit text in cursor position.
///         6. Place cursor inside preedit text.
///         7. Perform the requested action.
///
///         The serial number reflects the last state of the zwp_text_input_v3
///         object known to the compositor. The value of the serial argument must
///         be equal to the number of commit requests already issued on that object.
///
///         When the client receives a done event with a serial different than the
///         number of past commit requests, it must proceed with evaluating and
///         applying the changes as normal, except it should not change the current
///         state of the zwp_text_input_v3 object. All pending state requests
///         (set_surrounding_text, set_content_type and set_cursor_rectangle) on
///         the zwp_text_input_v3 object should be sent and committed after
///         receiving a zwp_text_input_v3.done event with a matching serial.
///       </description>
///       <arg name="serial" type="uint" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTextInputV3DoneEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `serial`.
    pub serial: u32,
}
impl ZwpTextInputV3DoneEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 5;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTextInputV3DoneEvent {
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
impl InterfaceReader for ZwpTextInputV3DoneEvent {
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
        let serial = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, serial })
    }
}
impl CommandWriter for ZwpTextInputV3DoneEvent {
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
        let interface =
            Self::writer_event_interface(state, self.object_id, "zwp_text_input_v3", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::TextInput(TextInputObjectInteface::ZwpTextInputV3 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        Ok(())
    }
}
