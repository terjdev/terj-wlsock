use crate::protocol::ClientState;
use crate::protocol::core::ObjectInteface as CoreObjectInteface;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wl_shell_surface.set_maximized`.
///
/// Opcode 7; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire framing and receiver metadata; object semantics belong to the compositor.
/// The caller handles compositor policy and message contents.
/// Client encoding checks the complete payload quota before writing arguments.
/// New object IDs are registered before payload I/O; client IDs remain reserved
/// after destruction until the server acknowledges them with delete_id.
/// ```xml
/// <request name="set_maximized">
///       <description summary="make the surface a maximized surface">
///         Map the surface as a maximized surface.
///
///         If an output parameter is given then the surface will be
///         maximized on that output. If the client does not specify the
///         output then the compositor will apply its policy - usually
///         choosing the output on which the surface has the biggest surface
///         area.
///
///         The compositor will reply with a configure event telling
///         the expected new surface size. The operation is completed
///         on the next buffer attach to this surface.
///
///         A maximized surface typically fills the entire output it is
///         bound to, except for desktop elements such as panels. This is
///         the main difference between a maximized shell surface and a
///         fullscreen shell surface.
///
///         The details depend on the compositor implementation.
///       </description>
///       <arg name="output" type="object" interface="wl_output" allow-null="true" summary="output on which the surface is to be maximized" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlShellSurfaceSetMaximizedRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `output`: output on which the surface is to be maximized.
    /// The protocol permits NULL.
    /// Object implementing interface wl_output.
    pub output: Option<u32>,
}
impl WlShellSurfaceSetMaximizedRequest {
    /// Opcode in the request namespace of interface wl_shell_surface.
    pub const OPCODE: u16 = 7;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlShellSurfaceSetMaximizedRequest {
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

impl InterfaceReader for WlShellSurfaceSetMaximizedRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Core(CoreObjectInteface::WlShellSurface { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let output = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, output })
    }
}

impl CommandWriter for WlShellSurfaceSetMaximizedRequest {
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
        Self::writer_request_interface(state, self.object_id, "wl_shell_surface", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.output.unwrap_or(0)).await?;
        Ok(())
    }
}
