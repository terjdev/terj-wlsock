use crate::protocol::ext::xdg_toplevel_drag::ObjectInteface as XdgToplevelDragObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_toplevel_drag_manager_v1.get_xdg_toplevel_drag`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="get_xdg_toplevel_drag">
///       <description summary="get an xdg_toplevel_drag for a wl_data_source">
///         Create an xdg_toplevel_drag for a drag and drop operation that is going
///         to be started with data_source.
///
///         This request can only be made on sources used in drag-and-drop, so it
///         must be performed before wl_data_device.start_drag. Attempting to use
///         the source other than for drag-and-drop such as in
///         wl_data_device.set_selection will raise an invalid_source error.
///
///         Destroying data_source while a toplevel is attached to the
///         xdg_toplevel_drag is undefined.
///       </description>
///
///       <arg name="id" type="new_id" interface="xdg_toplevel_drag_v1" />
///       <arg name="data_source" type="object" interface="wl_data_source" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgToplevelDragManagerV1GetXdgToplevelDragRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `id`.
    pub id: u32,
    /// Wire argument `data_source`.
    pub data_source: u32,
}
impl XdgToplevelDragManagerV1GetXdgToplevelDragRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgToplevelDragManagerV1GetXdgToplevelDragRequest {
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
impl InterfaceReader for XdgToplevelDragManagerV1GetXdgToplevelDragRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgToplevelDrag(
                XdgToplevelDragObjectInteface::XdgToplevelDragManagerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = reader.next_u32().await?;
        let data_source = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("xdg_toplevel_drag_v1", interface.version().min(1))
                    .ok_or(FdReaderError::InvalidProtocol)?;
            Self::reader_state(state)?
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            data_source,
        })
    }
}
impl CommandWriter for XdgToplevelDragManagerV1GetXdgToplevelDragRequest {
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
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "xdg_toplevel_drag_manager_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("xdg_toplevel_drag_v1", interface.version().min(1))
                    .ok_or(FdWriterError::InvalidProtocol)?;
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_u32(self.data_source).await?;
        Ok(())
    }
}
