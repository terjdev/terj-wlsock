use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_tearing_control_v1.presentation_hint`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="presentation_hint">
///       <description summary="presentation hint values">
///         This enum provides information for if submitted frames from the client
///         may be presented with tearing.
///       </description>
///       <entry name="vsync" value="0">
///         <description summary="tearing-free presentation">
///           The content of this surface is meant to be synchronized to the
///           vertical blanking period. This should not result in visible tearing
///           and may result in a delay before a surface commit is presented.
///         </description>
///       </entry>
///       <entry name="async" value="1">
///         <description summary="asynchronous presentation">
///           The content of this surface is meant to be presented with minimal
///           latency and tearing is acceptable.
///         </description>
///       </entry>
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpTearingControlV1PresentationHint(pub u32);
impl WpTearingControlV1PresentationHint {
    /// tearing-free presentation; available since version 1.
    pub const VSYNC: Self = Self(0);
    /// asynchronous presentation; available since version 1.
    pub const ASYNC: Self = Self(1);
}
impl CommandReader<Self> for WpTearingControlV1PresentationHint {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpTearingControlV1PresentationHint {
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
