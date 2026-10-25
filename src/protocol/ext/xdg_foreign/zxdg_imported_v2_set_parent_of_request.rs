use super::ObjectInteface as XdgForeignObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zxdg_imported_v2.set_parent_of`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_parent_of">
///       <description summary="set as the parent of some surface">
///         Set the imported surface as the parent of some surface of the client.
///         The passed surface must be an xdg_toplevel equivalent, otherwise an
///         invalid_surface protocol error is sent. Calling this function sets up
///         a surface to surface relation with the same stacking and positioning
///         semantics as xdg_toplevel.set_parent.
///       </description>
///       <arg name="surface" type="object" interface="wl_surface" summary="the child surface" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZxdgImportedV2SetParentOfRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the child surface.
    pub surface: u32,
}
impl ZxdgImportedV2SetParentOfRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZxdgImportedV2SetParentOfRequest {
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
impl InterfaceReader for ZxdgImportedV2SetParentOfRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgImportedV2 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let surface = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, surface })
    }
}
impl CommandWriter for ZxdgImportedV2SetParentOfRequest {
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
        let interface =
            Self::writer_request_interface(state, self.object_id, "zxdg_imported_v2", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgImportedV2 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.surface).await?;
        Ok(())
    }
}
