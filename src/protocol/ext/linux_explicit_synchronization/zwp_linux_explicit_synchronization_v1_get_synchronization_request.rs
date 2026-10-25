use super::ObjectInteface as LinuxExplicitSynchronizationObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_linux_explicit_synchronization_v1.get_synchronization`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="get_synchronization">
///       <description summary="extend surface interface for explicit synchronization">
///         Instantiate an interface extension for the given wl_surface to provide
///         explicit synchronization.
///
///         If the given wl_surface already has an explicit synchronization object
///         associated, the synchronization_exists protocol error is raised.
///
///         Graphics APIs, like EGL or Vulkan, that manage the buffer queue and
///         commits of a wl_surface themselves, are likely to be using this
///         extension internally. If a client is using such an API for a
///         wl_surface, it should not directly use this extension on that surface,
///         to avoid raising a synchronization_exists protocol error.
///       </description>
///
///       <arg name="id" type="new_id" interface="zwp_linux_surface_synchronization_v1" summary="the new synchronization interface id" />
///       <arg name="surface" type="object" interface="wl_surface" summary="the surface" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpLinuxExplicitSynchronizationV1GetSynchronizationRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the new synchronization interface id.
    pub id: u32,
    /// the surface.
    pub surface: u32,
}
impl ZwpLinuxExplicitSynchronizationV1GetSynchronizationRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpLinuxExplicitSynchronizationV1GetSynchronizationRequest {
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
impl InterfaceReader for ZwpLinuxExplicitSynchronizationV1GetSynchronizationRequest {
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
                LinuxExplicitSynchronizationObjectInteface::ZwpLinuxExplicitSynchronizationV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = reader.next_u32().await?;
        let surface = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::LinuxExplicitSynchronization(
                LinuxExplicitSynchronizationObjectInteface::ZwpLinuxSurfaceSynchronizationV1 {
                    version: interface.version().min(2),
                },
            );
            Self::reader_state(state)?
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            surface,
        })
    }
}
impl CommandWriter for ZwpLinuxExplicitSynchronizationV1GetSynchronizationRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "zwp_linux_explicit_synchronization_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::LinuxExplicitSynchronization(
                LinuxExplicitSynchronizationObjectInteface::ZwpLinuxExplicitSynchronizationV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::LinuxExplicitSynchronization(
                LinuxExplicitSynchronizationObjectInteface::ZwpLinuxSurfaceSynchronizationV1 {
                    version: interface.version().min(2),
                },
            );
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_u32(self.surface).await?;
        Ok(())
    }
}
