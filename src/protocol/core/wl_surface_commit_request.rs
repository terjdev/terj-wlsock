use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_surface.commit`.
///
/// Opcode 6; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="commit">
///       <description summary="commit pending surface state">
///         Surface state (input, opaque, and damage regions, attached buffers,
///         etc.) is double-buffered. Protocol requests modify the pending state,
///         as opposed to the active state in use by the compositor.
///
///         All requests that need a commit to become effective are documented
///         to affect double-buffered state.
///
///         Other interfaces may add further double-buffered surface state.
///
///         A commit request atomically creates a Content Update (CU) from the
///         pending state, even if the pending state has not been touched. The
///         content update is placed at the end of a per-surface queue until it
///         becomes active. After commit, the new pending state is as documented for
///         each related request.
///
///         A CU is either a Desync Content Update (DCU) or a Sync Content Update
///         (SCU). If the surface is effectively synchronized at the commit request,
///         it is a SCU, otherwise a DCU.
///
///         When a surface transitions from effectively synchronized to effectively
///         desynchronized, all SCUs in its queue which are not reachable by any
///         DCU become DCUs and dependency edges from outside the queue to these CUs
///         are removed.
///
///         See wl_subsurface for the definition of 'effectively synchronized' and
///         'effectively desynchronized'.
///
///         When a CU is placed in the queue, the CU has a dependency on the CU in
///         front of it and to the SCU at end of the queue of every direct child
///         surface if that SCU exists and does not have another dependent. This can
///         form a directed acyclic graph of CUs with dependencies as edges.
///
///         In addition to surface state, the CU can have constraints that must be
///         satisfied before it can be applied. Other interfaces may add CU
///         constraints.
///
///         All DCUs which do not have a SCU in front of themselves in their queue,
///         are candidates. If the graph that's reachable by a candidate does not
///         have any unsatisfied constraints, the entire graph must be applied
///         atomically.
///
///         When a CU is applied, the wl_buffer is applied before all other state.
///         This means that all coordinates in double-buffered state are relative to
///         the newly attached wl_buffers, except for wl_surface.attach itself. If
///         there is no newly attached wl_buffer, the coordinates are relative to
///         the previous content update.
///       </description>
///     </request>
/// ```
#[derive(Debug)]
pub struct WlSurfaceCommitRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
}
impl WlSurfaceCommitRequest {
    /// Opcode in the request namespace of interface wl_surface.
    pub const OPCODE: u16 = 6;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlSurfaceCommitRequest {
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
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id })
    }
}
