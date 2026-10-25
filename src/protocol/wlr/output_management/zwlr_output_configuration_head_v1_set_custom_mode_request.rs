use super::ObjectInteface as WlrOutputManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_output_configuration_head_v1.set_custom_mode`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_custom_mode">
///       <description summary="set a custom mode">
///         This request assigns a custom mode to the head. The size is given in
///         physical hardware units of the output device. If set to zero, the
///         refresh rate is unspecified.
///
///         It is a protocol error to set both a mode and a custom mode.
///       </description>
///       <arg name="width" type="int" summary="width of the mode in hardware units" />
///       <arg name="height" type="int" summary="height of the mode in hardware units" />
///       <arg name="refresh" type="int" summary="vertical refresh rate in mHz or zero" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrOutputConfigurationHeadV1SetCustomModeRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// width of the mode in hardware units.
    pub width: i32,
    /// height of the mode in hardware units.
    pub height: i32,
    /// vertical refresh rate in mHz or zero.
    pub refresh: i32,
}
impl ZwlrOutputConfigurationHeadV1SetCustomModeRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrOutputConfigurationHeadV1SetCustomModeRequest {
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
impl InterfaceReader for ZwlrOutputConfigurationHeadV1SetCustomModeRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputConfigurationHeadV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let width = reader.next_i32().await?;
        let height = reader.next_i32().await?;
        let refresh = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            width,
            height,
            refresh,
        })
    }
}
impl CommandWriter for ZwlrOutputConfigurationHeadV1SetCustomModeRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "zwlr_output_configuration_head_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputConfigurationHeadV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.width).await?;
        writer.next_i32(self.height).await?;
        writer.next_i32(self.refresh).await?;
        Ok(())
    }
}
