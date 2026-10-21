use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wl_subsurface.set_position`.
///
/// Opcode 1; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire framing and receiver metadata; object semantics belong to the compositor.
/// The caller handles compositor policy and message contents.
/// Client encoding checks the complete payload quota before writing arguments.
/// New object IDs are registered before payload I/O; client IDs remain reserved
/// after destruction until the server acknowledges them with delete_id.
/// ```xml
/// <request name="set_position">
///       <description summary="reposition the sub-surface">
///         This sets the position of the sub-surface, relative to the parent
///         surface.
///
///         The sub-surface will be moved so that its origin (top left
///         corner pixel) will be at the location x, y of the parent surface
///         coordinate system. The coordinates are not restricted to the parent
///         surface area. Negative values are allowed.
///
///         The initial position is 0, 0.
///
///         Position is double-buffered state on the parent surface, see
///         wl_subsurface and wl_surface.commit for more information.
///       </description>
///       <arg name="x" type="int" summary="x coordinate in the parent surface" />
///       <arg name="y" type="int" summary="y coordinate in the parent surface" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlSubsurfaceSetPositionRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `x`: x coordinate in the parent surface.
    pub x: i32,
    /// Argument `y`: y coordinate in the parent surface.
    pub y: i32,
}
impl WlSubsurfaceSetPositionRequest {
    /// Opcode in the request namespace of interface wl_subsurface.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlSubsurfaceSetPositionRequest {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Self::reader_request_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_subsurface",
            Self::SINCE,
        )?;
        let x = reader.next_i32().await?;
        let y = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, x, y })
    }
}

impl CommandWriter for WlSubsurfaceSetPositionRequest {
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
        Self::writer_request_interface(state, self.object_id, "wl_subsurface", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.x).await?;
        writer.next_i32(self.y).await?;
        Ok(())
    }
}
