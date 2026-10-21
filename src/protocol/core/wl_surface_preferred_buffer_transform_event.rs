use super::WlOutputTransform;
use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_surface.preferred_buffer_transform`.
///
/// Opcode 3; available since version 6. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="preferred_buffer_transform" since="6">
///       <description summary="preferred buffer transform for the surface">
///         This event indicates the preferred buffer transform for this surface.
///         It is sent whenever the compositor's preference changes.
///
///         Before receiving this event the preferred buffer transform for this
///         surface is normal.
///
///         Applying this transformation to the surface buffer contents and using
///         wl_surface.set_buffer_transform might allow the compositor to use the
///         surface buffer more efficiently.
///       </description>
///       <arg name="transform" type="uint" enum="wl_output.transform" summary="preferred transform" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlSurfacePreferredBufferTransformEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `transform`: preferred transform.
    pub transform: WlOutputTransform,
}
impl WlSurfacePreferredBufferTransformEvent {
    /// Opcode in the event namespace of interface wl_surface.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 6;
}
impl CommandWriter for WlSurfacePreferredBufferTransformEvent {
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
        Self::event_interface(state, self.object_id, "wl_surface", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.transform.0).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlSurfacePreferredBufferTransformEvent {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Self::event_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_surface",
            Self::SINCE,
        )?;
        let transform = WlOutputTransform::read(object_id, opcode, reader, state).await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            transform,
        })
    }
}
