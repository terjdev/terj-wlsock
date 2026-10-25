use super::ObjectInteface as XxInputMethodObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xx_input_method_v1.set_available_actions`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="set_available_actions" since="3">
///       <description summary="announce the available actions">
///         Announces the actions available for the currently active text input.
///
///         Values set with this event are double-buffered. They will get applied
///         on the next .done event.
///         They get reset to the initial value on the next committed deactivate event.
///
///         The initial value is an empty set: no actions are available.
///
///         Values in the available_actions array come from text-input-v3.action.
///       </description>
///       <arg name="available_actions" type="array" summary="available actions" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XxInputMethodV1SetAvailableActionsEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// available actions.
    pub available_actions: Vec<u8>,
}
impl XxInputMethodV1SetAvailableActionsEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 5;
    /// Minimum interface version.
    pub const SINCE: u32 = 3;
}
impl CommandReader<Self> for XxInputMethodV1SetAvailableActionsEvent {
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
impl InterfaceReader for XxInputMethodV1SetAvailableActionsEvent {
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
        let available_actions = reader.next_array().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            available_actions,
        })
    }
}
impl CommandWriter for XxInputMethodV1SetAvailableActionsEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_array_size(self.available_actions.len())])
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
        writer.next_array(&self.available_actions).await?;
        Ok(())
    }
}
