use super::{WlPointerAxis, WlPointerAxisRelativeDirection};
use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_pointer.axis_relative_direction`.
///
/// Opcode 10; available since version 9. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="axis_relative_direction" since="9">
///       <description summary="axis relative physical direction event">
///         Relative directional information of the entity causing the axis
///         motion.
///
///         For a wl_pointer.axis event, the wl_pointer.axis_relative_direction
///         event specifies the movement direction of the entity causing the
///         wl_pointer.axis event. For example:
///         - if a user's fingers on a touchpad move down and this
///           causes a wl_pointer.axis vertical_scroll down event, the physical
///           direction is 'identical'
///         - if a user's fingers on a touchpad move down and this causes a
///           wl_pointer.axis vertical_scroll up scroll up event ('natural
///           scrolling'), the physical direction is 'inverted'.
///
///         A client may use this information to adjust scroll motion of
///         components. Specifically, enabling natural scrolling causes the
///         content to change direction compared to traditional scrolling.
///         Some widgets like volume control sliders should usually match the
///         physical direction regardless of whether natural scrolling is
///         active. This event enables clients to match the scroll direction of
///         a widget to the physical direction.
///
///         This event does not occur on its own, it is coupled with a
///         wl_pointer.axis event that represents this axis value.
///         The protocol guarantees that each axis_relative_direction event is
///         always followed by exactly one axis event with the same
///         axis number within the same wl_pointer.frame. Note that the protocol
///         allows for other events to occur between the axis_relative_direction
///         and its coupled axis event.
///
///         The axis number is identical to the axis number in the associated
///         axis event.
///
///         The order of wl_pointer.axis_relative_direction,
///         wl_pointer.axis_discrete and wl_pointer.axis_source is not
///         guaranteed.
///       </description>
///       <arg name="axis" type="uint" enum="axis" summary="axis type" />
///       <arg name="direction" type="uint" enum="axis_relative_direction" summary="physical direction relative to axis motion" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlPointerAxisRelativeDirectionEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `axis`: axis type.
    pub axis: WlPointerAxis,
    /// Argument `direction`: physical direction relative to axis motion.
    pub direction: WlPointerAxisRelativeDirection,
}
impl WlPointerAxisRelativeDirectionEvent {
    /// Opcode in the event namespace of interface wl_pointer.
    pub const OPCODE: u16 = 10;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 9;
}
impl CommandWriter for WlPointerAxisRelativeDirectionEvent {
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
        Self::event_interface(state, self.object_id, "wl_pointer", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.axis.0).await?;
        writer.next_u32(self.direction.0).await?;
        Ok(())
    }
}
