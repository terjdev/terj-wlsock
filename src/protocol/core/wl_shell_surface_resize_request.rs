use super::WlShellSurfaceResize;
use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_shell_surface.resize`.
///
/// Opcode 2; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="resize">
///       <description summary="start an interactive resize">
///         Start a pointer-driven resizing of the surface.
///
///         This request must be used in response to a button press event.
///         The server may ignore resize requests depending on the state of
///         the surface (e.g. fullscreen or maximized).
///       </description>
///       <arg name="seat" type="object" interface="wl_seat" summary="seat whose pointer is used" />
///       <arg name="serial" type="uint" summary="serial number of the implicit grab on the pointer" />
///       <arg name="edges" type="uint" enum="resize" summary="which edge or corner is being dragged" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlShellSurfaceResizeRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `seat`: seat whose pointer is used.
    /// Object implementing interface wl_seat.
    pub seat: u32,
    /// Argument `serial`: serial number of the implicit grab on the pointer.
    pub serial: u32,
    /// Argument `edges`: which edge or corner is being dragged.
    pub edges: WlShellSurfaceResize,
}
impl WlShellSurfaceResizeRequest {
    /// Opcode in the request namespace of interface wl_shell_surface.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlShellSurfaceResizeRequest {
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
        let edges = WlShellSurfaceResize(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            seat,
            serial,
            edges,
        })
    }
}
