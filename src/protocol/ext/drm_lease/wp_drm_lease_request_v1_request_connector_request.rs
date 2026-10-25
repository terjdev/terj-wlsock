use crate::protocol::ext::drm_lease::ObjectInteface as DrmLeaseObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_drm_lease_request_v1.request_connector`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="request_connector">
///       <description summary="request a connector for this lease">
///         Indicates that the client would like to lease the given connector.
///         This is only used as a suggestion, the compositor may choose to
///         include any resources in the lease it issues, or change the set of
///         leased resources at any time. Compositors are however encouraged to
///         include the requested connector and other resources necessary
///         to drive the connected output in the lease.
///
///         Requesting a connector that was created from a different lease device
///         than this lease request raises the wrong_device error. Requesting a
///         connector twice will raise the duplicate_connector error.
///       </description>
///       <arg name="connector" type="object" interface="wp_drm_lease_connector_v1" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpDrmLeaseRequestV1RequestConnectorRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `connector`.
    pub connector: u32,
}
impl WpDrmLeaseRequestV1RequestConnectorRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpDrmLeaseRequestV1RequestConnectorRequest {
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
impl InterfaceReader for WpDrmLeaseRequestV1RequestConnectorRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::DrmLease(DrmLeaseObjectInteface::WpDrmLeaseRequestV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let connector = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            connector,
        })
    }
}
impl CommandWriter for WpDrmLeaseRequestV1RequestConnectorRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(
            state,
            self.object_id,
            "wp_drm_lease_request_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.connector).await?;
        Ok(())
    }
}
