use crate::protocol::ClientState;
use crate::protocol::ext::viewporter::ObjectInteface as ViewporterObjectInteface;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_viewport.set_destination`.
///
/// Opcode 2; available since version 1.
/// Receiver metadata and complete payload framing are checked.
/// Object arguments, surface state, and compositor policy belong to the caller.
/// ```xml
/// <request name="set_destination">
///       <description summary="set the surface size for scaling">
///     Set the destination size of the associated wl_surface. See
///     wp_viewport for the description, and relation to the wl_buffer
///     size.
///
///     If width is -1 and height is -1, the destination size is unset
///     instead. Any other pair of values for width and height that
///     contains zero or negative values raises the bad_value protocol
///     error.
///
///     The crop and scale state is double-buffered, see wl_surface.commit.
///       </description>
///       <arg name="width" type="int" summary="surface width" />
///       <arg name="height" type="int" summary="surface height" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpViewportSetDestinationRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Surface width.
    pub width: i32,
    /// Surface height.
    pub height: i32,
}
impl WpViewportSetDestinationRequest {
    /// Opcode in the request namespace of interface wp_viewport.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpViewportSetDestinationRequest {
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

impl InterfaceReader for WpViewportSetDestinationRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Viewporter(ViewporterObjectInteface::WpViewport { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let width = reader.next_i32().await?;
        let height = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            width,
            height,
        })
    }
}

impl CommandWriter for WpViewportSetDestinationRequest {
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
        Self::writer_request_interface(state, self.object_id, "wp_viewport", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.width).await?;
        writer.next_i32(self.height).await?;
        Ok(())
    }
}
