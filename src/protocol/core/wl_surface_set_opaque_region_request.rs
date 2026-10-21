use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_surface.set_opaque_region`.
///
/// Opcode 4; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="set_opaque_region">
///       <description summary="set opaque region">
///         This request sets the region of the surface that contains
///         opaque content.
///
///         The opaque region is an optimization hint for the compositor
///         that lets it optimize the redrawing of content behind opaque
///         regions.  Setting an opaque region is not required for correct
///         behaviour, but marking transparent content as opaque will result
///         in repaint artifacts.
///
///         The opaque region is specified in surface-local coordinates.
///
///         The compositor ignores the parts of the opaque region that fall
///         outside of the surface.
///
///         Opaque region is double-buffered state, see wl_surface.commit.
///
///         wl_surface.set_opaque_region changes the pending opaque region.
///         wl_surface.commit copies the pending region to the current region.
///         Otherwise, the pending and current regions are never changed.
///
///         The initial value for an opaque region is empty. Setting the pending
///         opaque region has copy semantics, and the wl_region object can be
///         destroyed immediately. A NULL wl_region causes the pending opaque
///         region to be set to empty.
///       </description>
///       <arg name="region" type="object" interface="wl_region" allow-null="true" summary="opaque region of the surface" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlSurfaceSetOpaqueRegionRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `region`: opaque region of the surface.
    /// The protocol permits NULL.
    /// Object implementing interface wl_region.
    pub region: Option<u32>,
}
impl WlSurfaceSetOpaqueRegionRequest {
    /// Opcode in the request namespace of interface wl_surface.
    pub const OPCODE: u16 = 4;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlSurfaceSetOpaqueRegionRequest {
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
        let region = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, region })
    }
}
