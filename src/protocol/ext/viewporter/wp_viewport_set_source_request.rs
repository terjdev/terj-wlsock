use crate::protocol::ClientState;
use crate::protocol::ext::viewporter::ObjectInteface as ViewporterObjectInteface;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_viewport.set_source`.
///
/// Opcode 1; available since version 1.
/// Receiver metadata and complete payload framing are checked.
/// Object arguments, surface state, and compositor policy belong to the caller.
/// Fixed arguments use signed 24.8 encoding, represented here as `f64`.
/// Encoding rejects non-finite or unrepresentable values before writing arguments.
/// ```xml
/// <request name="set_source">
///       <description summary="set the source rectangle for cropping">
///     Set the source rectangle of the associated wl_surface. See
///     wp_viewport for the description, and relation to the wl_buffer
///     size.
///
///     If all of x, y, width and height are -1.0, the source rectangle is
///     unset instead. Any other set of values where width or height are zero
///     or negative, or x or y are negative, raise the bad_value protocol
///     error.
///
///     The crop and scale state is double-buffered, see wl_surface.commit.
///       </description>
///       <arg name="x" type="fixed" summary="source rectangle x" />
///       <arg name="y" type="fixed" summary="source rectangle y" />
///       <arg name="width" type="fixed" summary="source rectangle width" />
///       <arg name="height" type="fixed" summary="source rectangle height" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpViewportSetSourceRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Source rectangle x.
    pub x: f64,
    /// Source rectangle y.
    pub y: f64,
    /// Source rectangle width.
    pub width: f64,
    /// Source rectangle height.
    pub height: f64,
}
impl WpViewportSetSourceRequest {
    /// Opcode in the request namespace of interface wp_viewport.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpViewportSetSourceRequest {
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

impl InterfaceReader for WpViewportSetSourceRequest {
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
        let x = reader.next_fixed().await?;
        let y = reader.next_fixed().await?;
        let width = reader.next_fixed().await?;
        let height = reader.next_fixed().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            x,
            y,
            width,
            height,
        })
    }
}

impl CommandWriter for WpViewportSetSourceRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4), Some(4)])
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
        if !FdWriter::is_valid_fixed(self.x) {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.y) {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.width) {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.height) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_fixed(self.x).await?;
        writer.next_fixed(self.y).await?;
        writer.next_fixed(self.width).await?;
        writer.next_fixed(self.height).await?;
        Ok(())
    }
}
