use crate::protocol::ext::fifo::ObjectInteface as FifoObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_fifo_v1.wait_barrier`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="wait_barrier">
///       <description summary="adds a fifo constraint to a content update">
///         Indicate that this content update is not ready while a
///         "fifo_barrier" condition is present on the surface.
///
///         This means that when the content update containing "set_barrier"
///         was made active at a latching deadline, it will be active for
///         at least one refresh cycle. A content update which is allowed to
///         tear might become active after a latching deadline if no content
///         update became active at the deadline.
///
///         The constraint must be ignored if the surface is a subsurface in
///         synchronized mode. If the surface is not being updated by the
///         compositor (off-screen, occluded) the compositor may ignore the
///         constraint. Clients must use an additional mechanism such as
///         frame callbacks or timestamps to ensure throttling occurs under
///         all conditions.
///
///         "wait_barrier" is double-buffered state, see wl_surface.commit.
///
///         Requesting "wait_barrier" after the fifo object's surface is
///         destroyed will generate a "surface_destroyed" error.
///       </description>
///     </request>
/// ```
#[derive(Debug)]
pub struct WpFifoV1WaitBarrierRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl WpFifoV1WaitBarrierRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpFifoV1WaitBarrierRequest {
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
impl InterfaceReader for WpFifoV1WaitBarrierRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Fifo(FifoObjectInteface::WpFifoV1 { .. })
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
impl CommandWriter for WpFifoV1WaitBarrierRequest {
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
        Self::writer_request_interface(state, self.object_id, "wp_fifo_v1", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        Ok(())
    }
}
