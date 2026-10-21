use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_data_device_manager.release`.
///
/// Opcode 2; available since version 4. The message ends the receiver's lifetime.
/// The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="release" type="destructor" since="4">
///       <description summary="destroy wl_data_device_manager">
///         This request destroys the wl_data_device_manager. This has no effect on any other
///         objects.
///       </description>
///     </request>
/// ```
#[derive(Debug)]
pub struct WlDataDeviceManagerReleaseRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
}
impl WlDataDeviceManagerReleaseRequest {
    /// Opcode in the request namespace of interface wl_data_device_manager.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 4;
}
impl CommandReader<Self> for WlDataDeviceManagerReleaseRequest {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Self::request_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_data_device_manager",
            Self::SINCE,
        )?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let mut state = Self::read_state(state)?;
            state.release_server_id(object_id);
        }
        Ok(Self { object_id })
    }
}
