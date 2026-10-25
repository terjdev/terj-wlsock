use super::ObjectInteface as XxHotkeyObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xx_hotkey_v1.released`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="released">
///       <description summary="the hotkey was released">
///         The key or button of a previously triggered hotkey was released,
///         regardless of modifier state. This enables press-and-hold uses such as
///         push-to-talk; clients that only act on the initial firing may ignore
///         it.
///
///         serial and time have the same meaning as in the "triggered" event.
///       </description>
///       <arg name="serial" type="uint" summary="input serial for the activation" />
///       <arg name="time" type="uint" summary="timestamp in milliseconds" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XxHotkeyV1ReleasedEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// input serial for the activation.
    pub serial: u32,
    /// timestamp in milliseconds.
    pub time: u32,
}
impl XxHotkeyV1ReleasedEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 4;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxHotkeyV1ReleasedEvent {
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
impl InterfaceReader for XxHotkeyV1ReleasedEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxHotkey(XxHotkeyObjectInteface::XxHotkeyV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let serial = reader.next_u32().await?;
        let time = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            serial,
            time,
        })
    }
}
impl CommandWriter for XxHotkeyV1ReleasedEvent {
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
            Self::writer_event_interface(state, self.object_id, "xx_hotkey_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XxHotkey(XxHotkeyObjectInteface::XxHotkeyV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        writer.next_u32(self.time).await?;
        Ok(())
    }
}
