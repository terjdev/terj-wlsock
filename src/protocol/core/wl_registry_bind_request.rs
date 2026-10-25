use super::WlRegistryBindId;
use crate::protocol::ClientState;
use crate::protocol::InterfaceReader;
use crate::protocol::core::ObjectInteface as CoreObjectInteface;
use crate::protocol::{CommandReader, CommandReaderExt, ObjectInteface};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wl_registry.bind`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire framing and receiver metadata; object semantics belong to the compositor.
/// The caller handles compositor policy and message contents.
/// Client encoding checks the complete payload quota before writing arguments.
/// New object IDs are registered before payload I/O; client IDs remain reserved
/// after destruction until the server acknowledges them with delete_id.
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
        let interface = Self::reader_state(state)?
            .object_interface(object_id)
            .ok_or(FdReaderError::InvalidProtocol)?;
        Self::read_with_interface(object_id, opcode, interface, reader, state).await
    }
}

impl InterfaceReader for WlRegistryBindRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Core(CoreObjectInteface::WlRegistry { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
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
            let mut state = Self::reader_state(state)?;
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

impl CommandWriter for WlRegistryBindRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(self.id.payload_size())])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(state, self.object_id, "wl_registry", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.id.interface) {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let mut state = Self::writer_state(state)?;
            let new_interface = ObjectInteface::from_name(&self.id.interface, self.id.version)
                .ok_or(FdWriterError::InvalidProtocol)?;
            state
                .register_object(self.id.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.name).await?;
        self.id.write(writer, state).await?;
        Ok(())
    }
}
