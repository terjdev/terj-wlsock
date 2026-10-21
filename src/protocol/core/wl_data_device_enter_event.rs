use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_data_device.enter`.
///
/// Opcode 1; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="enter">
///       <description summary="initiate drag-and-drop session">
///         This event is sent when an active drag-and-drop pointer enters
///         a surface owned by the client.  The position of the pointer at
///         enter time is provided by the x and y arguments, in surface-local
///         coordinates.
///       </description>
///       <arg name="serial" type="uint" summary="serial number of the enter event" />
///       <arg name="surface" type="object" interface="wl_surface" summary="client surface entered" />
///       <arg name="x" type="fixed" summary="surface-local x coordinate" />
///       <arg name="y" type="fixed" summary="surface-local y coordinate" />
///       <arg name="id" type="object" interface="wl_data_offer" allow-null="true" summary="source data_offer object" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlDataDeviceEnterEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `serial`: serial number of the enter event.
    pub serial: u32,
    /// Argument `surface`: client surface entered.
    /// Object implementing interface wl_surface.
    pub surface: u32,
    /// Argument `x`: surface-local x coordinate.
    pub x: f64,
    /// Argument `y`: surface-local y coordinate.
    pub y: f64,
    /// Argument `id`: source data_offer object.
    /// The protocol permits NULL.
    /// Object implementing interface wl_data_offer.
    pub id: Option<u32>,
}
impl WlDataDeviceEnterEvent {
    /// Opcode in the event namespace of interface wl_data_device.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlDataDeviceEnterEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::event_interface(state, self.object_id, "wl_data_device", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.x) {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.y) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        writer.next_u32(self.surface).await?;
        writer.next_fixed(self.x).await?;
        writer.next_fixed(self.y).await?;
        writer.next_u32(self.id.unwrap_or(0)).await?;
        Ok(())
    }
}
