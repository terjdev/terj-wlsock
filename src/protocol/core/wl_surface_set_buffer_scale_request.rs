use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_surface.set_buffer_scale`.
///
/// Opcode 8; available since version 3. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="set_buffer_scale" since="3">
///       <description summary="sets the buffer scaling factor">
///         This request sets an optional scaling factor on how the compositor
///         interprets the contents of the buffer attached to the window.
///
///         Buffer scale is double-buffered state, see wl_surface.commit.
///
///         A newly created surface has its buffer scale set to 1.
///
///         wl_surface.set_buffer_scale changes the pending buffer scale.
///         wl_surface.commit copies the pending buffer scale to the current one.
///         Otherwise, the pending and current values are never changed.
///
///         The purpose of this request is to allow clients to supply higher
///         resolution buffer data for use on high resolution outputs. It is
///         intended that you pick the same buffer scale as the scale of the
///         output that the surface is displayed on. This means the compositor
///         can avoid scaling when rendering the surface on that output.
///
///         Note that if the scale is larger than 1, then you have to attach
///         a buffer that is larger (by a factor of scale in each dimension)
///         than the desired surface size.
///
///         If scale is not greater than 0 the invalid_scale protocol error is
///         raised.
///       </description>
///       <arg name="scale" type="int" summary="scale for interpreting buffer contents" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlSurfaceSetBufferScaleRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `scale`: scale for interpreting buffer contents.
    pub scale: i32,
}
impl WlSurfaceSetBufferScaleRequest {
    /// Opcode in the request namespace of interface wl_surface.
    pub const OPCODE: u16 = 8;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 3;
}
impl CommandReader<Self> for WlSurfaceSetBufferScaleRequest {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Self::request_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_surface",
            Self::SINCE,
        )?;
        let scale = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, scale })
    }
}
