use super::WlShmFormat;
use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_shm.format`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="format">
///       <description summary="pixel format description">
///         Informs the client about a valid pixel format that
///         can be used for buffers. Known formats include
///         argb8888 and xrgb8888.
///
///         Extensions to drm_fourcc.h (or the format enum) do not require
///         increasing the wl_shm version; as a result, clients may receive format
///         codes which were not in the list at the time the client was made.
///       </description>
///       <arg name="format" type="uint" enum="format" summary="buffer pixel format" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlShmFormatEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `format`: buffer pixel format.
    pub format: WlShmFormat,
}
impl WlShmFormatEvent {
    /// Opcode in the event namespace of interface wl_shm.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlShmFormatEvent {
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
        Self::event_interface(state, self.object_id, "wl_shm", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.format.0).await?;
        Ok(())
    }
}
