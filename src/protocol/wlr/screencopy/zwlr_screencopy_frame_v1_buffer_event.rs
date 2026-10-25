use crate::protocol::core::WlShmFormat;
use crate::protocol::wlr::screencopy::ObjectInteface as WlrScreencopyObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_screencopy_frame_v1.buffer`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="buffer">
///       <description summary="wl_shm buffer information">
///         Provides information about wl_shm buffer parameters that need to be
///         used for this frame. This event is sent once after the frame is created
///         if wl_shm buffers are supported.
///       </description>
///       <arg name="format" type="uint" enum="wl_shm.format" summary="buffer format" />
///       <arg name="width" type="uint" summary="buffer width" />
///       <arg name="height" type="uint" summary="buffer height" />
///       <arg name="stride" type="uint" summary="buffer stride" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrScreencopyFrameV1BufferEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// buffer format.
    pub format: WlShmFormat,
    /// buffer width.
    pub width: u32,
    /// buffer height.
    pub height: u32,
    /// buffer stride.
    pub stride: u32,
}
impl ZwlrScreencopyFrameV1BufferEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrScreencopyFrameV1BufferEvent {
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
impl InterfaceReader for ZwlrScreencopyFrameV1BufferEvent {
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
        let format = WlShmFormat(reader.next_u32().await?);
        let width = reader.next_u32().await?;
        let height = reader.next_u32().await?;
        let stride = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            format,
            width,
            height,
            stride,
        })
    }
}
impl CommandWriter for ZwlrScreencopyFrameV1BufferEvent {
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
        writer.next_u32(self.format.0).await?;
        writer.next_u32(self.width).await?;
        writer.next_u32(self.height).await?;
        writer.next_u32(self.stride).await?;
        Ok(())
    }
}
