use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_fixes.destroy_registry`.
///
/// Opcode 1; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="destroy_registry">
///       <description summary="destroy a wl_registry">
///         This request destroys a wl_registry object.
///
///         The client should no longer use the wl_registry after making this
///         request.
///
///         The compositor will emit a wl_display.delete_id event with the object ID
///         of the registry and will no longer emit any events on the registry. The
///         client should re-use the object ID once it receives the
///         wl_display.delete_id event.
///       </description>
///       <arg name="registry" type="object" interface="wl_registry" summary="the registry to destroy" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlFixesDestroyRegistryRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `registry`: the registry to destroy.
    /// Object implementing interface wl_registry.
    pub registry: u32,
}
impl WlFixesDestroyRegistryRequest {
    /// Opcode in the request namespace of interface wl_fixes.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlFixesDestroyRegistryRequest {
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
            "wl_fixes",
            Self::SINCE,
        )?;
        let registry = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            registry,
        })
    }
}
