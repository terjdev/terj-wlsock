use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_keyboard.repeat_info`.
///
/// Opcode 5; available since version 4. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="repeat_info" since="4">
///       <description summary="repeat rate and delay">
///         Informs the client about the keyboard's repeat rate and delay.
///
///         This event is sent as soon as the wl_keyboard object has been created,
///         and is guaranteed to be received by the client before any key press
///         event.
///
///         Negative values for either rate or delay are illegal. A rate of zero
///         will disable any repeating (regardless of the value of delay).
///
///         This event can be sent later on as well with a new value if necessary,
///         so clients should continue listening for the event past the creation
///         of wl_keyboard.
///       </description>
///       <arg name="rate" type="int" summary="the rate of repeating keys in characters per second" />
///       <arg name="delay" type="int" summary="delay in milliseconds since key down until repeating starts" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlKeyboardRepeatInfoEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `rate`: the rate of repeating keys in characters per second.
    pub rate: i32,
    /// Argument `delay`: delay in milliseconds since key down until repeating starts.
    pub delay: i32,
}
impl WlKeyboardRepeatInfoEvent {
    /// Opcode in the event namespace of interface wl_keyboard.
    pub const OPCODE: u16 = 5;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 4;
}
impl CommandWriter for WlKeyboardRepeatInfoEvent {
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
        Self::writer_event_interface(state, self.object_id, "wl_keyboard", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.rate).await?;
        writer.next_i32(self.delay).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlKeyboardRepeatInfoEvent {
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

impl InterfaceReader for WlKeyboardRepeatInfoEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(interface, ObjectInteface::WlKeyboard { .. })
            || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let rate = reader.next_i32().await?;
        let delay = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            rate,
            delay,
        })
    }
}
