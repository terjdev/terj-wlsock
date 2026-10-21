use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_subcompositor.get_subsurface`.
///
/// Opcode 1; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="get_subsurface">
///       <description summary="give a surface the role sub-surface">
///         Create a sub-surface interface for the given surface, and
///         associate it with the given parent surface. This turns a
///         plain wl_surface into a sub-surface.
///
///         The to-be sub-surface must not already have another role, and it
///         must not have an existing wl_subsurface object. Otherwise the
///         bad_surface protocol error is raised.
///
///         Adding sub-surfaces to a parent is a double-buffered operation on the
///         parent (see wl_surface.commit). The effect of adding a sub-surface
///         becomes visible on the next time the state of the parent surface is
///         applied.
///
///         The parent surface must not be one of the child surface's descendants,
///         and the parent must be different from the child surface, otherwise the
///         bad_parent protocol error is raised.
///
///         This request modifies the behaviour of wl_surface.commit request on
///         the sub-surface, see the documentation on wl_subsurface interface.
///       </description>
///       <arg name="id" type="new_id" interface="wl_subsurface" summary="the new sub-surface object ID" />
///       <arg name="surface" type="object" interface="wl_surface" summary="the surface to be turned into a sub-surface" />
///       <arg name="parent" type="object" interface="wl_surface" summary="the parent surface" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlSubcompositorGetSubsurfaceRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `id`: the new sub-surface object ID.
    /// Object implementing interface wl_subsurface.
    pub id: u32,
    /// Argument `surface`: the surface to be turned into a sub-surface.
    /// Object implementing interface wl_surface.
    pub surface: u32,
    /// Argument `parent`: the parent surface.
    /// Object implementing interface wl_surface.
    pub parent: u32,
}
impl WlSubcompositorGetSubsurfaceRequest {
    /// Opcode in the request namespace of interface wl_subcompositor.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlSubcompositorGetSubsurfaceRequest {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        let receiver_interface = Self::request_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_subcompositor",
            Self::SINCE,
        )?;
        let id = reader.next_u32().await?;
        let surface = reader.next_u32().await?;
        let parent = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let mut state = Self::read_state(state)?;
            let new_interface =
                ObjectInteface::from_name("wl_subsurface", receiver_interface.version().min(1))
                    .ok_or(FdReaderError::InvalidProtocol)?;
            state
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            surface,
            parent,
        })
    }
}
