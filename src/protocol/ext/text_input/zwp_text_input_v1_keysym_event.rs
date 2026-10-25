use super::ObjectInteface as TextInputObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_text_input_v1.keysym`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="keysym">
///       <description summary="keysym">
///     Notify when a key event was sent. Key events should not be used
///     for normal text input operations, which should be done with
///     commit_string, delete_surrounding_text, etc. The key event follows
///     the wl_keyboard key event convention. Sym is an XKB keysym, state a
///     wl_keyboard key_state. Modifiers are a mask for effective modifiers
///     (where the modifier indices are set by the modifiers_map event)
///       </description>
///       <arg name="serial" type="uint" summary="serial of the latest known text input state" />
///       <arg name="time" type="uint" />
///       <arg name="sym" type="uint" />
///       <arg name="state" type="uint" />
///       <arg name="modifiers" type="uint" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTextInputV1KeysymEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// serial of the latest known text input state.
    pub serial: u32,
    /// Wire argument `time`.
    pub time: u32,
    /// Wire argument `sym`.
    pub sym: u32,
    /// Wire argument `state`.
    pub state: u32,
    /// Wire argument `modifiers`.
    pub modifiers: u32,
}
impl ZwpTextInputV1KeysymEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 10;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTextInputV1KeysymEvent {
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
impl InterfaceReader for ZwpTextInputV1KeysymEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::TextInput(TextInputObjectInteface::ZwpTextInputV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let serial = reader.next_u32().await?;
        let time = reader.next_u32().await?;
        let sym = reader.next_u32().await?;
        let state = reader.next_u32().await?;
        let modifiers = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            serial,
            time,
            sym,
            state,
            modifiers,
        })
    }
}
impl CommandWriter for ZwpTextInputV1KeysymEvent {
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
        let interface =
            Self::writer_event_interface(state, self.object_id, "zwp_text_input_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::TextInput(TextInputObjectInteface::ZwpTextInputV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        writer.next_u32(self.time).await?;
        writer.next_u32(self.sym).await?;
        writer.next_u32(self.state).await?;
        writer.next_u32(self.modifiers).await?;
        Ok(())
    }
}
