use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_touch.motion`.
///
/// Opcode 2; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="motion">
///       <description summary="update of touch point coordinates">
///         A touch point has changed coordinates.
///       </description>
///       <arg name="time" type="uint" summary="timestamp with millisecond granularity" />
///       <arg name="id" type="int" summary="the unique ID of this touch point" />
///       <arg name="x" type="fixed" summary="surface-local x coordinate" />
///       <arg name="y" type="fixed" summary="surface-local y coordinate" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlTouchMotionEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `time`: timestamp with millisecond granularity.
    pub time: u32,
    /// Argument `id`: the unique ID of this touch point.
    pub id: i32,
    /// Argument `x`: surface-local x coordinate.
    pub x: f64,
    /// Argument `y`: surface-local y coordinate.
    pub y: f64,
}
impl WlTouchMotionEvent {
    /// Opcode in the event namespace of interface wl_touch.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlTouchMotionEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4), Some(4)])
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
        if !FdWriter::is_valid_fixed(self.x) {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.y) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.time).await?;
        writer.next_i32(self.id).await?;
        writer.next_fixed(self.x).await?;
        writer.next_fixed(self.y).await?;
        Ok(())
    }
}
