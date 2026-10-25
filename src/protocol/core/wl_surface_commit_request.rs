use crate::protocol::ClientState;
use crate::protocol::core::ObjectInteface as CoreObjectInteface;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wl_surface.commit`.
///
/// Opcode 6; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire framing and receiver metadata; object semantics belong to the compositor.
/// The caller handles compositor policy and message contents.
/// Client encoding checks the complete payload quota before writing arguments.
/// New object IDs are registered before payload I/O; client IDs remain reserved
/// after destruction until the server acknowledges them with delete_id.
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
        let interface = Self::reader_state(state)?
            .object_interface(object_id)
            .ok_or(FdReaderError::InvalidProtocol)?;
        Self::read_with_interface(object_id, opcode, interface, reader, state).await
    }
}

impl InterfaceReader for WlSurfaceCommitRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Core(CoreObjectInteface::WlSurface { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id })
    }
}

impl CommandWriter for WlSurfaceCommitRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[])
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
        Ok(())
    }
}
