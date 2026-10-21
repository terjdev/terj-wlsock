use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wl_data_offer.finish`.
///
/// Opcode 3; available since version 3. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire framing and receiver metadata; object semantics belong to the compositor.
/// The caller handles compositor policy and message contents.
/// Client encoding checks the complete payload quota before writing arguments.
/// New object IDs are registered before payload I/O; client IDs remain reserved
/// after destruction until the server acknowledges them with delete_id.
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
        Self::reader_request_interface(
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

impl CommandWriter for WlDataOfferFinishRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[])
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
        Ok(())
    }
}
