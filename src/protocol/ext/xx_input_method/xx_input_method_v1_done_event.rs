use super::ObjectInteface as XxInputMethodObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xx_input_method_v1.done`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="done">
///       <description summary="apply state">
///         Atomically applies state changes recently sent to the client.
///
///         The done event establishes and updates the state of the client, and
///         must be issued after any changes to apply them.
///
///         Text input state (content purpose, content hint, surrounding text, and
///         change cause) is conceptually double-buffered within an input method
///         context.
///
///         Events modify the pending state, as opposed to the current state in use
///         by the input method. A done event atomically applies all pending state,
///         replacing the current state. After done, the new pending state is as
///         documented for each related request.
///
///         Events must be applied in the order of arrival.
///
///         Neither current nor pending state are modified unless noted otherwise.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct XxInputMethodV1DoneEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl XxInputMethodV1DoneEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 8;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxInputMethodV1DoneEvent {
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
impl InterfaceReader for XxInputMethodV1DoneEvent {
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
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id })
    }
}
impl CommandWriter for XxInputMethodV1DoneEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[])
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
        Ok(())
    }
}
