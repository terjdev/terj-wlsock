use super::ObjectInteface as WlrOutputManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_output_head_v1.mode`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <event name="mode">
///       <description summary="introduce a mode">
///         This event introduces a mode for this head. It is sent once per
///         supported mode.
///       </description>
///       <arg name="mode" type="new_id" interface="zwlr_output_mode_v1" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrOutputHeadV1ModeEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `mode`.
    pub mode: u32,
}
impl ZwlrOutputHeadV1ModeEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrOutputHeadV1ModeEvent {
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
impl InterfaceReader for ZwlrOutputHeadV1ModeEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
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
        let mode = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputModeV1 {
                    version: interface.version().min(3),
                },
            );
            Self::reader_state(state)?
                .register_object(mode, new_interface, true)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self { object_id, mode })
    }
}
impl CommandWriter for ZwlrOutputHeadV1ModeEvent {
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
        {
            let new_interface = ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputModeV1 {
                    version: interface.version().min(3),
                },
            );
            Self::writer_state(state)?
                .register_object(self.mode, new_interface, true)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.mode).await?;
        Ok(())
    }
}
