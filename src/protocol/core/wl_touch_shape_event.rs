use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_touch.shape`.
///
/// Opcode 5; available since version 6. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="shape" since="6">
///       <description summary="update shape of touch point">
///         Sent when a touchpoint has changed its shape.
///
///         This event does not occur on its own. It is sent before a
///         wl_touch.frame event and carries the new shape information for
///         any previously reported, or new touch points of that frame.
///
///         Other events describing the touch point such as wl_touch.down,
///         wl_touch.motion or wl_touch.orientation may be sent within the
///         same wl_touch.frame. A client should treat these events as a single
///         logical touch point update. The order of wl_touch.shape,
///         wl_touch.orientation and wl_touch.motion is not guaranteed.
///         A wl_touch.down event is guaranteed to occur before the first
///         wl_touch.shape event for this touch ID but both events may occur within
///         the same wl_touch.frame.
///
///         A touchpoint shape is approximated by an ellipse through the major and
///         minor axis length. The major axis length describes the longer diameter
///         of the ellipse, while the minor axis length describes the shorter
///         diameter. Major and minor are orthogonal and both are specified in
///         surface-local coordinates. The center of the ellipse is always at the
///         touchpoint location as reported by wl_touch.down or wl_touch.motion.
///
///         This event is only sent by the compositor if the touch device supports
///         shape reports. The client has to make reasonable assumptions about the
///         shape if it did not receive this event.
///       </description>
///       <arg name="id" type="int" summary="the unique ID of this touch point" />
///       <arg name="major" type="fixed" summary="length of the major axis in surface-local coordinates" />
///       <arg name="minor" type="fixed" summary="length of the minor axis in surface-local coordinates" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlTouchShapeEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `id`: the unique ID of this touch point.
    pub id: i32,
    /// Argument `major`: length of the major axis in surface-local coordinates.
    pub major: f64,
    /// Argument `minor`: length of the minor axis in surface-local coordinates.
    pub minor: f64,
}
impl WlTouchShapeEvent {
    /// Opcode in the event namespace of interface wl_touch.
    pub const OPCODE: u16 = 5;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 6;
}
impl CommandWriter for WlTouchShapeEvent {
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
        if !FdWriter::is_valid_fixed(self.major) {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.minor) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.id).await?;
        writer.next_fixed(self.major).await?;
        writer.next_fixed(self.minor).await?;
        Ok(())
    }
}
