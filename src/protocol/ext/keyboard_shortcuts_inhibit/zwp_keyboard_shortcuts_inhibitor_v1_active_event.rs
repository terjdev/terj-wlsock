use super::ObjectInteface as KeyboardShortcutsInhibitObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_keyboard_shortcuts_inhibitor_v1.active`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="active">
///       <description summary="shortcuts are inhibited">
///     This event indicates that the shortcut inhibitor is active.
///
///     The compositor sends this event every time compositor shortcuts
///     are inhibited on behalf of the surface. When active, the client
///     may receive input events normally reserved by the compositor
///     (see zwp_keyboard_shortcuts_inhibitor_v1).
///
///     This occurs typically when the initial request "inhibit_shortcuts"
///     first becomes active or when the user instructs the compositor to
///     re-enable and existing shortcuts inhibitor using any mechanism
///     offered by the compositor.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpKeyboardShortcutsInhibitorV1ActiveEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl ZwpKeyboardShortcutsInhibitorV1ActiveEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpKeyboardShortcutsInhibitorV1ActiveEvent {
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
impl InterfaceReader for ZwpKeyboardShortcutsInhibitorV1ActiveEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::KeyboardShortcutsInhibit(
                KeyboardShortcutsInhibitObjectInteface::ZwpKeyboardShortcutsInhibitorV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id })
    }
}
impl CommandWriter for ZwpKeyboardShortcutsInhibitorV1ActiveEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "zwp_keyboard_shortcuts_inhibitor_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::KeyboardShortcutsInhibit(
                KeyboardShortcutsInhibitObjectInteface::ZwpKeyboardShortcutsInhibitorV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        Ok(())
    }
}
