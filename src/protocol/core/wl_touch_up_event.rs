use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_touch.up`.
///
/// Opcode 1; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="up">
///       <description summary="end of a touch event sequence">
///         The touch point has disappeared. No further events will be sent for
///         this touch point and the touch point's ID is released and may be
///         reused in a future touch down event.
///       </description>
///       <arg name="serial" type="uint" summary="serial number of the touch up event" />
///       <arg name="time" type="uint" summary="timestamp with millisecond granularity" />
///       <arg name="id" type="int" summary="the unique ID of this touch point" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlTouchUpEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `serial`: serial number of the touch up event.
    pub serial: u32,
    /// Argument `time`: timestamp with millisecond granularity.
    pub time: u32,
    /// Argument `id`: the unique ID of this touch point.
    pub id: i32,
}
impl WlTouchUpEvent {
    /// Opcode in the event namespace of interface wl_touch.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlTouchUpEvent {
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
        Self::event_interface(state, self.object_id, "wl_touch", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        writer.next_u32(self.time).await?;
        writer.next_i32(self.id).await?;
        Ok(())
    }
}
