use super::WlPointerAxis;
use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_pointer.axis_value120`.
///
/// Opcode 9; available since version 8. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="axis_value120" since="8">
///       <description summary="axis high-resolution scroll event">
///         Discrete high-resolution scroll information.
///
///         This event carries high-resolution wheel scroll information,
///         with each multiple of 120 representing one logical scroll step
///         (a wheel detent). For example, an axis_value120 of 30 is one quarter of
///         a logical scroll step in the positive direction, a value120 of
///         -240 are two logical scroll steps in the negative direction within the
///         same hardware event.
///         Clients that rely on discrete scrolling should accumulate the
///         value120 to multiples of 120 before processing the event.
///
///         The value120 must not be zero.
///
///         This event replaces the wl_pointer.axis_discrete event in clients
///         supporting wl_pointer version 8 or later.
///
///         Where a wl_pointer.axis_source event occurs in the same
///         wl_pointer.frame, the axis source applies to this event.
///
///         The order of wl_pointer.axis_value120 and wl_pointer.axis_source is
///         not guaranteed.
///       </description>
///       <arg name="axis" type="uint" enum="axis" summary="axis type" />
///       <arg name="value120" type="int" summary="scroll distance as fraction of 120" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlPointerAxisValue120Event {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `axis`: axis type.
    pub axis: WlPointerAxis,
    /// Argument `value120`: scroll distance as fraction of 120.
    pub value120: i32,
}
impl WlPointerAxisValue120Event {
    /// Opcode in the event namespace of interface wl_pointer.
    pub const OPCODE: u16 = 9;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 8;
}
impl CommandWriter for WlPointerAxisValue120Event {
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
        writer.next_i32(self.value120).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlPointerAxisValue120Event {
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
        let axis = WlPointerAxis::read(object_id, opcode, reader, state).await?;
        let value120 = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            axis,
            value120,
        })
    }
}
