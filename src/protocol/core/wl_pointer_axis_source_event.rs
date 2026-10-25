use super::WlPointerAxisSource;
use crate::protocol::ClientState;
use crate::protocol::core::ObjectInteface as CoreObjectInteface;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_pointer.axis_source`.
///
/// Opcode 6; available since version 5. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="axis_source" since="5">
///       <description summary="axis source event">
///         Source information for scroll and other axes.
///
///         This event does not occur on its own. It is sent before a
///         wl_pointer.frame event and carries the source information for
///         all events within that frame.
///
///         The source specifies how this event was generated. If the source is
///         wl_pointer.axis_source.finger, a wl_pointer.axis_stop event will be
///         sent when the user lifts the finger off the device.
///
///         If the source is wl_pointer.axis_source.wheel,
///         wl_pointer.axis_source.wheel_tilt or
///         wl_pointer.axis_source.continuous, a wl_pointer.axis_stop event may
///         or may not be sent. Whether a compositor sends an axis_stop event
///         for these sources is hardware-specific and implementation-dependent;
///         clients must not rely on receiving an axis_stop event for these
///         scroll sources and should treat scroll sequences from these scroll
///         sources as unterminated by default.
///
///         This event is optional. If the source is unknown for a particular
///         axis event sequence, no event is sent.
///         Only one wl_pointer.axis_source event is permitted per frame.
///
///         The order of wl_pointer.axis_discrete and wl_pointer.axis_source is
///         not guaranteed.
///       </description>
///       <arg name="axis_source" type="uint" enum="axis_source" summary="source of the axis event" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlPointerAxisSourceEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `axis_source`: source of the axis event.
    pub axis_source: WlPointerAxisSource,
}
impl WlPointerAxisSourceEvent {
    /// Opcode in the event namespace of interface wl_pointer.
    pub const OPCODE: u16 = 6;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 5;
}
impl CommandWriter for WlPointerAxisSourceEvent {
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
        Self::writer_event_interface(state, self.object_id, "wl_pointer", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.axis_source.0).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlPointerAxisSourceEvent {
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

impl InterfaceReader for WlPointerAxisSourceEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Core(CoreObjectInteface::WlPointer { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let axis_source = WlPointerAxisSource::read(object_id, opcode, reader, state).await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            axis_source,
        })
    }
}
