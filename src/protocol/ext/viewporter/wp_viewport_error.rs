use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Protocol error value `wp_viewport.error`.
///
/// Unknown numeric values are preserved. This value encodes four bytes.
/// ```xml
/// <enum name="error">
///       <entry name="bad_value" value="0" summary="negative or zero values in width or height" />
///       <entry name="bad_size" value="1" summary="destination size is not integer" />
///       <entry name="out_of_buffer" value="2" summary="source rectangle extends outside of the content area" />
///       <entry name="no_surface" value="3" summary="the wl_surface was destroyed" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpViewportError(pub u32);
impl WpViewportError {
    /// Negative or zero values in width or height.
    pub const BAD_VALUE: Self = Self(0);
    /// Destination size is not integer.
    pub const BAD_SIZE: Self = Self(1);
    /// Source rectangle extends outside of the content area.
    pub const OUT_OF_BUFFER: Self = Self(2);
    /// The wl_surface was destroyed.
    pub const NO_SURFACE: Self = Self(3);
}
impl CommandReader<Self> for WpViewportError {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpViewportError {
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
