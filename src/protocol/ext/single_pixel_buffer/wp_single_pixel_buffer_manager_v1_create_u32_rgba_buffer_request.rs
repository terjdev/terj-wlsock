use crate::protocol::ext::single_pixel_buffer::ObjectInteface as SinglePixelBufferObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_single_pixel_buffer_manager_v1.create_u32_rgba_buffer`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="create_u32_rgba_buffer">
///       <description summary="create a 1×1 buffer from 32-bit RGBA values">
///         Create a single-pixel buffer from four 32-bit RGBA values.
///
///         Unless specified in another protocol extension, the RGBA values use
///         pre-multiplied alpha.
///
///         The width and height of the buffer are 1.
///
///         The r, g, b and a arguments valid range is from UINT32_MIN (0)
///         to UINT32_MAX (0xffffffff).
///
///         These arguments should be interpreted as a percentage, i.e.
///         - UINT32_MIN = 0% of the given color component
///         - UINT32_MAX = 100% of the given color component
///       </description>
///       <arg name="id" type="new_id" interface="wl_buffer" />
///       <arg name="r" type="uint" summary="value of the buffer's red channel" />
///       <arg name="g" type="uint" summary="value of the buffer's green channel" />
///       <arg name="b" type="uint" summary="value of the buffer's blue channel" />
///       <arg name="a" type="uint" summary="value of the buffer's alpha channel" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpSinglePixelBufferManagerV1CreateU32RgbaBufferRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `id`.
    pub id: u32,
    /// value of the buffer's red channel.
    pub r: u32,
    /// value of the buffer's green channel.
    pub g: u32,
    /// value of the buffer's blue channel.
    pub b: u32,
    /// value of the buffer's alpha channel.
    pub a: u32,
}
impl WpSinglePixelBufferManagerV1CreateU32RgbaBufferRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpSinglePixelBufferManagerV1CreateU32RgbaBufferRequest {
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
impl InterfaceReader for WpSinglePixelBufferManagerV1CreateU32RgbaBufferRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::SinglePixelBuffer(
                SinglePixelBufferObjectInteface::WpSinglePixelBufferManagerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = reader.next_u32().await?;
        let r = reader.next_u32().await?;
        let g = reader.next_u32().await?;
        let b = reader.next_u32().await?;
        let a = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("wl_buffer", 1).ok_or(FdReaderError::InvalidProtocol)?;
            Self::reader_state(state)?
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            r,
            g,
            b,
            a,
        })
    }
}
impl CommandWriter for WpSinglePixelBufferManagerV1CreateU32RgbaBufferRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(
            state,
            self.object_id,
            "wp_single_pixel_buffer_manager_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("wl_buffer", 1).ok_or(FdWriterError::InvalidProtocol)?;
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_u32(self.r).await?;
        writer.next_u32(self.g).await?;
        writer.next_u32(self.b).await?;
        writer.next_u32(self.a).await?;
        Ok(())
    }
}
