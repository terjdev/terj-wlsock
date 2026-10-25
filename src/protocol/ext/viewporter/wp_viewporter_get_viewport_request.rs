use crate::protocol::ClientState;
use crate::protocol::ext::viewporter::ObjectInteface as ViewporterObjectInteface;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_viewporter.get_viewport`.
///
/// Opcode 1; available since version 1.
/// Receiver metadata and complete payload framing are checked.
/// Object arguments, surface state, and compositor policy belong to the caller.
/// The viewport ID inherits the receiver version and is registered before payload writes.
/// A failed or cancelled write after registration requires closing the connection.
/// ```xml
/// <request name="get_viewport">
///       <description summary="extend surface interface for crop and scale">
///     Instantiate an interface extension for the given wl_surface to
///     crop and scale its content. If the given wl_surface already has
///     a wp_viewport object associated, the viewport_exists
///     protocol error is raised.
///       </description>
///       <arg name="id" type="new_id" interface="wp_viewport" summary="the new viewport interface id" />
///       <arg name="surface" type="object" interface="wl_surface" summary="the surface" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpViewporterGetViewportRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// New object implementing `wp_viewport`.
    pub id: u32,
    /// Surface implementing `wl_surface`.
    pub surface: u32,
}
impl WpViewporterGetViewportRequest {
    /// Opcode in the request namespace of `wp_viewporter`.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpViewporterGetViewportRequest {
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

impl InterfaceReader for WpViewporterGetViewportRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Viewporter(ViewporterObjectInteface::WpViewporter { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = reader.next_u32().await?;
        let surface = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let mut state = Self::reader_state(state)?;
            let new_interface = ObjectInteface::from_name("wp_viewport", interface.version())
                .ok_or(FdReaderError::InvalidProtocol)?;
            state
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            surface,
        })
    }
}

impl CommandWriter for WpViewporterGetViewportRequest {
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
        let receiver_interface =
            Self::writer_request_interface(state, self.object_id, "wp_viewporter", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let mut state = Self::writer_state(state)?;
            let new_interface =
                ObjectInteface::from_name("wp_viewport", receiver_interface.version())
                    .ok_or(FdWriterError::InvalidProtocol)?;
            state
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_u32(self.surface).await?;
        Ok(())
    }
}
