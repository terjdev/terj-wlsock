use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_seat.get_touch`.
///
/// Opcode 2; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="get_touch">
///       <description summary="return touch object">
///         The ID provided will be initialized to the wl_touch interface
///         for this seat.
///
///         This request only takes effect if the seat has the touch
///         capability, or has had the touch capability in the past.
///         It is a protocol violation to issue this request on a seat that has
///         never had the touch capability. The missing_capability error will
///         be sent in this case.
///       </description>
///       <arg name="id" type="new_id" interface="wl_touch" summary="seat touch interface" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlSeatGetTouchRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `id`: seat touch interface.
    /// Object implementing interface wl_touch.
    pub id: u32,
}
impl WlSeatGetTouchRequest {
    /// Opcode in the request namespace of interface wl_seat.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlSeatGetTouchRequest {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        let receiver_interface = Self::request_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_seat",
            Self::SINCE,
        )?;
        let id = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let mut state = Self::read_state(state)?;
            let new_interface =
                ObjectInteface::from_name("wl_touch", receiver_interface.version().min(11))
                    .ok_or(FdReaderError::InvalidProtocol)?;
            state
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self { object_id, id })
    }
}
