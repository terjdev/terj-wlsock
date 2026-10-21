use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_pointer.warp`.
///
/// Opcode 11; available since version 11. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="warp" since="11">
///       <description summary="pointer warp event">
///     Notification of pointer location change within a surface.
///
///     This location change is not due to events on the input device,
///     but because either the surface under the pointer was moved and
///     thus the relative position of the pointer changed, or because
///     the compositor changed the pointer position in response to an
///     event like pointer confinement being exited.
///
///     The arguments surface_x and surface_y are the location relative to
///     the focused surface.
///
///     This event must not occur in the same wl_pointer.frame as a
///     wl_pointer.enter or wl_pointer.motion event.
///       </description>
///       <arg name="surface_x" type="fixed" summary="surface-local x coordinate" />
///       <arg name="surface_y" type="fixed" summary="surface-local y coordinate" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlPointerWarpEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `surface_x`: surface-local x coordinate.
    pub surface_x: f64,
    /// Argument `surface_y`: surface-local y coordinate.
    pub surface_y: f64,
}
impl WlPointerWarpEvent {
    /// Opcode in the event namespace of interface wl_pointer.
    pub const OPCODE: u16 = 11;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 11;
}
impl CommandWriter for WlPointerWarpEvent {
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
        Self::writer_event_interface(state, self.object_id, "wl_pointer", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.surface_x) {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.surface_y) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_fixed(self.surface_x).await?;
        writer.next_fixed(self.surface_y).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlPointerWarpEvent {
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
            "wl_pointer",
            Self::SINCE,
        )?;
        let surface_x = reader.next_fixed().await?;
        let surface_y = reader.next_fixed().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            surface_x,
            surface_y,
        })
    }
}
