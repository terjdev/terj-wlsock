use super::ObjectInteface as WlrExportDmabufObjectInteface;
use super::ZwlrExportDmabufFrameV1Flags;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_export_dmabuf_frame_v1.frame`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="frame">
///       <description summary="a frame description">
///         Main event supplying the client with information about the frame. If the
///         capture didn't fail, this event is always emitted first before any other
///         events.
///
///         This event is followed by a number of "object" as specified by the
///         "num_objects" argument.
///       </description>
///       <arg name="width" type="uint" summary="frame width in pixels" />
///       <arg name="height" type="uint" summary="frame height in pixels" />
///       <arg name="offset_x" type="uint" summary="crop offset for the x axis" />
///       <arg name="offset_y" type="uint" summary="crop offset for the y axis" />
///       <arg name="buffer_flags" type="uint" summary="flags which indicate properties (invert, interlacing),                     has the same values as zwp_linux_buffer_params_v1:flags" />
///       <arg name="flags" type="uint" enum="flags" summary="indicates special frame features" />
///       <arg name="format" type="uint" summary="format of the frame (DRM_FORMAT_*)" />
///       <arg name="mod_high" type="uint" summary="drm format modifier, high" />
///       <arg name="mod_low" type="uint" summary="drm format modifier, low" />
///       <arg name="num_objects" type="uint" summary="indicates how many objects (FDs) the frame has (max 4)" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrExportDmabufFrameV1FrameEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// frame width in pixels.
    pub width: u32,
    /// frame height in pixels.
    pub height: u32,
    /// crop offset for the x axis.
    pub offset_x: u32,
    /// crop offset for the y axis.
    pub offset_y: u32,
    /// flags which indicate properties (invert, interlacing), has the same values as zwp_linux_buffer_params_v1:flags.
    pub buffer_flags: u32,
    /// indicates special frame features.
    pub flags: ZwlrExportDmabufFrameV1Flags,
    /// format of the frame (DRM_FORMAT_*).
    pub format: u32,
    /// drm format modifier, high.
    pub mod_high: u32,
    /// drm format modifier, low.
    pub mod_low: u32,
    /// indicates how many objects (FDs) the frame has (max 4).
    pub num_objects: u32,
}
impl ZwlrExportDmabufFrameV1FrameEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrExportDmabufFrameV1FrameEvent {
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
impl InterfaceReader for ZwlrExportDmabufFrameV1FrameEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrExportDmabuf(
                WlrExportDmabufObjectInteface::ZwlrExportDmabufFrameV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let width = reader.next_u32().await?;
        let height = reader.next_u32().await?;
        let offset_x = reader.next_u32().await?;
        let offset_y = reader.next_u32().await?;
        let buffer_flags = reader.next_u32().await?;
        let flags = ZwlrExportDmabufFrameV1Flags(reader.next_u32().await?);
        let format = reader.next_u32().await?;
        let mod_high = reader.next_u32().await?;
        let mod_low = reader.next_u32().await?;
        let num_objects = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            width,
            height,
            offset_x,
            offset_y,
            buffer_flags,
            flags,
            format,
            mod_high,
            mod_low,
            num_objects,
        })
    }
}
impl CommandWriter for ZwlrExportDmabufFrameV1FrameEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[
            Some(4),
            Some(4),
            Some(4),
            Some(4),
            Some(4),
            Some(4),
            Some(4),
            Some(4),
            Some(4),
            Some(4),
        ])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "zwlr_export_dmabuf_frame_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::WlrExportDmabuf(
                WlrExportDmabufObjectInteface::ZwlrExportDmabufFrameV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.width).await?;
        writer.next_u32(self.height).await?;
        writer.next_u32(self.offset_x).await?;
        writer.next_u32(self.offset_y).await?;
        writer.next_u32(self.buffer_flags).await?;
        writer.next_u32(self.flags.0).await?;
        writer.next_u32(self.format).await?;
        writer.next_u32(self.mod_high).await?;
        writer.next_u32(self.mod_low).await?;
        writer.next_u32(self.num_objects).await?;
        Ok(())
    }
}
