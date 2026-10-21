use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_data_offer.receive`.
///
/// Opcode 1; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="receive">
///       <description summary="request that the data is transferred">
///         To transfer the offered data, the client issues this request
///         and indicates the mime type it wants to receive.  The transfer
///         happens through the passed file descriptor (typically created
///         with the pipe system call).  The source client writes the data
///         in the mime type representation requested and then closes the
///         file descriptor.
///
///         The receiving client reads from the read end of the pipe until
///         EOF and then closes its end, at which point the transfer is
///         complete.
///
///         This request may happen multiple times for different mime types,
///         both before and after wl_data_device.drop. Drag-and-drop destination
///         clients may preemptively fetch data or examine it more closely to
///         determine acceptance.
///       </description>
///       <arg name="mime_type" type="string" summary="mime type desired by receiver" />
///       <arg name="fd" type="fd" summary="file descriptor for data transfer" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlDataOfferReceiveRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `mime_type`: mime type desired by receiver.
    pub mime_type: String,
    /// Argument `fd`: file descriptor for data transfer.
    pub fd: std::os::fd::OwnedFd,
}
impl WlDataOfferReceiveRequest {
    /// Opcode in the request namespace of interface wl_data_offer.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlDataOfferReceiveRequest {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Self::request_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_data_offer",
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
