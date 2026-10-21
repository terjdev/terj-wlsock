use super::WlRegistryBindId;
use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_registry.bind`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="bind">
///       <description summary="bind an object to the display">
///         Binds a new, client-created object to the server using the
///         specified name as the identifier.
///       </description>
///       <arg name="name" type="uint" summary="unique numeric name of the object" />
///       <arg name="id" type="new_id" summary="bound object" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlRegistryBindRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `name`: unique numeric name of the object.
    pub name: u32,
    /// Argument `id`: bound object.
    pub id: WlRegistryBindId,
}
impl WlRegistryBindRequest {
    /// Opcode in the request namespace of interface wl_registry.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlRegistryBindRequest {
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
            "wl_registry",
            Self::SINCE,
        )?;
        let name = reader.next_u32().await?;
        let id = WlRegistryBindId {
            interface: reader.next_string().await?,
            version: reader.next_u32().await?,
            id: reader.next_u32().await?,
        };
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let mut state = Self::read_state(state)?;
            let new_interface = ObjectInteface::from_name(&id.interface, id.version)
                .ok_or(FdReaderError::InvalidProtocol)?;
            state
                .register_object(id.id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            name,
            id,
        })
    }
}
