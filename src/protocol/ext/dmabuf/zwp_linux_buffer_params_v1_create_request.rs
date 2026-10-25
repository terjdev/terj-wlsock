use super::ZwpLinuxBufferParamsV1Flags;
use crate::protocol::ext::dmabuf::ObjectInteface as DmabufObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_linux_buffer_params_v1.create`.
///
/// Opcode 2; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="create">
///       <description summary="create a wl_buffer from the given dmabufs">
///         This asks for creation of a wl_buffer from the added dmabuf
///         buffers. The wl_buffer is not created immediately but returned via
///         the 'created' event if the dmabuf sharing succeeds. The sharing
///         may fail at runtime for reasons a client cannot predict, in
///         which case the 'failed' event is triggered.
///
///         The 'format' argument is a DRM_FORMAT code, as defined by the
///         libdrm's drm_fourcc.h. The Linux kernel's DRM sub-system is the
///         authoritative source on how the format codes should work.
///
///         The 'flags' is a bitfield of the flags defined in enum "flags".
///         'y_invert' means that the image needs to be y-flipped.
///
///         Flag 'interlaced' means that the frame in the buffer is not
///         progressive as usual, but interlaced. An interlaced buffer as
///         supported here must always contain both top and bottom fields.
///         The top field always begins on the first pixel row. The temporal
///         ordering between the two fields is top field first, unless
///         'bottom_first' is specified. It is undefined whether 'bottom_first'
///         is ignored if 'interlaced' is not set.
///
///         This protocol does not convey any information about field rate,
///         duration, or timing, other than the relative ordering between the
///         two fields in one buffer. A compositor may have to estimate the
///         intended field rate from the incoming buffer rate. It is undefined
///         whether the time of receiving wl_surface.commit with a new buffer
///         attached, applying the wl_surface state, wl_surface.frame callback
///         trigger, presentation, or any other point in the compositor cycle
///         is used to measure the frame or field times. There is no support
///         for detecting missed or late frames/fields/buffers either, and
///         there is no support whatsoever for cooperating with interlaced
///         compositor output.
///
///         The composited image quality resulting from the use of interlaced
///         buffers is explicitly undefined. A compositor may use elaborate
///         hardware features or software to deinterlace and create progressive
///         output frames from a sequence of interlaced input buffers, or it
///         may produce substandard image quality. However, compositors that
///         cannot guarantee reasonable image quality in all cases are recommended
///         to just reject all interlaced buffers.
///
///         Any argument errors, including non-positive width or height,
///         mismatch between the number of planes and the format, bad
///         format, bad offset or stride, may be indicated by fatal protocol
///         errors: INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS,
///         OUT_OF_BOUNDS.
///
///         Dmabuf import errors in the server that are not obvious client
///         bugs are returned via the 'failed' event as non-fatal. This
///         allows attempting dmabuf sharing and falling back in the client
///         if it fails.
///
///         This request can be sent only once in the object's lifetime, after
///         which the only legal request is destroy. This object should be
///         destroyed after issuing a 'create' request. Attempting to use this
///         object after issuing 'create' raises the ALREADY_USED protocol error.
///
///         It is not mandatory to issue 'create'. If a client wants to
///         cancel the buffer creation, it can just destroy this object.
///       </description>
///       <arg name="width" type="int" summary="base plane width in pixels" />
///       <arg name="height" type="int" summary="base plane height in pixels" />
///       <arg name="format" type="uint" summary="DRM_FORMAT code" />
///       <arg name="flags" type="uint" enum="flags" summary="see enum flags" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpLinuxBufferParamsV1CreateRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// base plane width in pixels.
    pub width: i32,
    /// base plane height in pixels.
    pub height: i32,
    /// DRM_FORMAT code.
    pub format: u32,
    /// see enum flags.
    pub flags: ZwpLinuxBufferParamsV1Flags,
}
impl ZwpLinuxBufferParamsV1CreateRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpLinuxBufferParamsV1CreateRequest {
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
impl InterfaceReader for ZwpLinuxBufferParamsV1CreateRequest {
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
        let width = reader.next_i32().await?;
        let height = reader.next_i32().await?;
        let format = reader.next_u32().await?;
        let flags = ZwpLinuxBufferParamsV1Flags(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            width,
            height,
            format,
            flags,
        })
    }
}
impl CommandWriter for ZwpLinuxBufferParamsV1CreateRequest {
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
        Self::writer_request_interface(
            state,
            self.object_id,
            "zwp_linux_buffer_params_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.width).await?;
        writer.next_i32(self.height).await?;
        writer.next_u32(self.format).await?;
        writer.next_u32(self.flags.0).await?;
        Ok(())
    }
}
