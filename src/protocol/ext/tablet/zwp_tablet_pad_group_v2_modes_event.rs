use crate::protocol::ext::tablet::ObjectInteface as TabletObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_tablet_pad_group_v2.modes`.
///
/// Opcode 3; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="modes">
///       <description summary="mode-switch ability announced">
///     Sent on zwp_tablet_pad_group_v2 initialization to announce that the pad
///     group may switch between modes. A client may use a mode to store a
///     specific configuration for buttons, rings and strips and use the
///     zwp_tablet_pad_group_v2.mode_switch event to toggle between these
///     configurations. Mode indices start at 0.
///
///     Switching modes is compositor-dependent. See the
///     zwp_tablet_pad_group_v2.mode_switch event for more details.
///
///     This event is sent in the initial burst of events before the
///     zwp_tablet_pad_group_v2.done event. This event is only sent when
///     more than one mode is available.
///       </description>
///       <arg name="modes" type="uint" summary="the number of modes" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTabletPadGroupV2ModesEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the number of modes.
    pub modes: u32,
}
impl ZwpTabletPadGroupV2ModesEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTabletPadGroupV2ModesEvent {
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
impl InterfaceReader for ZwpTabletPadGroupV2ModesEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletPadGroupV2 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let modes = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, modes })
    }
}
impl CommandWriter for ZwpTabletPadGroupV2ModesEvent {
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
        Self::writer_event_interface(
            state,
            self.object_id,
            "zwp_tablet_pad_group_v2",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.modes).await?;
        Ok(())
    }
}
