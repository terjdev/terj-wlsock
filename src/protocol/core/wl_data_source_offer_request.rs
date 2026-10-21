use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_data_source.offer`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="offer">
///       <description summary="add an offered mime type">
///         This request adds a mime type to the set of mime types
///         advertised to targets.  Can be called several times to offer
///         multiple types.
///       </description>
///       <arg name="mime_type" type="string" summary="mime type offered by the data source" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlDataSourceOfferRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `mime_type`: mime type offered by the data source.
    pub mime_type: String,
}
impl WlDataSourceOfferRequest {
    /// Opcode in the request namespace of interface wl_data_source.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlDataSourceOfferRequest {
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
            "wl_data_source",
            Self::SINCE,
        )?;
        let mime_type = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            mime_type,
        })
    }
}
