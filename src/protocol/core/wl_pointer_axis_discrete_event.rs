use super::WlPointerAxis;
use crate::protocol::ClientState;
use crate::protocol::core::ObjectInteface as CoreObjectInteface;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_pointer.axis_discrete`.
///
/// Opcode 8; available since version 5. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="axis_discrete" since="5" deprecated-since="8">
///       <description summary="axis click event">
///         Discrete step information for scroll and other axes.
///
///         This event carries the axis value of the wl_pointer.axis event in
///         discrete steps (e.g. mouse wheel clicks).
///
///         This event is deprecated with wl_pointer version 8 - this event is not
///         sent to clients supporting version 8 or later.
///
///         This event does not occur on its own, it is coupled with a
///         wl_pointer.axis event that represents this axis value on a
///         continuous scale. The protocol guarantees that each axis_discrete
///         event is always followed by exactly one axis event with the same
///         axis number within the same wl_pointer.frame. Note that the protocol
///         allows for other events to occur between the axis_discrete and
///         its coupled axis event, including other axis_discrete or axis
///         events. A wl_pointer.frame must not contain more than one axis_discrete
///         event per axis type.
///
///         This event is optional; continuous scrolling devices
///         like two-finger scrolling on touchpads do not have discrete
///         steps and do not generate this event.
///
///         The discrete value carries the directional information. e.g. a value
///         of -2 is two steps towards the negative direction of this axis.
///
///         The axis number is identical to the axis number in the associated
///         axis event.
///
///         The order of wl_pointer.axis_discrete and wl_pointer.axis_source is
///         not guaranteed.
///       </description>
///       <arg name="axis" type="uint" enum="axis" summary="axis type" />
///       <arg name="discrete" type="int" summary="number of steps" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlPointerAxisDiscreteEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `axis`: axis type.
    pub axis: WlPointerAxis,
    /// Argument `discrete`: number of steps.
    pub discrete: i32,
}
impl WlPointerAxisDiscreteEvent {
    /// Opcode in the event namespace of interface wl_pointer.
    pub const OPCODE: u16 = 8;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 5;
}
impl CommandWriter for WlPointerAxisDiscreteEvent {
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
        Self::writer_event_interface(state, self.object_id, "wl_pointer", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.axis.0).await?;
        writer.next_i32(self.discrete).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlPointerAxisDiscreteEvent {
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

impl InterfaceReader for WlPointerAxisDiscreteEvent {
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
        let axis = WlPointerAxis::read(object_id, opcode, reader, state).await?;
        let discrete = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            axis,
            discrete,
        })
    }
}
