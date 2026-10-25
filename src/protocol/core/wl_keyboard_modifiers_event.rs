use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_keyboard.modifiers`.
///
/// Opcode 4; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="modifiers">
///       <description summary="modifier and group state">
///         Notifies clients that the modifier and/or group state has
///         changed, and it should update its local state.
///
///         The compositor may send this event without a surface of the client
///         having keyboard focus, for example to tie modifier information to
///         pointer focus instead. If a modifier event with pressed modifiers is sent
///         without a prior enter event, the client can assume the modifier state is
///         valid until it receives the next wl_keyboard.modifiers event. In order to
///         reset the modifier state again, the compositor can send a
///         wl_keyboard.modifiers event with no pressed modifiers.
///
///         In the wl_keyboard logical state, this event updates the modifiers and
///         group.
///       </description>
///       <arg name="serial" type="uint" summary="serial number of the modifiers event" />
///       <arg name="mods_depressed" type="uint" summary="depressed modifiers" />
///       <arg name="mods_latched" type="uint" summary="latched modifiers" />
///       <arg name="mods_locked" type="uint" summary="locked modifiers" />
///       <arg name="group" type="uint" summary="keyboard layout" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlKeyboardModifiersEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `serial`: serial number of the modifiers event.
    pub serial: u32,
    /// Argument `mods_depressed`: depressed modifiers.
    pub mods_depressed: u32,
    /// Argument `mods_latched`: latched modifiers.
    pub mods_latched: u32,
    /// Argument `mods_locked`: locked modifiers.
    pub mods_locked: u32,
    /// Argument `group`: keyboard layout.
    pub group: u32,
}
impl WlKeyboardModifiersEvent {
    /// Opcode in the event namespace of interface wl_keyboard.
    pub const OPCODE: u16 = 4;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlKeyboardModifiersEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4), Some(4), Some(4)])
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
        writer.next_u32(self.mods_depressed).await?;
        writer.next_u32(self.mods_latched).await?;
        writer.next_u32(self.mods_locked).await?;
        writer.next_u32(self.group).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlKeyboardModifiersEvent {
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

impl InterfaceReader for WlKeyboardModifiersEvent {
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
        let mods_depressed = reader.next_u32().await?;
        let mods_latched = reader.next_u32().await?;
        let mods_locked = reader.next_u32().await?;
        let group = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            serial,
            mods_depressed,
            mods_latched,
            mods_locked,
            group,
        })
    }
}
