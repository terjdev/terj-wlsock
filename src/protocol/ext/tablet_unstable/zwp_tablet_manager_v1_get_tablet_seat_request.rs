use super::ObjectInteface as TabletUnstableObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_tablet_manager_v1.get_tablet_seat`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="get_tablet_seat">
///       <description summary="get the tablet seat">
///     Get the wp_tablet_seat object for the given seat. This object
///     provides access to all graphics tablets in this seat.
///       </description>
///       <arg name="tablet_seat" type="new_id" interface="zwp_tablet_seat_v1" />
///       <arg name="seat" type="object" interface="wl_seat" summary="The wl_seat object to retrieve the tablets for" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpTabletManagerV1GetTabletSeatRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `tablet_seat`.
    pub tablet_seat: u32,
    /// The wl_seat object to retrieve the tablets for.
    pub seat: u32,
}
impl ZwpTabletManagerV1GetTabletSeatRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTabletManagerV1GetTabletSeatRequest {
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
impl InterfaceReader for ZwpTabletManagerV1GetTabletSeatRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::TabletUnstable(TabletUnstableObjectInteface::ZwpTabletManagerV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let tablet_seat = reader.next_u32().await?;
        let seat = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::TabletUnstable(TabletUnstableObjectInteface::ZwpTabletSeatV1 {
                    version: interface.version().min(1),
                });
            Self::reader_state(state)?
                .register_object(tablet_seat, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            tablet_seat,
            seat,
        })
    }
}
impl CommandWriter for ZwpTabletManagerV1GetTabletSeatRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "zwp_tablet_manager_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::TabletUnstable(TabletUnstableObjectInteface::ZwpTabletManagerV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::TabletUnstable(TabletUnstableObjectInteface::ZwpTabletSeatV1 {
                    version: interface.version().min(1),
                });
            Self::writer_state(state)?
                .register_object(self.tablet_seat, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.tablet_seat).await?;
        writer.next_u32(self.seat).await?;
        Ok(())
    }
}
