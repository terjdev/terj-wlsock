use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_display.get_registry`.
///
/// Opcode 1; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="get_registry">
///       <description summary="get global registry object">
///         This request creates a registry object that allows the client
///         to list and bind the global objects available from the
///         compositor.
///
///         It should be noted that the server side resources consumed in
///         response to a get_registry request can only be released when the
///         client disconnects, not when the client side proxy is destroyed.
///         Therefore, clients should invoke get_registry as infrequently as
///         possible to avoid wasting memory.
///       </description>
///       <arg name="registry" type="new_id" interface="wl_registry" summary="global registry object" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlDisplayGetRegistryRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `registry`: global registry object.
    /// Object implementing interface wl_registry.
    pub registry: u32,
}
impl WlDisplayGetRegistryRequest {
    /// Opcode in the request namespace of interface wl_display.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlDisplayGetRegistryRequest {
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
            "wl_display",
            Self::SINCE,
        )?;
        let registry = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let mut state = Self::read_state(state)?;
            let new_interface =
                ObjectInteface::from_name("wl_registry", receiver_interface.version().min(1))
                    .ok_or(FdReaderError::InvalidProtocol)?;
            state
                .register_object(registry, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            registry,
        })
    }
}
