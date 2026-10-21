use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_data_offer.finish`.
///
/// Opcode 3; available since version 3. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="finish" since="3">
///       <description summary="the offer will no longer be used">
///         Notifies the compositor that the drag destination successfully
///         finished the drag-and-drop operation.
///
///         Upon receiving this request, the compositor will emit
///         wl_data_source.dnd_finished on the drag source client.
///
///         It is a client error to perform other requests than
///         wl_data_offer.destroy after this one. It is also an error to perform
///         this request after a NULL mime type has been set in
///         wl_data_offer.accept or no action was received through
///         wl_data_offer.action.
///
///         If wl_data_offer.finish request is received for a non drag and drop
///         operation, the invalid_finish protocol error is raised.
///       </description>
///     </request>
/// ```
#[derive(Debug)]
pub struct WlDataOfferFinishRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
}
impl WlDataOfferFinishRequest {
    /// Opcode in the request namespace of interface wl_data_offer.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 3;
}
impl CommandReader<Self> for WlDataOfferFinishRequest {
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
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id })
    }
}
