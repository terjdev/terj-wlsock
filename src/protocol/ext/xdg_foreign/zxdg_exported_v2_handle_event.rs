use super::ObjectInteface as XdgForeignObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zxdg_exported_v2.handle`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="handle">
///       <description summary="the exported surface handle">
///     The handle event contains the unique handle of this exported surface
///     reference. It may be shared with any client, which then can use it to
///     import the surface by calling xdg_importer.import_toplevel. A handle
///     may be used to import the surface multiple times.
///       </description>
///       <arg name="handle" type="string" summary="the exported surface handle" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZxdgExportedV2HandleEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the exported surface handle.
    pub handle: String,
}
impl ZxdgExportedV2HandleEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZxdgExportedV2HandleEvent {
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
impl InterfaceReader for ZxdgExportedV2HandleEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgExportedV2 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let handle = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, handle })
    }
}
impl CommandWriter for ZxdgExportedV2HandleEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_string_size(Some(&self.handle))])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface =
            Self::writer_event_interface(state, self.object_id, "zxdg_exported_v2", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgExportedV2 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.handle) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.handle).await?;
        Ok(())
    }
}
