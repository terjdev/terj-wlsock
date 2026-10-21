use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_region.subtract`.
///
/// Opcode 2; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="subtract">
///       <description summary="subtract rectangle from region">
///         Subtract the specified rectangle from the region.
///       </description>
///       <arg name="x" type="int" summary="region-local x coordinate" />
///       <arg name="y" type="int" summary="region-local y coordinate" />
///       <arg name="width" type="int" summary="rectangle width" />
///       <arg name="height" type="int" summary="rectangle height" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlRegionSubtractRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `x`: region-local x coordinate.
    pub x: i32,
    /// Argument `y`: region-local y coordinate.
    pub y: i32,
    /// Argument `width`: rectangle width.
    pub width: i32,
    /// Argument `height`: rectangle height.
    pub height: i32,
}
impl WlRegionSubtractRequest {
    /// Opcode in the request namespace of interface wl_region.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlRegionSubtractRequest {
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
            "wl_region",
            Self::SINCE,
        )?;
        let x = reader.next_i32().await?;
        let y = reader.next_i32().await?;
        let width = reader.next_i32().await?;
        let height = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            x,
            y,
            width,
            height,
        })
    }
}
