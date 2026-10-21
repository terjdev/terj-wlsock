use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wl_fixes.ack_global_remove`.
///
/// Opcode 2; available since version 2. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire framing and receiver metadata; object semantics belong to the compositor.
/// The caller handles compositor policy and message contents.
/// Client encoding checks the complete payload quota before writing arguments.
/// New object IDs are registered before payload I/O; client IDs remain reserved
/// after destruction until the server acknowledges them with delete_id.
/// ```xml
/// <request name="ack_global_remove" since="2">
///       <description summary="acknowledge global removal">
///         Acknowledge the removal of the specified global.
///
///         If no global with the specified name exists or the global is not removed,
///         the wl_fixes.invalid_ack_remove protocol error will be posted.
///
///         Due to the Wayland protocol being asynchronous, the wl_global objects
///         cannot be destroyed immediately. For example, if a wl_global is removed
///         and a client attempts to bind that global around same time, it can
///         result in a protocol error due to an unknown global name in the bind
///         request.
///
///         In order to avoid crashing clients, the compositor should remove the
///         wl_global once it is guaranteed that no more bind requests will come.
///
///         The wl_fixes.ack_global_remove() request is used to signal to the
///         compositor that the client will not bind the given global anymore. After
///         all clients acknowledge the removal of the global, the compositor can
///         safely destroy it.
///
///         The client must call the wl_fixes.ack_global_remove() request in
///         response to a wl_registry.global_remove() event even if it did not bind
///         the corresponding global.
///       </description>
///       <arg name="registry" type="object" interface="wl_registry" summary="the registry object" />
///       <arg name="name" type="uint" summary="unique name of the global" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlFixesAckGlobalRemoveRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `registry`: the registry object.
    /// Object implementing interface wl_registry.
    pub registry: u32,
    /// Argument `name`: unique name of the global.
    pub name: u32,
}
impl WlFixesAckGlobalRemoveRequest {
    /// Opcode in the request namespace of interface wl_fixes.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 2;
}
impl CommandReader<Self> for WlFixesAckGlobalRemoveRequest {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Self::reader_request_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_fixes",
            Self::SINCE,
        )?;
        let registry = reader.next_u32().await?;
        let name = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            registry,
            name,
        })
    }
}

impl CommandWriter for WlFixesAckGlobalRemoveRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(state, self.object_id, "wl_fixes", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.registry).await?;
        writer.next_u32(self.name).await?;
        Ok(())
    }
}
