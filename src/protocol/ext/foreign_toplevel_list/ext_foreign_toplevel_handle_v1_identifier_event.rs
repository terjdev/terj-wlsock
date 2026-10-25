use crate::protocol::ext::foreign_toplevel_list::ObjectInteface as ForeignToplevelListObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `ext_foreign_toplevel_handle_v1.identifier`.
///
/// Opcode 4; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="identifier">
///       <description summary="a stable identifier for a toplevel">
///         This identifier is used to check if two or more toplevel handles belong
///         to the same toplevel.
///
///         The identifier is useful for command line tools or privileged clients
///         which may need to reference an exact toplevel across processes or
///         instances of the ext_foreign_toplevel_list_v1 global.
///
///         The compositor must only send this event when the handle is created.
///
///         The identifier must be unique per toplevel and its handles. Two different
///         toplevels must not have the same identifier. The identifier is only valid
///         as long as the toplevel is mapped. If the toplevel is unmapped the identifier
///         must not be reused. An identifier must not be reused by the compositor to
///         ensure there are no races when sharing identifiers between processes.
///
///         An identifier is a string that contains up to 32 printable ASCII bytes.
///         An identifier must not be an empty string. It is recommended that a
///         compositor includes an opaque generation value in identifiers. How the
///         generation value is used when generating the identifier is implementation
///         dependent.
///       </description>
///       <arg name="identifier" type="string" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ExtForeignToplevelHandleV1IdentifierEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `identifier`.
    pub identifier: String,
}
impl ExtForeignToplevelHandleV1IdentifierEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 4;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ExtForeignToplevelHandleV1IdentifierEvent {
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
impl InterfaceReader for ExtForeignToplevelHandleV1IdentifierEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::ForeignToplevelList(
                ForeignToplevelListObjectInteface::ExtForeignToplevelHandleV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let identifier = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            identifier,
        })
    }
}
impl CommandWriter for ExtForeignToplevelHandleV1IdentifierEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_string_size(Some(&self.identifier))])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_event_interface(
            state,
            self.object_id,
            "ext_foreign_toplevel_handle_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.identifier) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.identifier).await?;
        Ok(())
    }
}
