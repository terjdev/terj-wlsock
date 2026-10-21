use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_surface.damage`.
///
/// Opcode 2; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="damage">
///       <description summary="mark part of the surface damaged">
///         This request is used to describe the regions where the pending
///         buffer is different from the current surface contents, and where
///         the surface therefore needs to be repainted. The compositor
///         ignores the parts of the damage that fall outside of the surface.
///
///         Damage is double-buffered state, see wl_surface.commit.
///
///         The damage rectangle is specified in surface-local coordinates,
///         where x and y specify the upper left corner of the damage rectangle.
///
///         The initial value for pending damage is empty: no damage.
///         wl_surface.damage adds pending damage: the new pending damage
///         is the union of old pending damage and the given rectangle.
///
///         wl_surface.commit assigns pending damage as the current damage,
///         and clears pending damage. The server will clear the current
///         damage as it repaints the surface.
///
///         Note! New clients should not use this request. Instead damage can be
///         posted with wl_surface.damage_buffer which uses buffer coordinates
///         instead of surface coordinates.
///       </description>
///       <arg name="x" type="int" summary="surface-local x coordinate" />
///       <arg name="y" type="int" summary="surface-local y coordinate" />
///       <arg name="width" type="int" summary="width of damage rectangle" />
///       <arg name="height" type="int" summary="height of damage rectangle" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlSurfaceDamageRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `x`: surface-local x coordinate.
    pub x: i32,
    /// Argument `y`: surface-local y coordinate.
    pub y: i32,
    /// Argument `width`: width of damage rectangle.
    pub width: i32,
    /// Argument `height`: height of damage rectangle.
    pub height: i32,
}
impl WlSurfaceDamageRequest {
    /// Opcode in the request namespace of interface wl_surface.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlSurfaceDamageRequest {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Self::request_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_surface",
            Self::SINCE,
        )?;
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
