use super::WlShellSurfaceTransient;
use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_shell_surface.set_popup`.
///
/// Opcode 6; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="set_popup">
///       <description summary="make the surface a popup surface">
///         Map the surface as a popup.
///
///         A popup surface is a transient surface with an added pointer
///         grab.
///
///         An existing implicit grab will be changed to owner-events mode,
///         and the popup grab will continue after the implicit grab ends
///         (i.e. releasing the mouse button does not cause the popup to
///         be unmapped).
///
///         The popup grab continues until the window is destroyed or a
///         mouse button is pressed in any other client's window. A click
///         in any of the client's surfaces is reported as normal, however,
///         clicks in other clients' surfaces will be discarded and trigger
///         the callback.
///
///         The x and y arguments specify the location of the upper left
///         corner of the surface relative to the upper left corner of the
///         parent surface, in surface-local coordinates.
///       </description>
///       <arg name="seat" type="object" interface="wl_seat" summary="seat whose pointer is used" />
///       <arg name="serial" type="uint" summary="serial number of the implicit grab on the pointer" />
///       <arg name="parent" type="object" interface="wl_surface" summary="parent surface" />
///       <arg name="x" type="int" summary="surface-local x coordinate" />
///       <arg name="y" type="int" summary="surface-local y coordinate" />
///       <arg name="flags" type="uint" enum="transient" summary="transient surface behavior" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlShellSurfaceSetPopupRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `seat`: seat whose pointer is used.
    /// Object implementing interface wl_seat.
    pub seat: u32,
    /// Argument `serial`: serial number of the implicit grab on the pointer.
    pub serial: u32,
    /// Argument `parent`: parent surface.
    /// Object implementing interface wl_surface.
    pub parent: u32,
    /// Argument `x`: surface-local x coordinate.
    pub x: i32,
    /// Argument `y`: surface-local y coordinate.
    pub y: i32,
    /// Argument `flags`: transient surface behavior.
    pub flags: WlShellSurfaceTransient,
}
impl WlShellSurfaceSetPopupRequest {
    /// Opcode in the request namespace of interface wl_shell_surface.
    pub const OPCODE: u16 = 6;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlShellSurfaceSetPopupRequest {
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
            "wl_shell_surface",
            Self::SINCE,
        )?;
        let seat = reader.next_u32().await?;
        let serial = reader.next_u32().await?;
        let parent = reader.next_u32().await?;
        let x = reader.next_i32().await?;
        let y = reader.next_i32().await?;
        let flags = WlShellSurfaceTransient(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            seat,
            serial,
            parent,
            x,
            y,
            flags,
        })
    }
}
