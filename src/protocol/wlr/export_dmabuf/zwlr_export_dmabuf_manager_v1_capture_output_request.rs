use super::ObjectInteface as WlrExportDmabufObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_export_dmabuf_manager_v1.capture_output`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="capture_output">
///       <description summary="capture a frame from an output">
///         Capture the next frame of an entire output.
///       </description>
///       <arg name="frame" type="new_id" interface="zwlr_export_dmabuf_frame_v1" />
///       <arg name="overlay_cursor" type="int" summary="include custom client hardware cursor on top of the frame" />
///       <arg name="output" type="object" interface="wl_output" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrExportDmabufManagerV1CaptureOutputRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `frame`.
    pub frame: u32,
    /// include custom client hardware cursor on top of the frame.
    pub overlay_cursor: i32,
    /// Wire argument `output`.
    pub output: u32,
}
impl ZwlrExportDmabufManagerV1CaptureOutputRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrExportDmabufManagerV1CaptureOutputRequest {
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
impl InterfaceReader for ZwlrExportDmabufManagerV1CaptureOutputRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrExportDmabuf(
                WlrExportDmabufObjectInteface::ZwlrExportDmabufManagerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let frame = reader.next_u32().await?;
        let overlay_cursor = reader.next_i32().await?;
        let output = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::WlrExportDmabuf(
                WlrExportDmabufObjectInteface::ZwlrExportDmabufFrameV1 {
                    version: interface.version().min(1),
                },
            );
            Self::reader_state(state)?
                .register_object(frame, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            frame,
            overlay_cursor,
            output,
        })
    }
}
impl CommandWriter for ZwlrExportDmabufManagerV1CaptureOutputRequest {
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
            "zwlr_export_dmabuf_manager_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::WlrExportDmabuf(
                WlrExportDmabufObjectInteface::ZwlrExportDmabufManagerV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::WlrExportDmabuf(
                WlrExportDmabufObjectInteface::ZwlrExportDmabufFrameV1 {
                    version: interface.version().min(1),
                },
            );
            Self::writer_state(state)?
                .register_object(self.frame, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.frame).await?;
        writer.next_i32(self.overlay_cursor).await?;
        writer.next_u32(self.output).await?;
        Ok(())
    }
}
