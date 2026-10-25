use super::ObjectInteface as XdgForeignObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zxdg_exporter_v1.export`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="export">
///       <description summary="export a surface">
///     The export request exports the passed surface so that it can later be
///     imported via xdg_importer. When called, a new xdg_exported object will
///     be created and xdg_exported.handle will be sent immediately. See the
///     corresponding interface and event for details.
///
///     A surface may be exported multiple times, and each exported handle may
///     be used to create an xdg_imported multiple times. Only xdg_surface
///     surfaces may be exported.
///       </description>
///       <arg name="id" type="new_id" interface="zxdg_exported_v1" summary="the new xdg_exported object" />
///       <arg name="surface" type="object" interface="wl_surface" summary="the surface to export" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZxdgExporterV1ExportRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the new xdg_exported object.
    pub id: u32,
    /// the surface to export.
    pub surface: u32,
}
impl ZxdgExporterV1ExportRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZxdgExporterV1ExportRequest {
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
impl InterfaceReader for ZxdgExporterV1ExportRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgExporterV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = reader.next_u32().await?;
        let surface = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgExportedV1 {
                    version: interface.version().min(1),
                });
            Self::reader_state(state)?
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            surface,
        })
    }
}
impl CommandWriter for ZxdgExporterV1ExportRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface =
            Self::writer_request_interface(state, self.object_id, "zxdg_exporter_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgExporterV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgExportedV1 {
                    version: interface.version().min(1),
                });
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_u32(self.surface).await?;
        Ok(())
    }
}
