use super::ObjectInteface as WlrOutputManagementObjectInteface;
use super::ZwlrOutputHeadV1AdaptiveSyncState;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_output_configuration_head_v1.set_adaptive_sync`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_adaptive_sync" since="4">
///       <description summary="enable/disable adaptive sync">
///         This request enables/disables adaptive sync. Adaptive sync is also
///         known as Variable Refresh Rate or VRR.
///       </description>
///       <arg name="state" type="uint" enum="zwlr_output_head_v1.adaptive_sync_state" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrOutputConfigurationHeadV1SetAdaptiveSyncRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `state`.
    pub state: ZwlrOutputHeadV1AdaptiveSyncState,
}
impl ZwlrOutputConfigurationHeadV1SetAdaptiveSyncRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 5;
    /// Minimum interface version.
    pub const SINCE: u32 = 4;
}
impl CommandReader<Self> for ZwlrOutputConfigurationHeadV1SetAdaptiveSyncRequest {
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
impl InterfaceReader for ZwlrOutputConfigurationHeadV1SetAdaptiveSyncRequest {
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
        let state = ZwlrOutputHeadV1AdaptiveSyncState(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, state })
    }
}
impl CommandWriter for ZwlrOutputConfigurationHeadV1SetAdaptiveSyncRequest {
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
        writer.next_u32(self.state.0).await?;
        Ok(())
    }
}
