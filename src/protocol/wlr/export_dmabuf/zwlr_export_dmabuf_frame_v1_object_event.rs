use super::ObjectInteface as WlrExportDmabufObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_export_dmabuf_frame_v1.object`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="object">
///       <description summary="an object description">
///         Event which serves to supply the client with the file descriptors
///         containing the data for each object.
///
///         After receiving this event, the client must always close the file
///         descriptor as soon as they're done with it and even if the frame fails.
///       </description>
///       <arg name="index" type="uint" summary="index of the current object" />
///       <arg name="fd" type="fd" summary="fd of the current object" />
///       <arg name="size" type="uint" summary="size in bytes for the current object" />
///       <arg name="offset" type="uint" summary="starting point for the data in the object's fd" />
///       <arg name="stride" type="uint" summary="line size in bytes" />
///       <arg name="plane_index" type="uint" summary="index of the plane the data in the object applies to" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrExportDmabufFrameV1ObjectEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// index of the current object.
    pub index: u32,
    /// fd of the current object.
    pub fd: std::os::fd::OwnedFd,
    /// size in bytes for the current object.
    pub size: u32,
    /// starting point for the data in the object's fd.
    pub offset: u32,
    /// line size in bytes.
    pub stride: u32,
    /// index of the plane the data in the object applies to.
    pub plane_index: u32,
}
impl ZwlrExportDmabufFrameV1ObjectEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrExportDmabufFrameV1ObjectEvent {
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
impl InterfaceReader for ZwlrExportDmabufFrameV1ObjectEvent {
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
        let index = reader.next_u32().await?;
        let size = reader.next_u32().await?;
        let offset = reader.next_u32().await?;
        let stride = reader.next_u32().await?;
        let plane_index = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        let fd = reader.next_fd().ok_or(FdReaderError::InvalidProtocol)?;
        Ok(Self {
            object_id,
            index,
            fd,
            size,
            offset,
            stride,
            plane_index,
        })
    }
}
impl CommandWriter for ZwlrExportDmabufFrameV1ObjectEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(0), Some(4), Some(4), Some(4), Some(4)])
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
        writer.next_u32(self.index).await?;
        writer.next_fd(self.fd).await?;
        writer.next_u32(self.size).await?;
        writer.next_u32(self.offset).await?;
        writer.next_u32(self.stride).await?;
        writer.next_u32(self.plane_index).await?;
        Ok(())
    }
}
