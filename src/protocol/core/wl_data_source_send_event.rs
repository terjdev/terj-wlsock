use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_data_source.send`.
///
/// Opcode 1; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="send">
///       <description summary="send the data">
///         Request for data from the client.  Send the data as the
///         specified mime type over the passed file descriptor, then
///         close it.
///       </description>
///       <arg name="mime_type" type="string" summary="mime type for the data" />
///       <arg name="fd" type="fd" summary="file descriptor for the data" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlDataSourceSendEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `mime_type`: mime type for the data.
    pub mime_type: String,
    /// Argument `fd`: file descriptor for the data.
    pub fd: std::os::fd::OwnedFd,
}
impl WlDataSourceSendEvent {
    /// Opcode in the event namespace of interface wl_data_source.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlDataSourceSendEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[
            FdWriter::encoded_string_size(Some(&self.mime_type)),
            Some(0),
        ])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::event_interface(state, self.object_id, "wl_data_source", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.mime_type) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.mime_type).await?;
        writer.next_fd(self.fd)?;
        Ok(())
    }
}

impl CommandReader<Self> for WlDataSourceSendEvent {
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
            "wl_data_source",
            Self::SINCE,
        )?;
        let mime_type = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        let fd = reader.next_fd().ok_or(FdReaderError::InvalidProtocol)?;
        Ok(Self {
            object_id,
            mime_type,
            fd,
        })
    }
}
