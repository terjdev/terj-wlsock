use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_registry.global_remove`.
///
/// Opcode 1; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="global_remove">
///       <description summary="announce removal of global object">
///         Notify the client of removed global objects.
///
///         This event notifies the client that the global identified
///         by name is no longer available.  If the client bound to
///         the global using the bind request, the client should now
///         destroy that object.
///
///         The object remains valid and requests to the object will be
///         ignored until the client destroys it, to avoid races between
///         the global going away and a client sending a request to it.
///       </description>
///       <arg name="name" type="uint" summary="numeric name of the global object" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlRegistryGlobalRemoveEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `name`: numeric name of the global object.
    pub name: u32,
}
impl WlRegistryGlobalRemoveEvent {
    /// Opcode in the event namespace of interface wl_registry.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlRegistryGlobalRemoveEvent {
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
        Self::writer_event_interface(state, self.object_id, "wl_registry", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.name).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlRegistryGlobalRemoveEvent {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Self::reader_event_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_registry",
            Self::SINCE,
        )?;
        let name = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, name })
    }
}
