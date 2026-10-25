use crate::protocol::ext::tablet::ObjectInteface as TabletObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_tablet_pad_group_v2.mode_switch`.
///
/// Opcode 5; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="mode_switch">
///       <description summary="mode switch event">
///     Notification that the mode was switched.
///
///     A mode applies to all buttons, rings, strips and dials in a group
///     simultaneously, but a client is not required to assign different actions
///     for each mode. For example, a client may have mode-specific button
///     mappings but map the ring to vertical scrolling in all modes. Mode
///     indices start at 0.
///
///     Switching modes is compositor-dependent. The compositor may provide
///     visual cues to the user about the mode, e.g. by toggling LEDs on
///     the tablet device. Mode-switching may be software-controlled or
///     controlled by one or more physical buttons. For example, on a Wacom
///     Intuos Pro, the button inside the ring may be assigned to switch
///     between modes.
///
///     The compositor will also send this event after zwp_tablet_pad_v2.enter on
///     each group in order to notify of the current mode. Groups that only
///     feature one mode will use mode=0 when emitting this event.
///
///     If a button action in the new mode differs from the action in the
///     previous mode, the client should immediately issue a
///     zwp_tablet_pad_v2.set_feedback request for each changed button.
///
///     If a ring, strip or dial action in the new mode differs from the action
///     in the previous mode, the client should immediately issue a
///     zwp_tablet_ring_v2.set_feedback, zwp_tablet_strip_v2.set_feedback or
///     zwp_tablet_dial_v2.set_feedback request for each changed ring, strip or dial.
///       </description>
///       <arg name="time" type="uint" summary="the time of the event with millisecond granularity" />
///       <arg name="serial" type="uint" />
///       <arg name="mode" type="uint" summary="the new mode of the pad" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTabletPadGroupV2ModeSwitchEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the time of the event with millisecond granularity.
    pub time: u32,
    /// Wire argument `serial`.
    pub serial: u32,
    /// the new mode of the pad.
    pub mode: u32,
}
impl ZwpTabletPadGroupV2ModeSwitchEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 5;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTabletPadGroupV2ModeSwitchEvent {
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
impl InterfaceReader for ZwpTabletPadGroupV2ModeSwitchEvent {
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
        let time = reader.next_u32().await?;
        let serial = reader.next_u32().await?;
        let mode = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            time,
            serial,
            mode,
        })
    }
}
impl CommandWriter for ZwpTabletPadGroupV2ModeSwitchEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4)])
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
        writer.next_u32(self.time).await?;
        writer.next_u32(self.serial).await?;
        writer.next_u32(self.mode).await?;
        Ok(())
    }
}
