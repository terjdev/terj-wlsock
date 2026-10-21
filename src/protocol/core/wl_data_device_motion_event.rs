use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_data_device.motion`.
///
/// Opcode 3; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="motion">
///       <description summary="drag-and-drop session motion">
///         This event is sent when the drag-and-drop pointer moves within
///         the currently focused surface. The new position of the pointer
///         is provided by the x and y arguments, in surface-local
///         coordinates.
///       </description>
///       <arg name="time" type="uint" summary="timestamp with millisecond granularity" />
///       <arg name="x" type="fixed" summary="surface-local x coordinate" />
///       <arg name="y" type="fixed" summary="surface-local y coordinate" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlDataDeviceMotionEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `time`: timestamp with millisecond granularity.
    pub time: u32,
    /// Argument `x`: surface-local x coordinate.
    pub x: f64,
    /// Argument `y`: surface-local y coordinate.
    pub y: f64,
}
impl WlDataDeviceMotionEvent {
    /// Opcode in the event namespace of interface wl_data_device.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlDataDeviceMotionEvent {
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
        Self::event_interface(state, self.object_id, "wl_data_device", Self::SINCE)?;
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
        writer.next_fixed(self.x).await?;
        writer.next_fixed(self.y).await?;
        Ok(())
    }
}
