use crate::protocol::ext::drm_lease::ObjectInteface as DrmLeaseObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wp_drm_lease_v1.lease_fd`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="lease_fd">
///       <description summary="shares the DRM file descriptor">
///         This event returns a file descriptor suitable for use with DRM-related
///         ioctls. The client should use drmModeGetLease to enumerate the DRM
///         objects which have been leased to them. The compositor guarantees it
///         will not use the leased DRM objects itself until it sends the finished
///         event. If the compositor cannot or will not grant a lease for the
///         requested connectors, it will not send this event, instead sending the
///         finished event.
///
///         The compositor will send this event at most once during this objects
///         lifetime.
///       </description>
///       <arg name="leased_fd" type="fd" summary="leased DRM file descriptor" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WpDrmLeaseV1LeaseFdEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// leased DRM file descriptor.
    pub leased_fd: std::os::fd::OwnedFd,
}
impl WpDrmLeaseV1LeaseFdEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpDrmLeaseV1LeaseFdEvent {
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
impl InterfaceReader for WpDrmLeaseV1LeaseFdEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::DrmLease(DrmLeaseObjectInteface::WpDrmLeaseV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        let leased_fd = reader.next_fd().ok_or(FdReaderError::InvalidProtocol)?;
        Ok(Self {
            object_id,
            leased_fd,
        })
    }
}
impl CommandWriter for WpDrmLeaseV1LeaseFdEvent {
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
        Self::writer_event_interface(state, self.object_id, "wp_drm_lease_v1", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_fd(self.leased_fd).await?;
        Ok(())
    }
}
