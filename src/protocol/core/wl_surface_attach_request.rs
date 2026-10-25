use crate::protocol::ClientState;
use crate::protocol::core::ObjectInteface as CoreObjectInteface;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wl_surface.attach`.
///
/// Opcode 1; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire framing and receiver metadata; object semantics belong to the compositor.
/// The caller handles compositor policy and message contents.
/// Client encoding checks the complete payload quota before writing arguments.
/// New object IDs are registered before payload I/O; client IDs remain reserved
/// after destruction until the server acknowledges them with delete_id.
/// ```xml
/// <request name="attach">
///       <description summary="set the surface contents">
///         Set a buffer as the content of this surface.
///
///         The new size of the surface is calculated based on the buffer
///         size transformed by the inverse buffer_transform and the
///         inverse buffer_scale. This means that at commit time the supplied
///         buffer size must be an integer multiple of the buffer_scale. If
///         that's not the case, an invalid_size error is sent.
///
///         The x and y arguments specify the location of the new pending
///         buffer's upper left corner, relative to the current buffer's upper
///         left corner, in surface-local coordinates. In other words, the
///         x and y, combined with the new surface size define in which
///         directions the surface's size changes. Setting anything other than 0
///         as x and y arguments is discouraged, and should instead be replaced
///         with using the separate wl_surface.offset request.
///
///         When the bound wl_surface version is 5 or higher, passing any
///         non-zero x or y is a protocol violation, and will result in an
///         'invalid_offset' error being raised. The x and y arguments are ignored
///         and do not change the pending state. To achieve equivalent semantics,
///         use wl_surface.offset.
///
///         Surface contents are double-buffered state, see wl_surface.commit.
///
///         The initial surface contents are void; there is no content.
///         wl_surface.attach assigns the given wl_buffer as the pending
///         wl_buffer. wl_surface.commit makes the pending wl_buffer the new
///         surface contents, and the size of the surface becomes the size
///         calculated from the wl_buffer, as described above. After commit,
///         there is no pending buffer until the next attach.
///
///         Committing a pending wl_buffer allows the compositor to read the
///         pixels in the wl_buffer. The compositor may access the pixels at
///         any time after the wl_surface.commit request. When the compositor
///         will not access the pixels anymore, it will send the
///         wl_buffer.release event. Only after receiving wl_buffer.release,
///         the client may reuse the wl_buffer. A wl_buffer that has been
///         attached and then replaced by another attach instead of committed
///         will not receive a release event, and is not used by the
///         compositor.
///
///         If a pending wl_buffer has been committed to more than one wl_surface,
///         the delivery of wl_buffer.release events becomes undefined. A well
///         behaved client should not rely on wl_buffer.release events in this
///         case. Instead, clients hitting this case should use
///         wl_surface.get_release or use a protocol extension providing per-commit
///         release notifications (if none of these options are available, a
///         fallback can be implemented by creating multiple wl_buffer objects from
///         the same backing storage).
///
///         Destroying the wl_buffer after wl_buffer.release does not change
///         the surface contents. Destroying the wl_buffer before wl_buffer.release
///         is allowed as long as the underlying buffer storage isn't re-used (this
///         can happen e.g. on client process termination). However, if the client
///         destroys the wl_buffer before receiving the wl_buffer.release event and
///         mutates the underlying buffer storage, the surface contents become
///         undefined immediately.
///
///         If wl_surface.attach is sent with a NULL wl_buffer, the
///         following wl_surface.commit will remove the surface content.
///
///         If a pending wl_buffer has been destroyed, the result is not specified.
///         Many compositors are known to remove the surface content on the following
///         wl_surface.commit, but this behaviour is not universal. Clients seeking to
///         maximise compatibility should not destroy pending buffers and should
///         ensure that they explicitly remove content from surfaces, even after
///         destroying buffers.
///       </description>
///       <arg name="buffer" type="object" interface="wl_buffer" allow-null="true" summary="buffer of surface contents" />
///       <arg name="x" type="int" summary="surface-local x coordinate" />
///       <arg name="y" type="int" summary="surface-local y coordinate" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlSurfaceAttachRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `buffer`: buffer of surface contents.
    /// The protocol permits NULL.
    /// Object implementing interface wl_buffer.
    pub buffer: Option<u32>,
    /// Argument `x`: surface-local x coordinate.
    pub x: i32,
    /// Argument `y`: surface-local y coordinate.
    pub y: i32,
}
impl WlSurfaceAttachRequest {
    /// Opcode in the request namespace of interface wl_surface.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlSurfaceAttachRequest {
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

impl InterfaceReader for WlSurfaceAttachRequest {
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
        let buffer = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        let x = reader.next_i32().await?;
        let y = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            buffer,
            x,
            y,
        })
    }
}

impl CommandWriter for WlSurfaceAttachRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4)])
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
        writer.next_u32(self.buffer.unwrap_or(0)).await?;
        writer.next_i32(self.x).await?;
        writer.next_i32(self.y).await?;
        Ok(())
    }
}
