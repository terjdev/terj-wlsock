use super::ObjectInteface as XxHotkeyObjectInteface;
use super::XxHotkeyV1DenyReason;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xx_hotkey_v1.denied`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="denied">
///       <description summary="the commit was rejected">
///         The committed trigger was not bound. Any previously established binding
///         remains active with its previous trigger.
///
///         message is an optional, advisory, human-readable explanation in the
///         compositor's locale, intended for verbatim display. It may be empty.
///         Clients must not parse it; all programmatic behavior keys off reason.
///       </description>
///       <arg name="reason" type="uint" enum="deny_reason" summary="why it was rejected" />
///       <arg name="message" type="string" summary="optional human-readable detail for display (may be empty)" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XxHotkeyV1DeniedEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// why it was rejected.
    pub reason: XxHotkeyV1DenyReason,
    /// optional human-readable detail for display (may be empty).
    pub message: String,
}
impl XxHotkeyV1DeniedEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxHotkeyV1DeniedEvent {
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
impl InterfaceReader for XxHotkeyV1DeniedEvent {
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
        let reason = XxHotkeyV1DenyReason(reader.next_u32().await?);
        let message = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            reason,
            message,
        })
    }
}
impl CommandWriter for XxHotkeyV1DeniedEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), FdWriter::encoded_string_size(Some(&self.message))])
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
        if !FdWriter::is_valid_string(&self.message) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.reason.0).await?;
        writer.next_string(&self.message).await?;
        Ok(())
    }
}
