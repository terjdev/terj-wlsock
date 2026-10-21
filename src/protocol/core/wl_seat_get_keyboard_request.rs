use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_seat.get_keyboard`.
///
/// Opcode 1; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="get_keyboard">
///       <description summary="return keyboard object">
///         The ID provided will be initialized to the wl_keyboard interface
///         for this seat.
///
///         This request only takes effect if the seat has the keyboard
///         capability, or has had the keyboard capability in the past.
///         It is a protocol violation to issue this request on a seat that has
///         never had the keyboard capability. The missing_capability error will
///         be sent in this case.
///       </description>
///       <arg name="id" type="new_id" interface="wl_keyboard" summary="seat keyboard" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlSeatGetKeyboardRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `id`: seat keyboard.
    /// Object implementing interface wl_keyboard.
    pub id: u32,
}
impl WlSeatGetKeyboardRequest {
    /// Opcode in the request namespace of interface wl_seat.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlSeatGetKeyboardRequest {
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
                ObjectInteface::from_name("wl_keyboard", receiver_interface.version().min(11))
                    .ok_or(FdReaderError::InvalidProtocol)?;
            state
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self { object_id, id })
    }
}
