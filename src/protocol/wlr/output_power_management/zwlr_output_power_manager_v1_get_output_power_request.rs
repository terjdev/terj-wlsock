use super::ObjectInteface as WlrOutputPowerManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_output_power_manager_v1.get_output_power`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="get_output_power">
///       <description summary="get a power management for an output">
///         Create an output power management mode control that can be used to
///         adjust the power management mode for a given output.
///       </description>
///       <arg name="id" type="new_id" interface="zwlr_output_power_v1" />
///       <arg name="output" type="object" interface="wl_output" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrOutputPowerManagerV1GetOutputPowerRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `id`.
    pub id: u32,
    /// Wire argument `output`.
    pub output: u32,
}
impl ZwlrOutputPowerManagerV1GetOutputPowerRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrOutputPowerManagerV1GetOutputPowerRequest {
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
impl InterfaceReader for ZwlrOutputPowerManagerV1GetOutputPowerRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrOutputPowerManagement(
                WlrOutputPowerManagementObjectInteface::ZwlrOutputPowerManagerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = reader.next_u32().await?;
        let output = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::WlrOutputPowerManagement(
                WlrOutputPowerManagementObjectInteface::ZwlrOutputPowerV1 {
                    version: interface.version().min(1),
                },
            );
            Self::reader_state(state)?
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            output,
        })
    }
}
impl CommandWriter for ZwlrOutputPowerManagerV1GetOutputPowerRequest {
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
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "zwlr_output_power_manager_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::WlrOutputPowerManagement(
                WlrOutputPowerManagementObjectInteface::ZwlrOutputPowerManagerV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::WlrOutputPowerManagement(
                WlrOutputPowerManagementObjectInteface::ZwlrOutputPowerV1 {
                    version: interface.version().min(1),
                },
            );
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_u32(self.output).await?;
        Ok(())
    }
}
