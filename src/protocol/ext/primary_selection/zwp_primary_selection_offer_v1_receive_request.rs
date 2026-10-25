use super::ObjectInteface as PrimarySelectionObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_primary_selection_offer_v1.receive`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="receive">
///       <description summary="request that the data is transferred">
///         To transfer the contents of the primary selection clipboard, the client
///         issues this request and indicates the mime type that it wants to
///         receive. The transfer happens through the passed file descriptor
///         (typically created with the pipe system call). The source client writes
///         the data in the mime type representation requested and then closes the
///         file descriptor.
///
///         The receiving client reads from the read end of the pipe until EOF and
///         closes its end, at which point the transfer is complete.
///       </description>
///       <arg name="mime_type" type="string" />
///       <arg name="fd" type="fd" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpPrimarySelectionOfferV1ReceiveRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `mime_type`.
    pub mime_type: String,
    /// Wire argument `fd`.
    pub fd: std::os::fd::OwnedFd,
}
impl ZwpPrimarySelectionOfferV1ReceiveRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpPrimarySelectionOfferV1ReceiveRequest {
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
impl InterfaceReader for ZwpPrimarySelectionOfferV1ReceiveRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::PrimarySelection(
                PrimarySelectionObjectInteface::ZwpPrimarySelectionOfferV1 { .. }
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
impl CommandWriter for ZwpPrimarySelectionOfferV1ReceiveRequest {
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
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "zwp_primary_selection_offer_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::PrimarySelection(
                PrimarySelectionObjectInteface::ZwpPrimarySelectionOfferV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
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
