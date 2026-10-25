use super::ObjectInteface as PointerConstraintsObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_confined_pointer_v1.unconfined`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="unconfined">
///       <description summary="pointer unconfined">
///     Notification that the pointer confinement of the seat's pointer is no
///     longer active. If this is a oneshot pointer confinement (see
///     wp_pointer_constraints.lifetime) this object is now defunct and should
///     be destroyed. If this is a persistent pointer confinement (see
///     wp_pointer_constraints.lifetime) this pointer confinement may again
///     reactivate in the future.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpConfinedPointerV1UnconfinedEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl ZwpConfinedPointerV1UnconfinedEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpConfinedPointerV1UnconfinedEvent {
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
impl InterfaceReader for ZwpConfinedPointerV1UnconfinedEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::PointerConstraints(
                PointerConstraintsObjectInteface::ZwpConfinedPointerV1 { .. }
            )
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
impl CommandWriter for ZwpConfinedPointerV1UnconfinedEvent {
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
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "zwp_confined_pointer_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::PointerConstraints(
                PointerConstraintsObjectInteface::ZwpConfinedPointerV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        Ok(())
    }
}
