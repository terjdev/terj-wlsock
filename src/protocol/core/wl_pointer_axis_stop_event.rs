use super::WlPointerAxis;
use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_pointer.axis_stop`.
///
/// Opcode 7; available since version 5. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="axis_stop" since="5">
///       <description summary="axis stop event">
///         Stop notification for scroll and other axes.
///
///         For some wl_pointer.axis_source types, a wl_pointer.axis_stop event
///         is sent to notify a client that the axis sequence has terminated.
///         This enables the client to implement kinetic scrolling.
///         See the wl_pointer.axis_source documentation for information on when
///         this event may be generated.
///
///         Any wl_pointer.axis events with the same axis_source after this
///         event should be considered as the start of a new axis motion.
///
///         The timestamp is to be interpreted identical to the timestamp in the
///         wl_pointer.axis event. The timestamp value may be the same as a
///         preceding wl_pointer.axis event.
///       </description>
///       <arg name="time" type="uint" summary="timestamp with millisecond granularity" />
///       <arg name="axis" type="uint" enum="axis" summary="the axis stopped with this event" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlPointerAxisStopEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `time`: timestamp with millisecond granularity.
    pub time: u32,
    /// Argument `axis`: the axis stopped with this event.
    pub axis: WlPointerAxis,
}
impl WlPointerAxisStopEvent {
    /// Opcode in the event namespace of interface wl_pointer.
    pub const OPCODE: u16 = 7;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 5;
}
impl CommandWriter for WlPointerAxisStopEvent {
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
        writer.next_u32(self.time).await?;
        writer.next_u32(self.axis.0).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlPointerAxisStopEvent {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Self::reader_event_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_pointer",
            Self::SINCE,
        )?;
        let time = reader.next_u32().await?;
        let axis = WlPointerAxis::read(object_id, opcode, reader, state).await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            time,
            axis,
        })
    }
}
