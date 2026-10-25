use crate::protocol::wlr::data_control::ObjectInteface as WlrDataControlObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_data_control_source_v1.send`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="send">
///       <description summary="send the data">
///         Request for data from the client. Send the data as the specified MIME
///         type over the passed file descriptor, then close it.
///       </description>
///       <arg name="mime_type" type="string" summary="MIME type for the data" />
///       <arg name="fd" type="fd" summary="file descriptor for the data" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrDataControlSourceV1SendEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// MIME type for the data.
    pub mime_type: String,
    /// file descriptor for the data.
    pub fd: std::os::fd::OwnedFd,
}
impl ZwlrDataControlSourceV1SendEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrDataControlSourceV1SendEvent {
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
impl InterfaceReader for ZwlrDataControlSourceV1SendEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrDataControl(
                WlrDataControlObjectInteface::ZwlrDataControlSourceV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
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
impl CommandWriter for ZwlrDataControlSourceV1SendEvent {
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
        Self::writer_event_interface(
            state,
            self.object_id,
            "zwlr_data_control_source_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.mime_type) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.mime_type).await?;
        writer.next_fd(self.fd).await?;
        Ok(())
    }
}
