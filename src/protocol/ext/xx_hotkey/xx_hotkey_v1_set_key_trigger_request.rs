use super::ObjectInteface as XxHotkeyObjectInteface;
use super::XxHotkeyManagerV1Modifiers;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_hotkey_v1.set_key_trigger`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_key_trigger">
///       <description summary="trigger on a key">
///         Describe the trigger as a key pressed while the given modifiers are
///         held. Replaces any trigger previously described. Takes effect on the
///         next commit.
///
///         The keysym uses the numbering shared with the keymap delivered over
///         wl_keyboard (for the xkb_v1 keymap format, an XKB_KEY_* value), in its
///         unshifted form (XKB_KEY_b, not XKB_KEY_B). The compositor matches it
///         against the keysym each key produces at its first shift level in any of
///         the user's layout groups, ignoring the effect Shift has on the key, so
///         that Shift is handled through the modifier mask like every other
///         modifier and a binding keeps firing after a layout group switch. A
///         binding whose keysym no group of the current keymap produces stays
///         bound and simply does not fire until the keymap changes again.
///
///         The trigger fires on key press; the compositor does not wait to see
///         whether another key follows. The press and the matching release are
///         consumed and must not be delivered to the focused surface.
///
///         The keysym may be one that contributes to modifier state, such as
///         XKB_KEY_Super_L; whether it does is decided by the seat's keymap. Such a
///         trigger is a tap: it fires when the key is released, and only if no
///         other key, button or axis event occurred while it was held. A
///         compositor may additionally require that it was held no longer than
///         some interval. The modifier mask is evaluated as the state before the
///         key was pressed, so a lone left Super key is (XKB_KEY_Super_L, 0) and
///         (XKB_KEY_Shift_L, ctrl) is a tap on Shift while Control is held.
///
///         A tap is never consumed: the key must still update modifier state and
///         reach the focused surface, so binding it does not stop it composing
///         other shortcuts. "triggered" and "released" are sent together on
///         release, so a tap cannot drive press-and-hold.
///
///         A null keysym raises the "invalid_trigger" error.
///       </description>
///       <arg name="keysym" type="uint" summary="keysym of the trigger key" />
///       <arg name="modifiers" type="uint" enum="xx_hotkey_manager_v1.modifiers" summary="bitmask of required modifiers" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxHotkeyV1SetKeyTriggerRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// keysym of the trigger key.
    pub keysym: u32,
    /// bitmask of required modifiers.
    pub modifiers: XxHotkeyManagerV1Modifiers,
}
impl XxHotkeyV1SetKeyTriggerRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxHotkeyV1SetKeyTriggerRequest {
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
impl InterfaceReader for XxHotkeyV1SetKeyTriggerRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxHotkey(XxHotkeyObjectInteface::XxHotkeyV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let keysym = reader.next_u32().await?;
        let modifiers = XxHotkeyManagerV1Modifiers(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            keysym,
            modifiers,
        })
    }
}
impl CommandWriter for XxHotkeyV1SetKeyTriggerRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface =
            Self::writer_request_interface(state, self.object_id, "xx_hotkey_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XxHotkey(XxHotkeyObjectInteface::XxHotkeyV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.keysym).await?;
        writer.next_u32(self.modifiers.0).await?;
        Ok(())
    }
}
