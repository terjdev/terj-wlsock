use super::ZwpTabletV2Bustype;
use crate::protocol::ext::tablet::ObjectInteface as TabletObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_tablet_v2.bustype`.
///
/// Opcode 5; available since version 2.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="bustype" since="2">
///       <description summary="tablet device bus type">
///     The bustype argument is one of the BUS_ defines in the Linux kernel's
///     linux/input.h
///
///     If the device has no known bustype or the bustype cannot be
///     queried, this event is not sent.
///
///     This event is sent in the initial burst of events before the
///     zwp_tablet_v2.done event.
///       </description>
///       <arg name="bustype" type="uint" enum="bustype" summary="bus type" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTabletV2BustypeEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// bus type.
    pub bustype: ZwpTabletV2Bustype,
}
impl ZwpTabletV2BustypeEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 5;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 2;
}
impl CommandReader<Self> for ZwpTabletV2BustypeEvent {
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
impl InterfaceReader for ZwpTabletV2BustypeEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletV2 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let bustype = ZwpTabletV2Bustype(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, bustype })
    }
}
impl CommandWriter for ZwpTabletV2BustypeEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_event_interface(state, self.object_id, "zwp_tablet_v2", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.bustype.0).await?;
        Ok(())
    }
}
