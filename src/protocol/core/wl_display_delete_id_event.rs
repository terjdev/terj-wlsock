use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_display.delete_id`.
///
/// Opcode 1; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="delete_id">
///       <description summary="acknowledge object ID deletion">
///         This event is used internally by the object ID management logic.
///
///         When the server stops using an object created by the client, the server
///         sends this event. In particular, after sending this event, the server
///         will no longer send any events that contain the object as the receiver
///         or as an argument.
///
///         When the client receives this event, it knows that it can reuse the
///         object ID.
///       </description>
///       <arg name="id" type="uint" summary="deleted object ID" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlDisplayDeleteIdEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `id`: deleted object ID.
    pub id: u32,
}
impl WlDisplayDeleteIdEvent {
    /// Opcode in the event namespace of interface wl_display.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlDisplayDeleteIdEvent {
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
        Self::event_interface(state, self.object_id, "wl_display", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !Self::write_state(state)?.can_delete_id(self.id) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.id).await?;
        {
            let mut state = Self::write_state(state)?;
            state.delete_id(self.id);
        }
        Ok(())
    }
}

impl CommandReader<Self> for WlDisplayDeleteIdEvent {
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
            "wl_display",
            Self::SINCE,
        )?;
        let id = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let mut state = Self::read_state(state)?;
            if !state.can_delete_id(id) {
                return Err(FdReaderError::InvalidProtocol);
            }
            state.delete_id(id);
        }
        Ok(Self { object_id, id })
    }
}
