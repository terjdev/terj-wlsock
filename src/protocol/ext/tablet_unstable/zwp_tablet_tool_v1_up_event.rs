use super::ObjectInteface as TabletUnstableObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_tablet_tool_v1.up`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="up">
///       <description summary="tablet tool is no longer making contact">
///     Sent whenever the tablet tool stops making contact with the surface of
///     the tablet, or when the tablet tool moves out of the input region
///     and the compositor grab (if any) is dismissed.
///
///     If the tablet tool moves out of the input region while in contact
///     with the surface of the tablet and the compositor does not have an
///     ongoing grab on the surface, the client owning said region will
///     receive a wp_tablet.up event, followed by a wp_tablet.proximity_out
///     event and a wp_tablet.frame event. If the compositor has an ongoing
///     grab on this device, this event sequence is sent whenever the grab
///     is dismissed in the future.
///
///     Note that this event describes logical contact, not physical
///     contact. On some devices, a compositor may not consider a tool out
///     of logical contact until physical pressure falls below a specific
///     threshold.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTabletToolV1UpEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl ZwpTabletToolV1UpEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 9;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTabletToolV1UpEvent {
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
impl InterfaceReader for ZwpTabletToolV1UpEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::TabletUnstable(TabletUnstableObjectInteface::ZwpTabletToolV1 { .. })
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
impl CommandWriter for ZwpTabletToolV1UpEvent {
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
        let interface =
            Self::writer_event_interface(state, self.object_id, "zwp_tablet_tool_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::TabletUnstable(TabletUnstableObjectInteface::ZwpTabletToolV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        Ok(())
    }
}
