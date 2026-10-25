use crate::protocol::ext::drm_lease::ObjectInteface as DrmLeaseObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wp_drm_lease_connector_v1.withdrawn`.
///
/// Opcode 4; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="withdrawn">
///       <description summary="lease offer withdrawn">
///         Sent to indicate that the compositor will no longer honor requests for
///         DRM leases which include this connector. The client may still issue a
///         lease request including this connector, but the compositor will send
///         wp_drm_lease_v1.finished without issuing a lease fd. Compositors are
///         encouraged to send this event when they lose access to connector, for
///         example when the connector is hot-unplugged, when the connector gets
///         leased to a client or when the compositor loses DRM master.
///
///         If a client holds a lease for the connector, the status of the lease
///         remains the same. The client should destroy the object after receiving
///         this event.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct WpDrmLeaseConnectorV1WithdrawnEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl WpDrmLeaseConnectorV1WithdrawnEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 4;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpDrmLeaseConnectorV1WithdrawnEvent {
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
impl InterfaceReader for WpDrmLeaseConnectorV1WithdrawnEvent {
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
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id })
    }
}
impl CommandWriter for WpDrmLeaseConnectorV1WithdrawnEvent {
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
        Self::writer_event_interface(
            state,
            self.object_id,
            "wp_drm_lease_connector_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        Ok(())
    }
}
