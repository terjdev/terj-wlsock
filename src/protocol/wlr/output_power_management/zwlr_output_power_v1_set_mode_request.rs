use super::ObjectInteface as WlrOutputPowerManagementObjectInteface;
use super::ZwlrOutputPowerV1Mode;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_output_power_v1.set_mode`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_mode">
///       <description summary="Set an outputs power save mode">
///         Set an output's power save mode to the given mode. The mode change
///         is effective immediately. If the output does not support the given
///         mode a failed event is sent.
///       </description>
///       <arg name="mode" type="uint" enum="mode" summary="the power save mode to set" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrOutputPowerV1SetModeRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the power save mode to set.
    pub mode: ZwlrOutputPowerV1Mode,
}
impl ZwlrOutputPowerV1SetModeRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrOutputPowerV1SetModeRequest {
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
impl InterfaceReader for ZwlrOutputPowerV1SetModeRequest {
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
impl CommandWriter for ZwlrOutputPowerV1SetModeRequest {
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
