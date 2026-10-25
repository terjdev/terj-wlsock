use crate::protocol::ext::fifo::ObjectInteface as FifoObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_fifo_v1.set_barrier`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_barrier">
///       <description summary="sets the start point for a fifo constraint">
///         When the content update containing the "set_barrier" is applied,
///         it sets a "fifo_barrier" condition on the surface associated with
///         the fifo object. The condition is cleared immediately after the
///         following latching deadline for non-tearing presentation.
///
///         The compositor may clear the condition early if it must do so to
///         ensure client forward progress assumptions.
///
///         To wait for this condition to clear, use the "wait_barrier" request.
///
///         "set_barrier" is double-buffered state, see wl_surface.commit.
///
///         Requesting set_barrier after the fifo object's surface is
///         destroyed will generate a "surface_destroyed" error.
///       </description>
///     </request>
/// ```
#[derive(Debug)]
pub struct WpFifoV1SetBarrierRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl WpFifoV1SetBarrierRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpFifoV1SetBarrierRequest {
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
impl InterfaceReader for WpFifoV1SetBarrierRequest {
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
impl CommandWriter for WpFifoV1SetBarrierRequest {
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
