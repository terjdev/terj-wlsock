use super::WlPointerAxis;
use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_pointer.axis`.
///
/// Opcode 4; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="axis">
///       <description summary="axis event">
///         Scroll and other axis notifications.
///
///         For scroll events (vertical and horizontal scroll axes), the
///         value parameter is the length of a vector along the specified
///         axis in a coordinate space identical to those of motion events,
///         representing a relative movement along the specified axis.
///
///         For devices that support movements non-parallel to axes multiple
///         axis events will be emitted.
///
///         When applicable, for example for touch pads, the server can
///         choose to emit scroll events where the motion vector is
///         equivalent to a motion event vector.
///
///         When applicable, a client can transform its content relative to the
///         scroll distance.
///       </description>
///       <arg name="time" type="uint" summary="timestamp with millisecond granularity" />
///       <arg name="axis" type="uint" enum="axis" summary="axis type" />
///       <arg name="value" type="fixed" summary="length of vector in surface-local coordinate space" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlPointerAxisEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `time`: timestamp with millisecond granularity.
    pub time: u32,
    /// Argument `axis`: axis type.
    pub axis: WlPointerAxis,
    /// Argument `value`: length of vector in surface-local coordinate space.
    pub value: f64,
}
impl WlPointerAxisEvent {
    /// Opcode in the event namespace of interface wl_pointer.
    pub const OPCODE: u16 = 4;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlPointerAxisEvent {
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
        Self::event_interface(state, self.object_id, "wl_pointer", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.value) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.time).await?;
        writer.next_u32(self.axis.0).await?;
        writer.next_fixed(self.value).await?;
        Ok(())
    }
}
