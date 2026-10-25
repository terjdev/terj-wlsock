use super::WlOutputTransform;
use crate::protocol::ClientState;
use crate::protocol::core::ObjectInteface as CoreObjectInteface;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wl_surface.set_buffer_transform`.
///
/// Opcode 7; available since version 2. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire framing and receiver metadata; object semantics belong to the compositor.
/// The caller handles compositor policy and message contents.
/// Client encoding checks the complete payload quota before writing arguments.
/// New object IDs are registered before payload I/O; client IDs remain reserved
/// after destruction until the server acknowledges them with delete_id.
/// ```xml
/// <request name="set_buffer_transform" since="2">
///       <description summary="sets the buffer transformation">
///         This request sets the transformation that the client has already applied
///         to the content of the buffer. The accepted values for the transform
///         parameter are the values for wl_output.transform.
///
///         The compositor applies the inverse of this transformation whenever it
///         uses the buffer contents.
///
///         Buffer transform is double-buffered state, see wl_surface.commit.
///
///         A newly created surface has its buffer transformation set to normal.
///
///         wl_surface.set_buffer_transform changes the pending buffer
///         transformation. wl_surface.commit copies the pending buffer
///         transformation to the current one. Otherwise, the pending and current
///         values are never changed.
///
///         The purpose of this request is to allow clients to render content
///         according to the output transform, thus permitting the compositor to
///         use certain optimizations even if the display is rotated. Using
///         hardware overlays and scanning out a client buffer for fullscreen
///         surfaces are examples of such optimizations. Those optimizations are
///         highly dependent on the compositor implementation, so the use of this
///         request should be considered on a case-by-case basis.
///
///         Note that if the transform value includes 90 or 270 degree rotation,
///         the width of the buffer will become the surface height and the height
///         of the buffer will become the surface width.
///
///         If transform is not one of the values from the
///         wl_output.transform enum the invalid_transform protocol error
///         is raised.
///       </description>
///       <arg name="transform" type="int" enum="wl_output.transform" summary="transform for interpreting buffer contents" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlSurfaceSetBufferTransformRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `transform`: transform for interpreting buffer contents.
    pub transform: WlOutputTransform,
}
impl WlSurfaceSetBufferTransformRequest {
    /// Opcode in the request namespace of interface wl_surface.
    pub const OPCODE: u16 = 7;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 2;
}
impl CommandReader<Self> for WlSurfaceSetBufferTransformRequest {
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

impl InterfaceReader for WlSurfaceSetBufferTransformRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Core(CoreObjectInteface::WlSurface { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let transform = WlOutputTransform(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            transform,
        })
    }
}

impl CommandWriter for WlSurfaceSetBufferTransformRequest {
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
        Self::writer_request_interface(state, self.object_id, "wl_surface", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.transform.0).await?;
        Ok(())
    }
}
