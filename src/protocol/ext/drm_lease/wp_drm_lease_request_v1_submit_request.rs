use crate::protocol::ext::drm_lease::ObjectInteface as DrmLeaseObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_drm_lease_request_v1.submit`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="submit" type="destructor">
///       <description summary="submit the lease request">
///         Submits the lease request and creates a new wp_drm_lease_v1 object.
///         After calling submit the compositor will immediately destroy this
///         object, issuing any more requests will cause a wl_display error.
///         The compositor doesn't make any guarantees about the events of the
///         lease object, clients cannot expect an immediate response.
///         Not requesting any connectors before submitting the lease request
///         will raise the empty_lease error.
///       </description>
///       <arg name="id" type="new_id" interface="wp_drm_lease_v1" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpDrmLeaseRequestV1SubmitRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `id`.
    pub id: u32,
}
impl WpDrmLeaseRequestV1SubmitRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpDrmLeaseRequestV1SubmitRequest {
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
impl InterfaceReader for WpDrmLeaseRequestV1SubmitRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::DrmLease(DrmLeaseObjectInteface::WpDrmLeaseRequestV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("wp_drm_lease_v1", interface.version().min(1))
                    .ok_or(FdReaderError::InvalidProtocol)?;
            Self::reader_state(state)?
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Self::reader_state(state)?.release_server_id(object_id);
        Ok(Self { object_id, id })
    }
}
impl CommandWriter for WpDrmLeaseRequestV1SubmitRequest {
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
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "wp_drm_lease_request_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("wp_drm_lease_v1", interface.version().min(1))
                    .ok_or(FdWriterError::InvalidProtocol)?;
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        Self::writer_state(state)?.release_server_id(self.object_id);
        Ok(())
    }
}
