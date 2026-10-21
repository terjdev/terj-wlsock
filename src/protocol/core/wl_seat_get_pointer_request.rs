use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_seat.get_pointer`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="get_pointer">
///       <description summary="return pointer object">
///         The ID provided will be initialized to the wl_pointer interface
///         for this seat.
///
///         This request only takes effect if the seat has the pointer
///         capability, or has had the pointer capability in the past.
///         It is a protocol violation to issue this request on a seat that has
///         never had the pointer capability. The missing_capability error will
///         be sent in this case.
///       </description>
///       <arg name="id" type="new_id" interface="wl_pointer" summary="seat pointer" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlSeatGetPointerRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `id`: seat pointer.
    /// Object implementing interface wl_pointer.
    pub id: u32,
}
impl WlSeatGetPointerRequest {
    /// Opcode in the request namespace of interface wl_seat.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlSeatGetPointerRequest {
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
                ObjectInteface::from_name("wl_pointer", receiver_interface.version().min(11))
                    .ok_or(FdReaderError::InvalidProtocol)?;
            state
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self { object_id, id })
    }
}
