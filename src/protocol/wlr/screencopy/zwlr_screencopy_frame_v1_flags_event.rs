use super::ZwlrScreencopyFrameV1Flags;
use crate::protocol::wlr::screencopy::ObjectInteface as WlrScreencopyObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_screencopy_frame_v1.flags`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="flags">
///       <description summary="frame flags">
///         Provides flags about the frame. This event is sent once before the
///         "ready" event.
///       </description>
///       <arg name="flags" type="uint" enum="flags" summary="frame flags" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrScreencopyFrameV1FlagsEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// frame flags.
    pub flags: ZwlrScreencopyFrameV1Flags,
}
impl ZwlrScreencopyFrameV1FlagsEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrScreencopyFrameV1FlagsEvent {
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
impl InterfaceReader for ZwlrScreencopyFrameV1FlagsEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrScreencopy(
                WlrScreencopyObjectInteface::ZwlrScreencopyFrameV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let flags = ZwlrScreencopyFrameV1Flags(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, flags })
    }
}
impl CommandWriter for ZwlrScreencopyFrameV1FlagsEvent {
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
        Self::writer_event_interface(
            state,
            self.object_id,
            "zwlr_screencopy_frame_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.flags.0).await?;
        Ok(())
    }
}
