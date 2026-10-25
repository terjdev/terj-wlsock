use super::ObjectInteface as LinuxExplicitSynchronizationObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_linux_surface_synchronization_v1.set_acquire_fence`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_acquire_fence">
///       <description summary="set the acquire fence">
///         Set the acquire fence that must be signaled before the compositor
///         may sample from the buffer attached with wl_surface.attach. The fence
///         is a dma_fence kernel object.
///
///         The acquire fence is double-buffered state, and will be applied on the
///         next wl_surface.commit request for the associated surface. Thus, it
///         applies only to the buffer that is attached to the surface at commit
///         time.
///
///         If the provided fd is not a valid dma_fence fd, then an INVALID_FENCE
///         error is raised.
///
///         If a fence has already been attached during the same commit cycle, a
///         DUPLICATE_FENCE error is raised.
///
///         If the associated wl_surface was destroyed, a NO_SURFACE error is
///         raised.
///
///         If at surface commit time the attached buffer does not support explicit
///         synchronization, an UNSUPPORTED_BUFFER error is raised.
///
///         If at surface commit time there is no buffer attached, a NO_BUFFER
///         error is raised.
///       </description>
///       <arg name="fd" type="fd" summary="acquire fence fd" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpLinuxSurfaceSynchronizationV1SetAcquireFenceRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// acquire fence fd.
    pub fd: std::os::fd::OwnedFd,
}
impl ZwpLinuxSurfaceSynchronizationV1SetAcquireFenceRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpLinuxSurfaceSynchronizationV1SetAcquireFenceRequest {
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
impl InterfaceReader for ZwpLinuxSurfaceSynchronizationV1SetAcquireFenceRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::LinuxExplicitSynchronization(
                LinuxExplicitSynchronizationObjectInteface::ZwpLinuxSurfaceSynchronizationV1 { .. }
            )
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
impl CommandWriter for ZwpLinuxSurfaceSynchronizationV1SetAcquireFenceRequest {
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
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "zwp_linux_surface_synchronization_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::LinuxExplicitSynchronization(
                LinuxExplicitSynchronizationObjectInteface::ZwpLinuxSurfaceSynchronizationV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_fd(self.fd).await?;
        Ok(())
    }
}
