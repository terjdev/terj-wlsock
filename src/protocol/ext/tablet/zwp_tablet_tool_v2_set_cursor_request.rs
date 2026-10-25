use crate::protocol::ext::tablet::ObjectInteface as TabletObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_tablet_tool_v2.set_cursor`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_cursor">
///       <description summary="set the tablet tool's surface">
///     Sets the surface of the cursor used for this tool on the given
///     tablet. This request only takes effect if the tool is in proximity
///     of one of the requesting client's surfaces or the surface parameter
///     is the current pointer surface. If there was a previous surface set
///     with this request it is replaced. If surface is NULL, the cursor
///     image is hidden.
///
///     The parameters hotspot_x and hotspot_y define the position of the
///     pointer surface relative to the pointer location. Its top-left corner
///     is always at (x, y) - (hotspot_x, hotspot_y), where (x, y) are the
///     coordinates of the pointer location, in surface-local coordinates.
///
///     On surface.attach requests to the pointer surface, hotspot_x and
///     hotspot_y are decremented by the x and y parameters passed to the
///     request. Attach must be confirmed by wl_surface.commit as usual.
///
///     The hotspot can also be updated by passing the currently set pointer
///     surface to this request with new values for hotspot_x and hotspot_y.
///
///     The current and pending input regions of the wl_surface are cleared,
///     and wl_surface.set_input_region is ignored until the wl_surface is no
///     longer used as the cursor. When the use as a cursor ends, the current
///     and pending input regions become undefined, and the wl_surface is
///     unmapped.
///
///     This request gives the surface the role of a zwp_tablet_tool_v2 cursor. A
///     surface may only ever be used as the cursor surface for one
///     zwp_tablet_tool_v2. If the surface already has another role or has
///     previously been used as cursor surface for a different tool, a
///     protocol error is raised.
///       </description>
///       <arg name="serial" type="uint" summary="serial of the proximity_in event" />
///       <arg name="surface" type="object" interface="wl_surface" allow-null="true" />
///       <arg name="hotspot_x" type="int" summary="surface-local x coordinate" />
///       <arg name="hotspot_y" type="int" summary="surface-local y coordinate" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpTabletToolV2SetCursorRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// serial of the proximity_in event.
    pub serial: u32,
    /// Wire argument `surface`.
    pub surface: Option<u32>,
    /// surface-local x coordinate.
    pub hotspot_x: i32,
    /// surface-local y coordinate.
    pub hotspot_y: i32,
}
impl ZwpTabletToolV2SetCursorRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTabletToolV2SetCursorRequest {
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
impl InterfaceReader for ZwpTabletToolV2SetCursorRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletToolV2 { .. })
        ) || opcode != Self::OPCODE
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
impl CommandWriter for ZwpTabletToolV2SetCursorRequest {
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
        Self::writer_request_interface(state, self.object_id, "zwp_tablet_tool_v2", Self::SINCE)?;
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
