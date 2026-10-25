use crate::protocol::ext::dmabuf::ObjectInteface as DmabufObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_linux_buffer_params_v1.add`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="add">
///       <description summary="add a dmabuf to the temporary set">
///         This request adds one dmabuf to the set in this
///         zwp_linux_buffer_params_v1.
///
///         The 64-bit unsigned value combined from modifier_hi and modifier_lo
///         is the dmabuf layout modifier. DRM AddFB2 ioctl calls this the
///         fb modifier, which is defined in drm_mode.h of Linux UAPI.
///         This is an opaque token. Drivers use this token to express tiling,
///         compression, etc. driver-specific modifications to the base format
///         defined by the DRM fourcc code.
///
///         Starting from version 4, the invalid_format protocol error is sent if
///         the format + modifier pair was not advertised as supported.
///
///         Starting from version 5, the invalid_format protocol error is sent if
///         all planes don't use the same modifier.
///
///         This request raises the PLANE_IDX error if plane_idx is too large.
///         The error PLANE_SET is raised if attempting to set a plane that
///         was already set.
///       </description>
///       <arg name="fd" type="fd" summary="dmabuf fd" />
///       <arg name="plane_idx" type="uint" summary="plane index" />
///       <arg name="offset" type="uint" summary="offset in bytes" />
///       <arg name="stride" type="uint" summary="stride in bytes" />
///       <arg name="modifier_hi" type="uint" summary="high 32 bits of layout modifier" />
///       <arg name="modifier_lo" type="uint" summary="low 32 bits of layout modifier" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpLinuxBufferParamsV1AddRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// dmabuf fd.
    pub fd: std::os::fd::OwnedFd,
    /// plane index.
    pub plane_idx: u32,
    /// offset in bytes.
    pub offset: u32,
    /// stride in bytes.
    pub stride: u32,
    /// high 32 bits of layout modifier.
    pub modifier_hi: u32,
    /// low 32 bits of layout modifier.
    pub modifier_lo: u32,
}
impl ZwpLinuxBufferParamsV1AddRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpLinuxBufferParamsV1AddRequest {
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
impl InterfaceReader for ZwpLinuxBufferParamsV1AddRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Dmabuf(DmabufObjectInteface::ZwpLinuxBufferParamsV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let plane_idx = reader.next_u32().await?;
        let offset = reader.next_u32().await?;
        let stride = reader.next_u32().await?;
        let modifier_hi = reader.next_u32().await?;
        let modifier_lo = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        let fd = reader.next_fd().ok_or(FdReaderError::InvalidProtocol)?;
        Ok(Self {
            object_id,
            fd,
            plane_idx,
            offset,
            stride,
            modifier_hi,
            modifier_lo,
        })
    }
}
impl CommandWriter for ZwpLinuxBufferParamsV1AddRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(0), Some(4), Some(4), Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(
            state,
            self.object_id,
            "zwp_linux_buffer_params_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_fd(self.fd).await?;
        writer.next_u32(self.plane_idx).await?;
        writer.next_u32(self.offset).await?;
        writer.next_u32(self.stride).await?;
        writer.next_u32(self.modifier_hi).await?;
        writer.next_u32(self.modifier_lo).await?;
        Ok(())
    }
}
