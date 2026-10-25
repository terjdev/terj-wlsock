use crate::protocol::ext::foreign_toplevel_list::ObjectInteface as ForeignToplevelListObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `ext_foreign_toplevel_list_v1.toplevel`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <event name="toplevel">
///       <description summary="a toplevel has been created">
///         This event is emitted whenever a new toplevel window is created. It is
///         emitted for all toplevels, regardless of the app that has created them.
///
///         All initial properties of the toplevel (identifier, title, app_id) will be sent
///         immediately after this event using the corresponding events for
///         ext_foreign_toplevel_handle_v1. The compositor will use the
///         ext_foreign_toplevel_handle_v1.done event to indicate when all data has
///         been sent.
///       </description>
///       <arg name="toplevel" type="new_id" interface="ext_foreign_toplevel_handle_v1" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ExtForeignToplevelListV1ToplevelEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `toplevel`.
    pub toplevel: u32,
}
impl ExtForeignToplevelListV1ToplevelEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ExtForeignToplevelListV1ToplevelEvent {
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
impl InterfaceReader for ExtForeignToplevelListV1ToplevelEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::ForeignToplevelList(
                ForeignToplevelListObjectInteface::ExtForeignToplevelListV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let toplevel = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::from_name(
                "ext_foreign_toplevel_handle_v1",
                interface.version().min(1),
            )
            .ok_or(FdReaderError::InvalidProtocol)?;
            Self::reader_state(state)?
                .register_object(toplevel, new_interface, true)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            toplevel,
        })
    }
}
impl CommandWriter for ExtForeignToplevelListV1ToplevelEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "ext_foreign_toplevel_list_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::from_name(
                "ext_foreign_toplevel_handle_v1",
                interface.version().min(1),
            )
            .ok_or(FdWriterError::InvalidProtocol)?;
            Self::writer_state(state)?
                .register_object(self.toplevel, new_interface, true)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.toplevel).await?;
        Ok(())
    }
}
