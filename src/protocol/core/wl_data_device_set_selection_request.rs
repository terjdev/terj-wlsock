use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_data_device.set_selection`.
///
/// Opcode 1; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="set_selection">
///       <description summary="copy data to the selection">
///         This request asks the compositor to set the selection
///         to the data from the source on behalf of the client.
///
///         To unset the selection, set the source to NULL.
///
///         The given source may not be used in any further set_selection or
///         start_drag requests. Attempting to reuse a previously-used source
///         may send a used_source error.
///       </description>
///       <arg name="source" type="object" interface="wl_data_source" allow-null="true" summary="data source for the selection" />
///       <arg name="serial" type="uint" summary="serial number of the event that triggered this request" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlDataDeviceSetSelectionRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `source`: data source for the selection.
    /// The protocol permits NULL.
    /// Object implementing interface wl_data_source.
    pub source: Option<u32>,
    /// Argument `serial`: serial number of the event that triggered this request.
    pub serial: u32,
}
impl WlDataDeviceSetSelectionRequest {
    /// Opcode in the request namespace of interface wl_data_device.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlDataDeviceSetSelectionRequest {
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
            "wl_data_device",
            Self::SINCE,
        )?;
        let source = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        let serial = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            source,
            serial,
        })
    }
}
