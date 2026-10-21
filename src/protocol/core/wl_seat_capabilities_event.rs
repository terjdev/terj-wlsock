use super::WlSeatCapability;
use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_seat.capabilities`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="capabilities">
///       <description summary="seat capabilities changed">
///         This is sent on binding to the seat global or whenever a seat gains
///         or loses the pointer, keyboard or touch capabilities.
///         The argument is a capability enum containing the complete set of
///         capabilities this seat has.
///
///         When the pointer capability is added, a client may create a
///         wl_pointer object using the wl_seat.get_pointer request. This object
///         will receive pointer events until the capability is removed in the
///         future.
///
///         When the pointer capability is removed, a client should destroy the
///         wl_pointer objects associated with the seat where the capability was
///         removed, using the wl_pointer.release request. No further pointer
///         events will be received on these objects.
///
///         In some compositors, if a seat regains the pointer capability and a
///         client has a previously obtained wl_pointer object of version 4 or
///         less, that object may start sending pointer events again. This
///         behavior is considered a misinterpretation of the intended behavior
///         and must not be relied upon by the client. wl_pointer objects of
///         version 5 or later must not send events if created before the most
///         recent event notifying the client of an added pointer capability.
///
///         The above behavior also applies to wl_keyboard and wl_touch with the
///         keyboard and touch capabilities, respectively.
///       </description>
///       <arg name="capabilities" type="uint" enum="capability" summary="capabilities of the seat" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlSeatCapabilitiesEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `capabilities`: capabilities of the seat.
    pub capabilities: WlSeatCapability,
}
impl WlSeatCapabilitiesEvent {
    /// Opcode in the event namespace of interface wl_seat.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlSeatCapabilitiesEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::event_interface(state, self.object_id, "wl_seat", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.capabilities.0).await?;
        Ok(())
    }
}
