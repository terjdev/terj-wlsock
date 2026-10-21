use super::WlKeyboardKeyState;
use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_keyboard.key`.
///
/// Opcode 3; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="key">
///       <description summary="key event">
///         A key was pressed or released.
///         The time argument is a timestamp with millisecond
///         granularity, with an undefined base.
///
///         The key is a platform-specific key code that can be interpreted
///         by feeding it to the keyboard mapping (see the keymap event).
///
///         If this event produces a change in modifiers, then the resulting
///         wl_keyboard.modifiers event must be sent after this event.
///
///         In the wl_keyboard logical state, this event adds the key to the keys
///         currently logically down (if the state argument is pressed) or removes
///         the key from the keys currently logically down (if the state argument is
///         released). The compositor must not send this event if the wl_keyboard
///         did not have an active surface immediately before this event. The
///         compositor must not send this event if state is pressed (resp. released)
///         and the key was already logically down (resp. was not logically down)
///         immediately before this event.
///
///         Since version 10, compositors may send key events with the "repeated"
///         key state when a wl_keyboard.repeat_info event with a rate argument of
///         0 has been received. This allows the compositor to take over the
///         responsibility of key repetition.
///       </description>
///       <arg name="serial" type="uint" summary="serial number of the key event" />
///       <arg name="time" type="uint" summary="timestamp with millisecond granularity" />
///       <arg name="key" type="uint" summary="key that produced the event" />
///       <arg name="state" type="uint" enum="key_state" summary="physical state of the key" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlKeyboardKeyEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `serial`: serial number of the key event.
    pub serial: u32,
    /// Argument `time`: timestamp with millisecond granularity.
    pub time: u32,
    /// Argument `key`: key that produced the event.
    pub key: u32,
    /// Argument `state`: physical state of the key.
    pub state: WlKeyboardKeyState,
}
impl WlKeyboardKeyEvent {
    /// Opcode in the event namespace of interface wl_keyboard.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlKeyboardKeyEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::event_interface(state, self.object_id, "wl_keyboard", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        writer.next_u32(self.time).await?;
        writer.next_u32(self.key).await?;
        writer.next_u32(self.state.0).await?;
        Ok(())
    }
}
