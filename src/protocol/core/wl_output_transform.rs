use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_output.transform`.
///
/// The numeric representation preserves unknown values without losing information.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="transform">
///       <description summary="transformation applied to buffer contents">
///         This describes transformations that clients and compositors apply to
///         buffer contents.
///
///         The flipped values correspond to an initial flip around a
///         vertical axis followed by rotation.
///
///         The purpose is mainly to allow clients to render accordingly and
///         tell the compositor, so that for fullscreen surfaces, the
///         compositor will still be able to scan out directly from client
///         surfaces.
///       </description>
///       <entry name="normal" value="0" summary="no transform" />
///       <entry name="90" value="1" summary="90 degrees counter-clockwise" />
///       <entry name="180" value="2" summary="180 degrees counter-clockwise" />
///       <entry name="270" value="3" summary="270 degrees counter-clockwise" />
///       <entry name="flipped" value="4" summary="180 degree flip around a vertical axis" />
///       <entry name="flipped_90" value="5" summary="flip and rotate 90 degrees counter-clockwise" />
///       <entry name="flipped_180" value="6" summary="flip and rotate 180 degrees counter-clockwise" />
///       <entry name="flipped_270" value="7" summary="flip and rotate 270 degrees counter-clockwise" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlOutputTransform(pub u32);
impl WlOutputTransform {
    /// no transform; XML since=1.
    /// ```xml
    /// <entry name="normal" value="0" summary="no transform" />
    /// ```
    pub const NORMAL: Self = Self(0);
    /// 90 degrees counter-clockwise; XML since=1.
    /// ```xml
    /// <entry name="90" value="1" summary="90 degrees counter-clockwise" />
    /// ```
    pub const ROTATE_90: Self = Self(1);
    /// 180 degrees counter-clockwise; XML since=1.
    /// ```xml
    /// <entry name="180" value="2" summary="180 degrees counter-clockwise" />
    /// ```
    pub const ROTATE_180: Self = Self(2);
    /// 270 degrees counter-clockwise; XML since=1.
    /// ```xml
    /// <entry name="270" value="3" summary="270 degrees counter-clockwise" />
    /// ```
    pub const ROTATE_270: Self = Self(3);
    /// 180 degree flip around a vertical axis; XML since=1.
    /// ```xml
    /// <entry name="flipped" value="4" summary="180 degree flip around a vertical axis" />
    /// ```
    pub const FLIPPED: Self = Self(4);
    /// flip and rotate 90 degrees counter-clockwise; XML since=1.
    /// ```xml
    /// <entry name="flipped_90" value="5" summary="flip and rotate 90 degrees counter-clockwise" />
    /// ```
    pub const FLIPPED_90: Self = Self(5);
    /// flip and rotate 180 degrees counter-clockwise; XML since=1.
    /// ```xml
    /// <entry name="flipped_180" value="6" summary="flip and rotate 180 degrees counter-clockwise" />
    /// ```
    pub const FLIPPED_180: Self = Self(6);
    /// flip and rotate 270 degrees counter-clockwise; XML since=1.
    /// ```xml
    /// <entry name="flipped_270" value="7" summary="flip and rotate 270 degrees counter-clockwise" />
    /// ```
    pub const FLIPPED_270: Self = Self(7);
}
impl CommandReader<Self> for WlOutputTransform {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlOutputTransform {
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
