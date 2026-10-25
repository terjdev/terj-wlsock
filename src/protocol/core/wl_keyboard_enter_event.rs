use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_keyboard.enter`.
///
/// Opcode 1; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="enter">
///       <description summary="enter event">
///         Notification that this seat's keyboard focus is on a certain
///         surface.
///
///         The compositor must send the wl_keyboard.modifiers event after this
///         event.
///
///         In the wl_keyboard logical state, this event sets the active surface to
///         the surface argument and the keys currently logically down to the keys
///         in the keys argument. The compositor must not send this event if the
///         wl_keyboard already had an active surface immediately before this event.
///
///         Clients should not use the list of pressed keys to emulate key-press
///         events. The order of keys in the list is unspecified.
///       </description>
///       <arg name="serial" type="uint" summary="serial number of the enter event" />
///       <arg name="surface" type="object" interface="wl_surface" summary="surface gaining keyboard focus" />
///       <arg name="keys" type="array" summary="the keys currently logically down" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlKeyboardEnterEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `serial`: serial number of the enter event.
    pub serial: u32,
    /// Argument `surface`: surface gaining keyboard focus.
    /// Object implementing interface wl_surface.
    pub surface: u32,
    /// Argument `keys`: the keys currently logically down.
    pub keys: Vec<u8>,
}
impl WlKeyboardEnterEvent {
    /// Opcode in the event namespace of interface wl_keyboard.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlKeyboardEnterEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[
            Some(4),
            Some(4),
            FdWriter::encoded_array_size(self.keys.len()),
        ])
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
        writer.next_u32(self.serial).await?;
        writer.next_u32(self.surface).await?;
        writer.next_array(&self.keys).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlKeyboardEnterEvent {
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

impl InterfaceReader for WlKeyboardEnterEvent {
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
        let serial = reader.next_u32().await?;
        let surface = reader.next_u32().await?;
        let keys = reader.next_array().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            serial,
            surface,
            keys,
        })
    }
}
