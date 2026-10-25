use super::ObjectInteface as WlrOutputPowerManagementObjectInteface;
use super::ZwlrOutputPowerV1Mode;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_output_power_v1.mode`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="mode">
///       <description summary="Report a power management mode change">
///         Report the power management mode change of an output.
///
///         The mode event is sent after an output changed its power
///         management mode. The reason can be a client using set_mode or the
///         compositor deciding to change an output's mode.
///         This event is also sent immediately when the object is created
///         so the client is informed about the current power management mode.
///       </description>
///       <arg name="mode" type="uint" enum="mode" summary="the output's new power management mode" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrOutputPowerV1ModeEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the output's new power management mode.
    pub mode: ZwlrOutputPowerV1Mode,
}
impl ZwlrOutputPowerV1ModeEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrOutputPowerV1ModeEvent {
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
impl InterfaceReader for ZwlrOutputPowerV1ModeEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrOutputPowerManagement(
                WlrOutputPowerManagementObjectInteface::ZwlrOutputPowerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let mode = ZwlrOutputPowerV1Mode(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, mode })
    }
}
impl CommandWriter for ZwlrOutputPowerV1ModeEvent {
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
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "zwlr_output_power_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::WlrOutputPowerManagement(
                WlrOutputPowerManagementObjectInteface::ZwlrOutputPowerV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.mode.0).await?;
        Ok(())
    }
}
