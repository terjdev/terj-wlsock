use super::ObjectInteface as XxInputMethodObjectInteface;
use crate::protocol::ext::xx_text_input::XxTextInputV3ChangeCause;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xx_input_method_v1.text_change_cause`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="text_change_cause">
///       <description summary="indicates the cause of surrounding text change">
///         Tells the input method why the text surrounding the cursor changed.
///
///         Whenever the client detects an external change in text, cursor, or
///         anchor position, it must issue this request to the compositor. This
///         request is intended to give the input method a chance to update the
///         preedit text in an appropriate way, e.g. by removing it when the user
///         starts typing with a keyboard.
///
///         cause describes the source of the change.
///
///         The value set with this event is double-buffered. It will get applied
///         and set to its initial value on the next xx_input_method_v1.done
///         event.
///
///         The initial value of cause is input_method.
///       </description>
///       <arg name="cause" type="uint" enum="xx_text_input_v3.change_cause" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XxInputMethodV1TextChangeCauseEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `cause`.
    pub cause: XxTextInputV3ChangeCause,
}
impl XxInputMethodV1TextChangeCauseEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxInputMethodV1TextChangeCauseEvent {
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
impl InterfaceReader for XxInputMethodV1TextChangeCauseEvent {
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
        let cause = XxTextInputV3ChangeCause(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, cause })
    }
}
impl CommandWriter for XxInputMethodV1TextChangeCauseEvent {
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
            Self::writer_event_interface(state, self.object_id, "xx_input_method_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XxInputMethod(XxInputMethodObjectInteface::XxInputMethodV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.cause.0).await?;
        Ok(())
    }
}
