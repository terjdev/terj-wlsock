use crate::protocol::ext::drm_lease::ObjectInteface as DrmLeaseObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wp_drm_lease_connector_v1.description`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="description">
///       <description summary="description">
///         The compositor sends this event once the connector is created to provide
///         a human-readable description for this connector, which may be presented
///         to the user. The compositor may send this event multiple times over the
///         lifetime of this object to reflect changes in the description.
///       </description>
///       <arg name="description" type="string" summary="connector description" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WpDrmLeaseConnectorV1DescriptionEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// connector description.
    pub description: String,
}
impl WpDrmLeaseConnectorV1DescriptionEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpDrmLeaseConnectorV1DescriptionEvent {
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
impl InterfaceReader for WpDrmLeaseConnectorV1DescriptionEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::DrmLease(DrmLeaseObjectInteface::WpDrmLeaseConnectorV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let description = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            description,
        })
    }
}
impl CommandWriter for WpDrmLeaseConnectorV1DescriptionEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_string_size(Some(&self.description))])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_event_interface(
            state,
            self.object_id,
            "wp_drm_lease_connector_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.description) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.description).await?;
        Ok(())
    }
}
