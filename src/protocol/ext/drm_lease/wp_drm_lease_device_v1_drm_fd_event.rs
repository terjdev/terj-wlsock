use crate::protocol::ext::drm_lease::ObjectInteface as DrmLeaseObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wp_drm_lease_device_v1.drm_fd`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="drm_fd">
///       <description summary="open a non-master fd for this DRM node">
///         The compositor will send this event when the wp_drm_lease_device_v1
///         global is bound, although there are no guarantees as to how long this
///         takes - the compositor might need to wait until regaining DRM master.
///         The included fd is a non-master DRM file descriptor opened for this
///         device and the compositor must not authenticate it.
///         The purpose of this event is to give the client the ability to
///         query DRM and discover information which may help them pick the
///         appropriate DRM device or select the appropriate connectors therein.
///       </description>
///       <arg name="fd" type="fd" summary="DRM file descriptor" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WpDrmLeaseDeviceV1DrmFdEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// DRM file descriptor.
    pub fd: std::os::fd::OwnedFd,
}
impl WpDrmLeaseDeviceV1DrmFdEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpDrmLeaseDeviceV1DrmFdEvent {
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
impl InterfaceReader for WpDrmLeaseDeviceV1DrmFdEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::DrmLease(DrmLeaseObjectInteface::WpDrmLeaseDeviceV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        let fd = reader.next_fd().ok_or(FdReaderError::InvalidProtocol)?;
        Ok(Self { object_id, fd })
    }
}
impl CommandWriter for WpDrmLeaseDeviceV1DrmFdEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(0)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_event_interface(state, self.object_id, "wp_drm_lease_device_v1", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_fd(self.fd).await?;
        Ok(())
    }
}
