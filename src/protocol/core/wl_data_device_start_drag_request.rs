use crate::protocol::ClientState;
use crate::protocol::core::ObjectInteface as CoreObjectInteface;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wl_data_device.start_drag`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire framing and receiver metadata; object semantics belong to the compositor.
/// The caller handles compositor policy and message contents.
/// Client encoding checks the complete payload quota before writing arguments.
/// New object IDs are registered before payload I/O; client IDs remain reserved
/// after destruction until the server acknowledges them with delete_id.
/// ```xml
/// <request name="start_drag">
///       <description summary="start drag-and-drop operation">
///         This request asks the compositor to start a drag-and-drop
///         operation on behalf of the client.
///
///         The source argument is the data source that provides the data
///         for the eventual data transfer. If source is NULL, enter, leave
///         and motion events are sent only to the client that initiated the
///         drag and the client is expected to handle the data passing
///         internally. If source is destroyed, the drag-and-drop session will be
///         cancelled.
///
///         The origin surface is the surface where the drag originates and
///         the client must have an active implicit grab that matches the
///         serial.
///
///         The icon surface is an optional (can be NULL) surface that
///         provides an icon to be moved around with the cursor.  Initially,
///         the top-left corner of the icon surface is placed at the cursor
///         hotspot, but subsequent wl_surface.offset requests can move the
///         relative position. Attach requests must be confirmed with
///         wl_surface.commit as usual. The icon surface is given the role of
///         a drag-and-drop icon. If the icon surface already has another role,
///         it raises a protocol error.
///
///         The input region is ignored for wl_surfaces with the role of a
///         drag-and-drop icon.
///
///         The given source may not be used in any further set_selection or
///         start_drag requests. Attempting to reuse a previously-used source
///         may send a used_source error.
///       </description>
///       <arg name="source" type="object" interface="wl_data_source" allow-null="true" summary="data source for the eventual transfer" />
///       <arg name="origin" type="object" interface="wl_surface" summary="surface where the drag originates" />
///       <arg name="icon" type="object" interface="wl_surface" allow-null="true" summary="drag-and-drop icon surface" />
///       <arg name="serial" type="uint" summary="serial number of the implicit grab on the origin" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlDataDeviceStartDragRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `source`: data source for the eventual transfer.
    /// The protocol permits NULL.
    /// Object implementing interface wl_data_source.
    pub source: Option<u32>,
    /// Argument `origin`: surface where the drag originates.
    /// Object implementing interface wl_surface.
    pub origin: u32,
    /// Argument `icon`: drag-and-drop icon surface.
    /// The protocol permits NULL.
    /// Object implementing interface wl_surface.
    pub icon: Option<u32>,
    /// Argument `serial`: serial number of the implicit grab on the origin.
    pub serial: u32,
}
impl WlDataDeviceStartDragRequest {
    /// Opcode in the request namespace of interface wl_data_device.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlDataDeviceStartDragRequest {
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

impl InterfaceReader for WlDataDeviceStartDragRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Core(CoreObjectInteface::WlDataDevice { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let source = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        let origin = reader.next_u32().await?;
        let icon = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        let serial = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            source,
            origin,
            icon,
            serial,
        })
    }
}

impl CommandWriter for WlDataDeviceStartDragRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(state, self.object_id, "wl_data_device", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.source.unwrap_or(0)).await?;
        writer.next_u32(self.origin).await?;
        writer.next_u32(self.icon.unwrap_or(0)).await?;
        writer.next_u32(self.serial).await?;
        Ok(())
    }
}
