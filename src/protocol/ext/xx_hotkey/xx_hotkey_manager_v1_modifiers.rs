use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xx_hotkey_manager_v1.modifiers`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="modifiers" bitfield="true">
///       <description summary="modifier keys required by a trigger">
///         A bitmask of the modifiers that must be held for the trigger to fire.
///         The match is exact: a trigger does not fire while a modifier outside
///         its mask is held, so Super+Space does not fire on Super+Shift+Space.
///         Bits outside this enum raise the "invalid_trigger" error.
///
///         These are fixed, keymap-independent semantic bits. A compositor matches
///         each bit against the corresponding modifier in its keyboard state as the
///         keymap defines it; for the xkb_v1 keymap format that is shift -&gt;
///         XKB_MOD_NAME_SHIFT, ctrl -&gt; XKB_MOD_NAME_CTRL, alt -&gt; XKB_MOD_NAME_ALT
///         ("Mod1"), super -&gt; XKB_MOD_NAME_LOGO ("Mod4").
///
///         A client needs neither a wl_keyboard nor a keymap to describe a trigger.
///
///         Lock modifiers (Caps Lock, Num Lock, Scroll Lock) are never part of a
///         binding and are ignored when matching.
///       </description>
///       <entry name="shift" value="1" summary="the Shift modifier" />
///       <entry name="ctrl" value="2" summary="the Control modifier" />
///       <entry name="alt" value="4" summary="the Alt modifier" />
///       <entry name="super" value="8" summary="the Super/Logo modifier" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XxHotkeyManagerV1Modifiers(pub u32);
impl XxHotkeyManagerV1Modifiers {
    /// the Shift modifier; available since version 1.
    pub const SHIFT: Self = Self(1);
    /// the Control modifier; available since version 1.
    pub const CTRL: Self = Self(2);
    /// the Alt modifier; available since version 1.
    pub const ALT: Self = Self(4);
    /// the Super/Logo modifier; available since version 1.
    pub const SUPER: Self = Self(8);
}
impl std::ops::BitOr for XxHotkeyManagerV1Modifiers {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl std::ops::BitAnd for XxHotkeyManagerV1Modifiers {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl CommandReader<Self> for XxHotkeyManagerV1Modifiers {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XxHotkeyManagerV1Modifiers {
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
