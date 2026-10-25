use super::ObjectInteface as XxInputMethodObjectInteface;
use crate::protocol::ext::xx_text_input::XxTextInputV3ContentHint;
use crate::protocol::ext::xx_text_input::XxTextInputV3ContentPurpose;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xx_input_method_v1.content_type`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="content_type">
///       <description summary="content purpose and hint">
///         Indicates the content type and hint for the current
///         xx_input_method_v1 instance.
///
///         Values set with this event are double-buffered. They will get applied
///         on the next xx_input_method_v1.done event.
///         They get reset to initial on the next committed deactivate event.
///
///         The initial value for hint is none, and the initial value for purpose
///         is normal.
///       </description>
///       <arg name="hint" type="uint" enum="xx_text_input_v3.content_hint" />
///       <arg name="purpose" type="uint" enum="xx_text_input_v3.content_purpose" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XxInputMethodV1ContentTypeEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `hint`.
    pub hint: XxTextInputV3ContentHint,
    /// Wire argument `purpose`.
    pub purpose: XxTextInputV3ContentPurpose,
}
impl XxInputMethodV1ContentTypeEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 4;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxInputMethodV1ContentTypeEvent {
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
impl InterfaceReader for XxInputMethodV1ContentTypeEvent {
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
        let hint = XxTextInputV3ContentHint(reader.next_u32().await?);
        let purpose = XxTextInputV3ContentPurpose(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            hint,
            purpose,
        })
    }
}
impl CommandWriter for XxInputMethodV1ContentTypeEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4)])
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
        writer.next_u32(self.hint.0).await?;
        writer.next_u32(self.purpose.0).await?;
        Ok(())
    }
}
