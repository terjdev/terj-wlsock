use crate::protocol::ext::foreign_toplevel_list::ObjectInteface as ForeignToplevelListObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `ext_foreign_toplevel_handle_v1.destroy`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="destroy" type="destructor">
///       <description summary="destroy the ext_foreign_toplevel_handle_v1 object">
///         This request should be used when the client will no longer use the handle
///         or after the closed event has been received to allow destruction of the
///         object.
///
///         When a handle is destroyed, a new handle may not be created by the server
///         until the toplevel is unmapped and then remapped. Destroying a toplevel handle
///         is not recommended unless the client is cleaning up child objects
///         before destroying the ext_foreign_toplevel_list_v1 object, the toplevel
///         was closed or the toplevel handle will not be used in the future.
///
///         Other protocols which extend the ext_foreign_toplevel_handle_v1
///         interface should require destructors for extension interfaces be
///         called before allowing the toplevel handle to be destroyed.
///       </description>
///     </request>
/// ```
#[derive(Debug)]
pub struct ExtForeignToplevelHandleV1DestroyRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl ExtForeignToplevelHandleV1DestroyRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ExtForeignToplevelHandleV1DestroyRequest {
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
impl InterfaceReader for ExtForeignToplevelHandleV1DestroyRequest {
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
                ForeignToplevelListObjectInteface::ExtForeignToplevelHandleV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Self::reader_state(state)?.release_server_id(object_id);
        Ok(Self { object_id })
    }
}
impl CommandWriter for ExtForeignToplevelHandleV1DestroyRequest {
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
        Self::writer_request_interface(
            state,
            self.object_id,
            "ext_foreign_toplevel_handle_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        Self::writer_state(state)?.release_server_id(self.object_id);
        Ok(())
    }
}
