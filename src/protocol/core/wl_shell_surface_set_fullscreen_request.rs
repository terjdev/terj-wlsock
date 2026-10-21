use super::WlShellSurfaceFullscreenMethod;
use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wl_shell_surface.set_fullscreen`.
///
/// Opcode 5; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire framing and receiver metadata; object semantics belong to the compositor.
/// The caller handles compositor policy and message contents.
/// Client encoding checks the complete payload quota before writing arguments.
/// New object IDs are registered before payload I/O; client IDs remain reserved
/// after destruction until the server acknowledges them with delete_id.
/// ```xml
/// <request name="set_fullscreen">
///       <description summary="make the surface a fullscreen surface">
///         Map the surface as a fullscreen surface.
///
///         If an output parameter is given then the surface will be made
///         fullscreen on that output. If the client does not specify the
///         output then the compositor will apply its policy - usually
///         choosing the output on which the surface has the biggest surface
///         area.
///
///         The client may specify a method to resolve a size conflict
///         between the output size and the surface size - this is provided
///         through the method parameter.
///
///         The framerate parameter is used only when the method is set
///         to "driver", to indicate the preferred framerate. A value of 0
///         indicates that the client does not care about framerate.  The
///         framerate is specified in mHz, that is framerate of 60000 is 60Hz.
///
///         A method of "scale" or "driver" implies a scaling operation of
///         the surface, either via a direct scaling operation or a change of
///         the output mode. This will override any kind of output scaling, so
///         that mapping a surface with a buffer size equal to the mode can
///         fill the screen independent of buffer_scale.
///
///         A method of "fill" means we don't scale up the buffer, however
///         any output scale is applied. This means that you may run into
///         an edge case where the application maps a buffer with the same
///         size of the output mode but buffer_scale 1 (thus making a
///         surface larger than the output). In this case it is allowed to
///         downscale the results to fit the screen.
///
///         The compositor must reply to this request with a configure event
///         with the dimensions for the output on which the surface will
///         be made fullscreen.
///       </description>
///       <arg name="method" type="uint" enum="fullscreen_method" summary="method for resolving size conflict" />
///       <arg name="framerate" type="uint" summary="framerate in mHz" />
///       <arg name="output" type="object" interface="wl_output" allow-null="true" summary="output on which the surface is to be fullscreen" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlShellSurfaceSetFullscreenRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `method`: method for resolving size conflict.
    pub method: WlShellSurfaceFullscreenMethod,
    /// Argument `framerate`: framerate in mHz.
    pub framerate: u32,
    /// Argument `output`: output on which the surface is to be fullscreen.
    /// The protocol permits NULL.
    /// Object implementing interface wl_output.
    pub output: Option<u32>,
}
impl WlShellSurfaceSetFullscreenRequest {
    /// Opcode in the request namespace of interface wl_shell_surface.
    pub const OPCODE: u16 = 5;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlShellSurfaceSetFullscreenRequest {
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
            "wl_shell_surface",
            Self::SINCE,
        )?;
        let method = WlShellSurfaceFullscreenMethod(reader.next_u32().await?);
        let framerate = reader.next_u32().await?;
        let output = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            method,
            framerate,
            output,
        })
    }
}

impl CommandWriter for WlShellSurfaceSetFullscreenRequest {
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
        Self::writer_request_interface(state, self.object_id, "wl_shell_surface", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.method.0).await?;
        writer.next_u32(self.framerate).await?;
        writer.next_u32(self.output.unwrap_or(0)).await?;
        Ok(())
    }
}
