use super::ObjectInteface as XxInputMethodObjectInteface;
use super::XxInputMethodV1ProtocolCompat;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xx_input_method_v1.announce_protocol_compat`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="announce_protocol_compat" since="3">
///       <description summary="set text input's compatibility level">
///         Tells the input method client what kinds of events the text input client supports.
///
///
///
///         Values set with this event are double-buffered. They will get applied
///         on the next .done event.
///         They get reset to initial on the next committed deactivate event.
///
///         The compositor may send this event as part of a .done chain that switches the active state from inactive to active. It must not send this event otherwise.
///
///         The initial value for version is text_input_v3.
///       </description>
///       <arg name="compat_level" type="uint" enum="protocol_compat" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XxInputMethodV1AnnounceProtocolCompatEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `compat_level`.
    pub compat_level: XxInputMethodV1ProtocolCompat,
}
impl XxInputMethodV1AnnounceProtocolCompatEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 7;
    /// Minimum interface version.
    pub const SINCE: u32 = 3;
}
impl CommandReader<Self> for XxInputMethodV1AnnounceProtocolCompatEvent {
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
impl InterfaceReader for XxInputMethodV1AnnounceProtocolCompatEvent {
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
        let compat_level = XxInputMethodV1ProtocolCompat(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            compat_level,
        })
    }
}
impl CommandWriter for XxInputMethodV1AnnounceProtocolCompatEvent {
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
        writer.next_u32(self.compat_level.0).await?;
        Ok(())
    }
}
