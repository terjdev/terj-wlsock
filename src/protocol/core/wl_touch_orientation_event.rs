use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_touch.orientation`.
///
/// Opcode 6; available since version 6. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="orientation" since="6">
///       <description summary="update orientation of touch point">
///         Sent when a touchpoint has changed its orientation.
///
///         This event does not occur on its own. It is sent before a
///         wl_touch.frame event and carries the new shape information for
///         any previously reported, or new touch points of that frame.
///
///         Other events describing the touch point such as wl_touch.down,
///         wl_touch.motion or wl_touch.shape may be sent within the
///         same wl_touch.frame. A client should treat these events as a single
///         logical touch point update. The order of wl_touch.shape,
///         wl_touch.orientation and wl_touch.motion is not guaranteed.
///         A wl_touch.down event is guaranteed to occur before the first
///         wl_touch.orientation event for this touch ID but both events may occur
///         within the same wl_touch.frame.
///
///         The orientation describes the clockwise angle of a touchpoint's major
///         axis to the positive surface y-axis and is normalized to the -180 to
///         +180 degree range. The granularity of orientation depends on the touch
///         device, some devices only support binary rotation values between 0 and
///         90 degrees.
///
///         This event is only sent by the compositor if the touch device supports
///         orientation reports.
///       </description>
///       <arg name="id" type="int" summary="the unique ID of this touch point" />
///       <arg name="orientation" type="fixed" summary="angle between major axis and positive surface y-axis in degrees" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlTouchOrientationEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `id`: the unique ID of this touch point.
    pub id: i32,
    /// Argument `orientation`: angle between major axis and positive surface y-axis in degrees.
    pub orientation: f64,
}
impl WlTouchOrientationEvent {
    /// Opcode in the event namespace of interface wl_touch.
    pub const OPCODE: u16 = 6;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 6;
}
impl CommandWriter for WlTouchOrientationEvent {
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
        Self::event_interface(state, self.object_id, "wl_touch", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.orientation) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.id).await?;
        writer.next_fixed(self.orientation).await?;
        Ok(())
    }
}
