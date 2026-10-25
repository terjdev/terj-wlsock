use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wl_surface.damage_buffer`.
///
/// Opcode 9; available since version 4. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire framing and receiver metadata; object semantics belong to the compositor.
/// The caller handles compositor policy and message contents.
/// Client encoding checks the complete payload quota before writing arguments.
/// New object IDs are registered before payload I/O; client IDs remain reserved
/// after destruction until the server acknowledges them with delete_id.
/// ```xml
/// <request name="damage_buffer" since="4">
///       <description summary="mark part of the surface damaged using buffer coordinates">
///         This request is used to describe the regions where the pending
///         buffer is different from the current surface contents, and where
///         the surface therefore needs to be repainted. The compositor
///         ignores the parts of the damage that fall outside of the surface.
///
///         Damage is double-buffered state, see wl_surface.commit.
///
///         The damage rectangle is specified in buffer coordinates,
///         where x and y specify the upper left corner of the damage rectangle.
///
///         The initial value for pending damage is empty: no damage.
///         wl_surface.damage_buffer adds pending damage: the new pending
///         damage is the union of old pending damage and the given rectangle.
///
///         wl_surface.commit assigns pending damage as the current damage,
///         and clears pending damage. The server will clear the current
///         damage as it repaints the surface.
///
///         This request differs from wl_surface.damage in only one way - it
///         takes damage in buffer coordinates instead of surface-local
///         coordinates. While this generally is more intuitive than surface
///         coordinates, it is especially desirable when using wp_viewport
///         or when a drawing library (like EGL) is unaware of buffer scale
///         and buffer transform.
///
///         Note: Because buffer transformation changes and damage requests may
///         be interleaved in the protocol stream, it is impossible to determine
///         the actual mapping between surface and buffer damage until
///         wl_surface.commit time. Therefore, compositors wishing to take both
///         kinds of damage into account will have to accumulate damage from the
///         two requests separately and only transform from one to the other
///         after receiving the wl_surface.commit.
///       </description>
///       <arg name="x" type="int" summary="buffer-local x coordinate" />
///       <arg name="y" type="int" summary="buffer-local y coordinate" />
///       <arg name="width" type="int" summary="width of damage rectangle" />
///       <arg name="height" type="int" summary="height of damage rectangle" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlSurfaceDamageBufferRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `x`: buffer-local x coordinate.
    pub x: i32,
    /// Argument `y`: buffer-local y coordinate.
    pub y: i32,
    /// Argument `width`: width of damage rectangle.
    pub width: i32,
    /// Argument `height`: height of damage rectangle.
    pub height: i32,
}
impl WlSurfaceDamageBufferRequest {
    /// Opcode in the request namespace of interface wl_surface.
    pub const OPCODE: u16 = 9;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 4;
}
impl CommandReader<Self> for WlSurfaceDamageBufferRequest {
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

impl InterfaceReader for WlSurfaceDamageBufferRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(interface, ObjectInteface::WlSurface { .. })
            || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let x = reader.next_i32().await?;
        let y = reader.next_i32().await?;
        let width = reader.next_i32().await?;
        let height = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            x,
            y,
            width,
            height,
        })
    }
}

impl CommandWriter for WlSurfaceDamageBufferRequest {
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
        Self::writer_request_interface(state, self.object_id, "wl_surface", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.x).await?;
        writer.next_i32(self.y).await?;
        writer.next_i32(self.width).await?;
        writer.next_i32(self.height).await?;
        Ok(())
    }
}
