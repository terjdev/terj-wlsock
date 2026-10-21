use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_surface.preferred_buffer_scale`.
///
/// Opcode 2; available since version 6. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="preferred_buffer_scale" since="6">
///       <description summary="preferred buffer scale for the surface">
///         This event indicates the preferred buffer scale for this surface. It is
///         sent whenever the compositor's preference changes.
///
///         Before receiving this event the preferred buffer scale for this surface
///         is 1.
///
///         It is intended that scaling aware clients use this event to scale their
///         content and use wl_surface.set_buffer_scale to indicate the scale they
///         have rendered with. This allows clients to supply a higher detail
///         buffer.
///
///         The compositor shall emit a scale value greater than 0.
///       </description>
///       <arg name="factor" type="int" summary="preferred scaling factor" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlSurfacePreferredBufferScaleEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `factor`: preferred scaling factor.
    pub factor: i32,
}
impl WlSurfacePreferredBufferScaleEvent {
    /// Opcode in the event namespace of interface wl_surface.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 6;
}
impl CommandWriter for WlSurfacePreferredBufferScaleEvent {
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
        Self::writer_event_interface(state, self.object_id, "wl_surface", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.factor).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlSurfacePreferredBufferScaleEvent {
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
            "wl_surface",
            Self::SINCE,
        )?;
        let factor = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, factor })
    }
}
