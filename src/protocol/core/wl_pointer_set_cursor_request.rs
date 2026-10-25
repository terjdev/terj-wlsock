use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wl_pointer.set_cursor`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire framing and receiver metadata; object semantics belong to the compositor.
/// The caller handles compositor policy and message contents.
/// Client encoding checks the complete payload quota before writing arguments.
/// New object IDs are registered before payload I/O; client IDs remain reserved
/// after destruction until the server acknowledges them with delete_id.
/// ```xml
/// <request name="set_cursor">
///       <description summary="set the pointer surface">
///         Set the pointer surface, i.e., the surface that contains the
///         pointer image (cursor). This request gives the surface the role
///         of a cursor. If the surface already has another role, it raises
///         a protocol error.
///
///         The cursor actually changes only if the pointer
///         focus for this device is one of the requesting client's surfaces
///         or the surface parameter is the current pointer surface. If
///         there was a previous surface set with this request it is
///         replaced. If surface is NULL, the pointer image is hidden.
///
///         The parameters hotspot_x and hotspot_y define the position of
///         the pointer surface relative to the pointer location. Its
///         top-left corner is always at (x, y) - (hotspot_x, hotspot_y),
///         where (x, y) are the coordinates of the pointer location, in
///         surface-local coordinates.
///
///         On wl_surface.offset requests to the pointer surface, hotspot_x
///         and hotspot_y are decremented by the x and y parameters
///         passed to the request. The offset must be applied by
///         wl_surface.commit as usual.
///
///         The hotspot can also be updated by passing the currently set
///         pointer surface to this request with new values for hotspot_x
///         and hotspot_y.
///
///         The input region is ignored for wl_surfaces with the role of
///         a cursor. When the use as a cursor ends, the wl_surface is
///         unmapped.
///
///         The serial parameter must match the latest wl_pointer.enter
///         serial number sent to the client. Otherwise the request will be
///         ignored.
///       </description>
///       <arg name="serial" type="uint" summary="serial number of the enter event" />
///       <arg name="surface" type="object" interface="wl_surface" allow-null="true" summary="pointer surface" />
///       <arg name="hotspot_x" type="int" summary="surface-local x coordinate" />
///       <arg name="hotspot_y" type="int" summary="surface-local y coordinate" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlPointerSetCursorRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `serial`: serial number of the enter event.
    pub serial: u32,
    /// Argument `surface`: pointer surface.
    /// The protocol permits NULL.
    /// Object implementing interface wl_surface.
    pub surface: Option<u32>,
    /// Argument `hotspot_x`: surface-local x coordinate.
    pub hotspot_x: i32,
    /// Argument `hotspot_y`: surface-local y coordinate.
    pub hotspot_y: i32,
}
impl WlPointerSetCursorRequest {
    /// Opcode in the request namespace of interface wl_pointer.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlPointerSetCursorRequest {
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

impl InterfaceReader for WlPointerSetCursorRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(interface, ObjectInteface::WlPointer { .. })
            || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let serial = reader.next_u32().await?;
        let surface = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        let hotspot_x = reader.next_i32().await?;
        let hotspot_y = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            serial,
            surface,
            hotspot_x,
            hotspot_y,
        })
    }
}

impl CommandWriter for WlPointerSetCursorRequest {
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
        Self::writer_request_interface(state, self.object_id, "wl_pointer", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        writer.next_u32(self.surface.unwrap_or(0)).await?;
        writer.next_i32(self.hotspot_x).await?;
        writer.next_i32(self.hotspot_y).await?;
        Ok(())
    }
}
