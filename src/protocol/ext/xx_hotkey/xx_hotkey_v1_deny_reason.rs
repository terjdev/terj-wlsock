use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xx_hotkey_v1.deny_reason`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="deny_reason">
///       <description summary="why a commit was rejected">
///         A reason code describing why the commit got denied. The object remains
///         valid, any previously established binding is unaffected, and the client
///         may describe a different trigger and commit again.
///
///         A compositor is not obliged to distinguish "already_bound" from
///         "not_permitted" and may report the latter for both.
///       </description>
///       <entry name="already_bound" value="0" summary="held by another hotkey the compositor treats as exclusive" />
///       <entry name="not_permitted" value="1" summary="the trigger is disallowed by compositor policy" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XxHotkeyV1DenyReason(pub u32);
impl XxHotkeyV1DenyReason {
    /// held by another hotkey the compositor treats as exclusive; available since version 1.
    pub const ALREADY_BOUND: Self = Self(0);
    /// the trigger is disallowed by compositor policy; available since version 1.
    pub const NOT_PERMITTED: Self = Self(1);
}
impl CommandReader<Self> for XxHotkeyV1DenyReason {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XxHotkeyV1DenyReason {
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
