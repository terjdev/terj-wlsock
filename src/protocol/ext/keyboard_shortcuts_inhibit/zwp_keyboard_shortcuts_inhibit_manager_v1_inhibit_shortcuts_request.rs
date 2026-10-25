use super::ObjectInteface as KeyboardShortcutsInhibitObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_keyboard_shortcuts_inhibit_manager_v1.inhibit_shortcuts`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="inhibit_shortcuts">
///       <description summary="create a new keyboard shortcuts inhibitor object">
///     Create a new keyboard shortcuts inhibitor object associated with
///     the given surface for the given seat.
///
///     If shortcuts are already inhibited for the specified seat and surface,
///     a protocol error "already_inhibited" is raised by the compositor.
///       </description>
///       <arg name="id" type="new_id" interface="zwp_keyboard_shortcuts_inhibitor_v1" />
///       <arg name="surface" type="object" interface="wl_surface" summary="the surface that inhibits the keyboard shortcuts behavior" />
///       <arg name="seat" type="object" interface="wl_seat" summary="the wl_seat for which keyboard shortcuts should be disabled" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpKeyboardShortcutsInhibitManagerV1InhibitShortcutsRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `id`.
    pub id: u32,
    /// the surface that inhibits the keyboard shortcuts behavior.
    pub surface: u32,
    /// the wl_seat for which keyboard shortcuts should be disabled.
    pub seat: u32,
}
impl ZwpKeyboardShortcutsInhibitManagerV1InhibitShortcutsRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpKeyboardShortcutsInhibitManagerV1InhibitShortcutsRequest {
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
impl InterfaceReader for ZwpKeyboardShortcutsInhibitManagerV1InhibitShortcutsRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::KeyboardShortcutsInhibit(
                KeyboardShortcutsInhibitObjectInteface::ZwpKeyboardShortcutsInhibitManagerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = reader.next_u32().await?;
        let surface = reader.next_u32().await?;
        let seat = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::KeyboardShortcutsInhibit(
                KeyboardShortcutsInhibitObjectInteface::ZwpKeyboardShortcutsInhibitorV1 {
                    version: interface.version().min(1),
                },
            );
            Self::reader_state(state)?
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            surface,
            seat,
        })
    }
}
impl CommandWriter for ZwpKeyboardShortcutsInhibitManagerV1InhibitShortcutsRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "zwp_keyboard_shortcuts_inhibit_manager_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::KeyboardShortcutsInhibit(
                KeyboardShortcutsInhibitObjectInteface::ZwpKeyboardShortcutsInhibitManagerV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::KeyboardShortcutsInhibit(
                KeyboardShortcutsInhibitObjectInteface::ZwpKeyboardShortcutsInhibitorV1 {
                    version: interface.version().min(1),
                },
            );
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_u32(self.surface).await?;
        writer.next_u32(self.seat).await?;
        Ok(())
    }
}
