use super::ObjectInteface as XdgForeignObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zxdg_importer_v2.import_toplevel`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="import_toplevel">
///       <description summary="import a toplevel surface">
///     The import_toplevel request imports a surface from any client given a handle
///     retrieved by exporting said surface using xdg_exporter.export_toplevel.
///     When called, a new xdg_imported object will be created. This new object
///     represents the imported surface, and the importing client can
///     manipulate its relationship using it. See xdg_imported for details.
///       </description>
///       <arg name="id" type="new_id" interface="zxdg_imported_v2" summary="the new xdg_imported object" />
///       <arg name="handle" type="string" summary="the exported surface handle" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZxdgImporterV2ImportToplevelRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the new xdg_imported object.
    pub id: u32,
    /// the exported surface handle.
    pub handle: String,
}
impl ZxdgImporterV2ImportToplevelRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZxdgImporterV2ImportToplevelRequest {
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
impl InterfaceReader for ZxdgImporterV2ImportToplevelRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgImporterV2 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = reader.next_u32().await?;
        let handle = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgImportedV2 {
                    version: interface.version().min(1),
                });
            Self::reader_state(state)?
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            handle,
        })
    }
}
impl CommandWriter for ZxdgImporterV2ImportToplevelRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), FdWriter::encoded_string_size(Some(&self.handle))])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface =
            Self::writer_request_interface(state, self.object_id, "zxdg_importer_v2", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgImporterV2 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.handle) {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgImportedV2 {
                    version: interface.version().min(1),
                });
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_string(&self.handle).await?;
        Ok(())
    }
}
