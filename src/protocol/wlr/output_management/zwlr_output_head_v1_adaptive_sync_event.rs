use super::ObjectInteface as WlrOutputManagementObjectInteface;
use super::ZwlrOutputHeadV1AdaptiveSyncState;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_output_head_v1.adaptive_sync`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="adaptive_sync" since="4">
///       <description summary="current adaptive sync state">
///         This event describes whether adaptive sync is currently enabled for
///         the head or not. Adaptive sync is also known as Variable Refresh
///         Rate or VRR.
///       </description>
///       <arg name="state" type="uint" enum="adaptive_sync_state" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrOutputHeadV1AdaptiveSyncEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `state`.
    pub state: ZwlrOutputHeadV1AdaptiveSyncState,
}
impl ZwlrOutputHeadV1AdaptiveSyncEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 13;
    /// Minimum interface version.
    pub const SINCE: u32 = 4;
}
impl CommandReader<Self> for ZwlrOutputHeadV1AdaptiveSyncEvent {
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
impl InterfaceReader for ZwlrOutputHeadV1AdaptiveSyncEvent {
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
                WlrOutputManagementObjectInteface::ZwlrOutputHeadV1 { .. }
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
impl CommandWriter for ZwlrOutputHeadV1AdaptiveSyncEvent {
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
            "zwlr_output_head_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputHeadV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.state.0).await?;
        Ok(())
    }
}
