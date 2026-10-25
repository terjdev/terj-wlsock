use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xdg_wm_base.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="role" value="0" summary="given wl_surface has another role" />
///       <entry name="defunct_surfaces" value="1" summary="xdg_wm_base was destroyed before children" />
///       <entry name="not_the_topmost_popup" value="2" summary="the client tried to map or destroy a non-topmost popup" />
///       <entry name="invalid_popup_parent" value="3" summary="the client specified an invalid popup parent surface" />
///       <entry name="invalid_surface_state" value="4" summary="the client provided an invalid surface state" />
///       <entry name="invalid_positioner" value="5" summary="the client provided an invalid positioner" />
///       <entry name="unresponsive" value="6" summary="the client didn’t respond to a ping event in time" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XdgWmBaseError(pub u32);
impl XdgWmBaseError {
    /// given wl_surface has another role; available since version 1.
    pub const ROLE: Self = Self(0);
    /// xdg_wm_base was destroyed before children; available since version 1.
    pub const DEFUNCT_SURFACES: Self = Self(1);
    /// the client tried to map or destroy a non-topmost popup; available since version 1.
    pub const NOT_THE_TOPMOST_POPUP: Self = Self(2);
    /// the client specified an invalid popup parent surface; available since version 1.
    pub const INVALID_POPUP_PARENT: Self = Self(3);
    /// the client provided an invalid surface state; available since version 1.
    pub const INVALID_SURFACE_STATE: Self = Self(4);
    /// the client provided an invalid positioner; available since version 1.
    pub const INVALID_POSITIONER: Self = Self(5);
    /// the client didn’t respond to a ping event in time; available since version 1.
    pub const UNRESPONSIVE: Self = Self(6);
}
impl CommandReader<Self> for XdgWmBaseError {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XdgWmBaseError {
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
