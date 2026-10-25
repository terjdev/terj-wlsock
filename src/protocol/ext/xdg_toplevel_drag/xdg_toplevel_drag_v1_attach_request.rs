use crate::protocol::ext::xdg_toplevel_drag::ObjectInteface as XdgToplevelDragObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_toplevel_drag_v1.attach`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="attach">
///       <description summary="Move a toplevel with the drag operation">
///         Request that the window will be moved with the cursor during the drag
///         operation. The offset is a hint to the compositor how the toplevel
///         should be positioned relative to the cursor hotspot in surface local
///         coordinates and relative to the geometry of the toplevel being attached.
///         See xdg_surface.set_window_geometry. For example it might only
///         be used when an unmapped window is attached. The attached window
///         does not participate in the selection of the drag target.
///
///         If the toplevel is unmapped while it is attached, it is automatically
///         detached from the drag. In this case this request has to be called again
///         if the window should be attached after it is remapped.
///
///         This request can be called multiple times but issuing it while a
///         toplevel with an active role is attached raises a toplevel_attached
///         error.
///       </description>
///
///       <arg name="toplevel" type="object" interface="xdg_toplevel" />
///       <arg name="x_offset" type="int" summary="dragged surface x offset" />
///       <arg name="y_offset" type="int" summary="dragged surface y offset" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgToplevelDragV1AttachRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `toplevel`.
    pub toplevel: u32,
    /// dragged surface x offset.
    pub x_offset: i32,
    /// dragged surface y offset.
    pub y_offset: i32,
}
impl XdgToplevelDragV1AttachRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgToplevelDragV1AttachRequest {
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
impl InterfaceReader for XdgToplevelDragV1AttachRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgToplevelDrag(
                XdgToplevelDragObjectInteface::XdgToplevelDragV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let toplevel = reader.next_u32().await?;
        let x_offset = reader.next_i32().await?;
        let y_offset = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            toplevel,
            x_offset,
            y_offset,
        })
    }
}
impl CommandWriter for XdgToplevelDragV1AttachRequest {
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
        Self::writer_request_interface(state, self.object_id, "xdg_toplevel_drag_v1", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.toplevel).await?;
        writer.next_i32(self.x_offset).await?;
        writer.next_i32(self.y_offset).await?;
        Ok(())
    }
}
