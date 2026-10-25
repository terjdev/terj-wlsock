use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xx_session_manager_v1.reason`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="reason">
///       <description summary="reason for getting a session">
///         The reason may determine in what way a session restores the window
///         management state of associated toplevels.
///
///         For example newly launched applications might be launched on the active
///         workspace with restored size and position, while a recovered
///         applications might restore additional state such as active workspace and
///         stacking order.
///       </description>
///       <entry name="launch" value="1">
///         <description summary="an app is newly launched">
///           A new app instance is launched, for example from an app launcher.
///         </description>
///       </entry>
///       <entry name="recover" value="2">
///         <description summary="an app recovered">
///           An app instance is recovering from for example a compositor or app crash.
///         </description>
///       </entry>
///       <entry name="session_restore" value="3">
///         <description summary="an app restored">
///           An app instance is restored, for example part of a restored session, or
///           restored from having been temporarily terminated due to resource
///           constraints.
///         </description>
///       </entry>
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XxSessionManagerV1Reason(pub u32);
impl XxSessionManagerV1Reason {
    /// an app is newly launched; available since version 1.
    pub const LAUNCH: Self = Self(1);
    /// an app recovered; available since version 1.
    pub const RECOVER: Self = Self(2);
    /// an app restored; available since version 1.
    pub const SESSION_RESTORE: Self = Self(3);
}
impl CommandReader<Self> for XxSessionManagerV1Reason {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XxSessionManagerV1Reason {
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
