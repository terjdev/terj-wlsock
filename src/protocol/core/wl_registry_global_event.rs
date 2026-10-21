use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_registry.global`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="global">
///       <description summary="announce global object">
///         Notify the client of global objects.
///
///         The event notifies the client that a global object with
///         the given name is now available, and it implements the
///         given version of the given interface.
///       </description>
///       <arg name="name" type="uint" summary="numeric name of the global object" />
///       <arg name="interface" type="string" summary="interface implemented by the object" />
///       <arg name="version" type="uint" summary="interface version" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlRegistryGlobalEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `name`: numeric name of the global object.
    pub name: u32,
    /// Argument `interface`: interface implemented by the object.
    pub interface: String,
    /// Argument `version`: interface version.
    pub version: u32,
}
impl WlRegistryGlobalEvent {
    /// Opcode in the event namespace of interface wl_registry.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlRegistryGlobalEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[
            Some(4),
            FdWriter::encoded_string_size(Some(&self.interface)),
            Some(4),
        ])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::event_interface(state, self.object_id, "wl_registry", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.interface) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.name).await?;
        writer.next_string(&self.interface).await?;
        writer.next_u32(self.version).await?;
        Ok(())
    }
}
