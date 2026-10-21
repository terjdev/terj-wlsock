use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wl_data_offer.accept`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire framing and receiver metadata; object semantics belong to the compositor.
/// The caller handles compositor policy and message contents.
/// Client encoding checks the complete payload quota before writing arguments.
/// New object IDs are registered before payload I/O; client IDs remain reserved
/// after destruction until the server acknowledges them with delete_id.
/// ```xml
/// <request name="accept">
///       <description summary="accept one of the offered mime types">
///         Indicate that the client can accept the given mime type, or
///         NULL for not accepted.
///
///         For objects of version 2 or older, this request is used by the
///         client to give feedback whether the client can receive the given
///         mime type, or NULL if none is accepted; the feedback does not
///         determine whether the drag-and-drop operation succeeds or not.
///
///         For objects of version 3 or newer, this request determines the
///         final result of the drag-and-drop operation. If the end result
///         is that no mime types were accepted, the drag-and-drop operation
///         will be cancelled and the corresponding drag source will receive
///         wl_data_source.cancelled. Clients may still use this event in
///         conjunction with wl_data_source.action for feedback.
///       </description>
///       <arg name="serial" type="uint" summary="serial number of the accept request" />
///       <arg name="mime_type" type="string" allow-null="true" summary="mime type accepted by the client" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlDataOfferAcceptRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `serial`: serial number of the accept request.
    pub serial: u32,
    /// Argument `mime_type`: mime type accepted by the client.
    /// The protocol permits NULL.
    pub mime_type: Option<String>,
}
impl WlDataOfferAcceptRequest {
    /// Opcode in the request namespace of interface wl_data_offer.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlDataOfferAcceptRequest {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Self::reader_request_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_data_offer",
            Self::SINCE,
        )?;
        let serial = reader.next_u32().await?;
        let mime_type = reader.next_nullable_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            serial,
            mime_type,
        })
    }
}

impl CommandWriter for WlDataOfferAcceptRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[
            Some(4),
            FdWriter::encoded_string_size(self.mime_type.as_deref()),
        ])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(state, self.object_id, "wl_data_offer", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if self
            .mime_type
            .as_deref()
            .is_some_and(|value| !FdWriter::is_valid_string(value))
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        writer
            .next_nullable_string(self.mime_type.as_deref())
            .await?;
        Ok(())
    }
}
