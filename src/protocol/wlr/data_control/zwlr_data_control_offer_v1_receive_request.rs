use crate::protocol::wlr::data_control::ObjectInteface as WlrDataControlObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_data_control_offer_v1.receive`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="receive">
///       <description summary="request that the data is transferred">
///         To transfer the offered data, the client issues this request and
///         indicates the MIME type it wants to receive. The transfer happens
///         through the passed file descriptor (typically created with the pipe
///         system call). The source client writes the data in the MIME type
///         representation requested and then closes the file descriptor.
///
///         The receiving client reads from the read end of the pipe until EOF and
///         then closes its end, at which point the transfer is complete.
///
///         This request may happen multiple times for different MIME types.
///       </description>
///       <arg name="mime_type" type="string" summary="MIME type desired by receiver" />
///       <arg name="fd" type="fd" summary="file descriptor for data transfer" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrDataControlOfferV1ReceiveRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// MIME type desired by receiver.
    pub mime_type: String,
    /// file descriptor for data transfer.
    pub fd: std::os::fd::OwnedFd,
}
impl ZwlrDataControlOfferV1ReceiveRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrDataControlOfferV1ReceiveRequest {
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
impl InterfaceReader for ZwlrDataControlOfferV1ReceiveRequest {
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
                WlrDataControlObjectInteface::ZwlrDataControlOfferV1 { .. }
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
impl CommandWriter for ZwlrDataControlOfferV1ReceiveRequest {
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
        Self::writer_request_interface(
            state,
            self.object_id,
            "zwlr_data_control_offer_v1",
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
