use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_data_device_manager.get_data_device`.
///
/// Opcode 1; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="get_data_device">
///       <description summary="create a new data device">
///         Create a new data device for a given seat.
///       </description>
///       <arg name="id" type="new_id" interface="wl_data_device" summary="data device to create" />
///       <arg name="seat" type="object" interface="wl_seat" summary="seat associated with the data device" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlDataDeviceManagerGetDataDeviceRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `id`: data device to create.
    /// Object implementing interface wl_data_device.
    pub id: u32,
    /// Argument `seat`: seat associated with the data device.
    /// Object implementing interface wl_seat.
    pub seat: u32,
}
impl WlDataDeviceManagerGetDataDeviceRequest {
    /// Opcode in the request namespace of interface wl_data_device_manager.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlDataDeviceManagerGetDataDeviceRequest {
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
            "wl_data_device_manager",
            Self::SINCE,
        )?;
        let id = reader.next_u32().await?;
        let seat = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let mut state = Self::read_state(state)?;
            let new_interface =
                ObjectInteface::from_name("wl_data_device", receiver_interface.version().min(4))
                    .ok_or(FdReaderError::InvalidProtocol)?;
            state
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            seat,
        })
    }
}
