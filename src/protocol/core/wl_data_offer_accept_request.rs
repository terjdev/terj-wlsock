use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_data_offer.accept`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
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
        Self::request_interface(
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
