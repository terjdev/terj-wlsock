use crate::protocol::ClientState;
use crate::protocol::InterfaceReader;
use crate::protocol::{CommandReader, CommandReaderExt, ObjectInteface};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wl_display.get_registry`.
///
/// Opcode 1; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire framing and receiver metadata; object semantics belong to the compositor.
/// The caller handles compositor policy and message contents.
/// Client encoding checks the complete payload quota before writing arguments.
/// New object IDs are registered before payload I/O; client IDs remain reserved
/// after destruction until the server acknowledges them with delete_id.
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
        let interface = Self::reader_state(state)?
            .object_interface(object_id)
            .ok_or(FdReaderError::InvalidProtocol)?;
        Self::read_with_interface(object_id, opcode, interface, reader, state).await
    }
}

impl InterfaceReader for WlDisplayGetRegistryRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(interface, ObjectInteface::WlDisplay { .. })
            || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let receiver_interface = interface;
        let registry = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let mut state = Self::reader_state(state)?;
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

impl CommandWriter for WlDisplayGetRegistryRequest {
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
        let receiver_interface =
            Self::writer_request_interface(state, self.object_id, "wl_display", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let mut state = Self::writer_state(state)?;
            let new_interface =
                ObjectInteface::from_name("wl_registry", receiver_interface.version().min(1))
                    .ok_or(FdWriterError::InvalidProtocol)?;
            state
                .register_object(self.registry, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.registry).await?;
        Ok(())
    }
}
