use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwp_linux_buffer_params_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="already_used" value="0" summary="the zwp_linux_buffer_params_v1 object has already been used to create a wl_buffer" />
///       <entry name="plane_idx" value="1" summary="plane index out of bounds" />
///       <entry name="plane_set" value="2" summary="the plane index was already set" />
///       <entry name="incomplete" value="3" summary="missing or too many planes to create a buffer" />
///       <entry name="invalid_format" value="4" summary="format not supported" />
///       <entry name="invalid_dimensions" value="5" summary="invalid width or height" />
///       <entry name="out_of_bounds" value="6" summary="offset + stride * height goes out of dmabuf bounds" />
///       <entry name="invalid_wl_buffer" value="7" summary="invalid wl_buffer resulted from importing dmabufs via                the create_immed request on given buffer_params" />
///       <entry name="invalid_dev_t_size" value="8" summary="an array with mismatching size for a dev_t was used" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwpLinuxBufferParamsV1Error(pub u32);
impl ZwpLinuxBufferParamsV1Error {
    /// the zwp_linux_buffer_params_v1 object has already been used to create a wl_buffer; available since version 1.
    pub const ALREADY_USED: Self = Self(0);
    /// plane index out of bounds; available since version 1.
    pub const PLANE_IDX: Self = Self(1);
    /// the plane index was already set; available since version 1.
    pub const PLANE_SET: Self = Self(2);
    /// missing or too many planes to create a buffer; available since version 1.
    pub const INCOMPLETE: Self = Self(3);
    /// format not supported; available since version 1.
    pub const INVALID_FORMAT: Self = Self(4);
    /// invalid width or height; available since version 1.
    pub const INVALID_DIMENSIONS: Self = Self(5);
    /// offset + stride * height goes out of dmabuf bounds; available since version 1.
    pub const OUT_OF_BOUNDS: Self = Self(6);
    /// invalid wl_buffer resulted from importing dmabufs via                the create_immed request on given buffer_params; available since version 1.
    pub const INVALID_WL_BUFFER: Self = Self(7);
    /// an array with mismatching size for a dev_t was used; available since version 1.
    pub const INVALID_DEV_T_SIZE: Self = Self(8);
}
impl CommandReader<Self> for ZwpLinuxBufferParamsV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwpLinuxBufferParamsV1Error {
    fn object_id(&self) -> u32 {
        0
    }
    fn opcode(&self) -> u16 {
        0
    }
    fn payload_size(&self) -> usize {
        4
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        writer.next_u32(self.0).await
    }
}
