use super::ObjectInteface as LinuxExplicitSynchronizationObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_linux_surface_synchronization_v1.get_release`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="get_release">
///       <description summary="release fence for last-attached buffer">
///         Create a listener for the release of the buffer attached by the
///         client with wl_surface.attach. See zwp_linux_buffer_release_v1
///         documentation for more information.
///
///         The release object is double-buffered state, and will be associated
///         with the buffer that is attached to the surface at wl_surface.commit
///         time.
///
///         If a zwp_linux_buffer_release_v1 object has already been requested for
///         the surface in the same commit cycle, a DUPLICATE_RELEASE error is
///         raised.
///
///         If the associated wl_surface was destroyed, a NO_SURFACE error
///         is raised.
///
///         If at surface commit time there is no buffer attached, a NO_BUFFER
///         error is raised.
///       </description>
///       <arg name="release" type="new_id" interface="zwp_linux_buffer_release_v1" summary="new zwp_linux_buffer_release_v1 object" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpLinuxSurfaceSynchronizationV1GetReleaseRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// new zwp_linux_buffer_release_v1 object.
    pub release: u32,
}
impl ZwpLinuxSurfaceSynchronizationV1GetReleaseRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpLinuxSurfaceSynchronizationV1GetReleaseRequest {
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
impl InterfaceReader for ZwpLinuxSurfaceSynchronizationV1GetReleaseRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
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
        let release = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::LinuxExplicitSynchronization(
                LinuxExplicitSynchronizationObjectInteface::ZwpLinuxBufferReleaseV1 {
                    version: interface.version().min(1),
                },
            );
            Self::reader_state(state)?
                .register_object(release, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self { object_id, release })
    }
}
impl CommandWriter for ZwpLinuxSurfaceSynchronizationV1GetReleaseRequest {
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
        {
            let new_interface = ObjectInteface::LinuxExplicitSynchronization(
                LinuxExplicitSynchronizationObjectInteface::ZwpLinuxBufferReleaseV1 {
                    version: interface.version().min(1),
                },
            );
            Self::writer_state(state)?
                .register_object(self.release, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.release).await?;
        Ok(())
    }
}
