use super::ObjectInteface as XxInputMethodObjectInteface;
use crate::protocol::ext::xx_text_input::XxTextInputV3Action;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_input_method_v1.perform_action`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="perform_action" since="3">
///       <description summary="perform action">
///         Perform an action on this text input.
///
///         Values set with this event are double-buffered. They must be applied
///         and reset to initial on the next commit request.
///
///         The initial value of action is none.
///       </description>
///       <arg name="action" type="uint" enum="xx_text_input_v3.action" summary="action to perform" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxInputMethodV1PerformActionRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// action to perform.
    pub action: XxTextInputV3Action,
}
impl XxInputMethodV1PerformActionRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 3;
}
impl CommandReader<Self> for XxInputMethodV1PerformActionRequest {
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
impl InterfaceReader for XxInputMethodV1PerformActionRequest {
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
        let action = XxTextInputV3Action(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, action })
    }
}
impl CommandWriter for XxInputMethodV1PerformActionRequest {
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
        writer.next_u32(self.action.0).await?;
        Ok(())
    }
}
