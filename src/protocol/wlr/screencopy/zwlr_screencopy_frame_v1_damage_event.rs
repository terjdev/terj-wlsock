use crate::protocol::wlr::screencopy::ObjectInteface as WlrScreencopyObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_screencopy_frame_v1.damage`.
///
/// Opcode 4; available since version 2.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="damage" since="2">
///       <description summary="carries the coordinates of the damaged region">
///         This event is sent right before the ready event when copy_with_damage is
///         requested. It may be generated multiple times for each copy_with_damage
///         request.
///
///         The arguments describe a box around an area that has changed since the
///         last copy request that was derived from the current screencopy manager
///         instance.
///
///         The union of all regions received between the call to copy_with_damage
///         and a ready event is the total damage since the prior ready event.
///       </description>
///       <arg name="x" type="uint" summary="damaged x coordinates" />
///       <arg name="y" type="uint" summary="damaged y coordinates" />
///       <arg name="width" type="uint" summary="current width" />
///       <arg name="height" type="uint" summary="current height" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrScreencopyFrameV1DamageEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// damaged x coordinates.
    pub x: u32,
    /// damaged y coordinates.
    pub y: u32,
    /// current width.
    pub width: u32,
    /// current height.
    pub height: u32,
}
impl ZwlrScreencopyFrameV1DamageEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 4;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 2;
}
impl CommandReader<Self> for ZwlrScreencopyFrameV1DamageEvent {
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
impl InterfaceReader for ZwlrScreencopyFrameV1DamageEvent {
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
        let x = reader.next_u32().await?;
        let y = reader.next_u32().await?;
        let width = reader.next_u32().await?;
        let height = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            x,
            y,
            width,
            height,
        })
    }
}
impl CommandWriter for ZwlrScreencopyFrameV1DamageEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4), Some(4)])
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
        writer.next_u32(self.x).await?;
        writer.next_u32(self.y).await?;
        writer.next_u32(self.width).await?;
        writer.next_u32(self.height).await?;
        Ok(())
    }
}
