use super::WlShellSurfaceResize;
use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_shell_surface.configure`.
///
/// Opcode 1; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="configure">
///       <description summary="suggest resize">
///         The configure event asks the client to resize its surface.
///
///         The size is a hint, in the sense that the client is free to
///         ignore it if it doesn't resize, pick a smaller size (to
///         satisfy aspect ratio or resize in steps of NxM pixels).
///
///         The edges parameter provides a hint about how the surface
///         was resized. The client may use this information to decide
///         how to adjust its content to the new size (e.g. a scrolling
///         area might adjust its content position to leave the viewable
///         content unmoved).
///
///         The client is free to dismiss all but the last configure
///         event it received.
///
///         The width and height arguments specify the size of the window
///         in surface-local coordinates.
///       </description>
///       <arg name="edges" type="uint" enum="resize" summary="how the surface was resized" />
///       <arg name="width" type="int" summary="new width of the surface" />
///       <arg name="height" type="int" summary="new height of the surface" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlShellSurfaceConfigureEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `edges`: how the surface was resized.
    pub edges: WlShellSurfaceResize,
    /// Argument `width`: new width of the surface.
    pub width: i32,
    /// Argument `height`: new height of the surface.
    pub height: i32,
}
impl WlShellSurfaceConfigureEvent {
    /// Opcode in the event namespace of interface wl_shell_surface.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlShellSurfaceConfigureEvent {
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
        Self::writer_event_interface(state, self.object_id, "wl_shell_surface", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.edges.0).await?;
        writer.next_i32(self.width).await?;
        writer.next_i32(self.height).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlShellSurfaceConfigureEvent {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Self::reader_event_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_shell_surface",
            Self::SINCE,
        )?;
        let edges = WlShellSurfaceResize::read(object_id, opcode, reader, state).await?;
        let width = reader.next_i32().await?;
        let height = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            edges,
            width,
            height,
        })
    }
}
