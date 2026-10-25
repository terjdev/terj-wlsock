use super::ObjectInteface as PointerConstraintsObjectInteface;
use super::ZwpPointerConstraintsV1Lifetime;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_pointer_constraints_v1.lock_pointer`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="lock_pointer">
///       <description summary="lock pointer to a position">
///     The lock_pointer request lets the client request to disable movements of
///     the virtual pointer (i.e. the cursor), effectively locking the pointer
///     to a position. This request may not take effect immediately; in the
///     future, when the compositor deems implementation-specific constraints
///     are satisfied, the pointer lock will be activated and the compositor
///     sends a locked event.
///
///     The protocol provides no guarantee that the constraints are ever
///     satisfied, and does not require the compositor to send an error if the
///     constraints cannot ever be satisfied. It is thus possible to request a
///     lock that will never activate.
///
///     There may not be another pointer constraint of any kind requested or
///     active on the surface for any of the wl_pointer objects of the seat of
///     the passed pointer when requesting a lock. If there is, an error will be
///     raised. See general pointer lock documentation for more details.
///
///     The intersection of the region passed with this request and the input
///     region of the surface is used to determine where the pointer must be
///     in order for the lock to activate. It is up to the compositor whether to
///     warp the pointer or require some kind of user interaction for the lock
///     to activate. If the region is null the surface input region is used.
///
///     A surface may receive pointer focus without the lock being activated.
///
///     The request creates a new object wp_locked_pointer which is used to
///     interact with the lock as well as receive updates about its state. See
///     the description of wp_locked_pointer for further information.
///
///     Note that while a pointer is locked, the wl_pointer objects of the
///     corresponding seat will not emit any wl_pointer.motion events, but
///     relative motion events will still be emitted via wp_relative_pointer
///     objects of the same seat. wl_pointer.axis and wl_pointer.button events
///     are unaffected.
///       </description>
///       <arg name="id" type="new_id" interface="zwp_locked_pointer_v1" />
///       <arg name="surface" type="object" interface="wl_surface" summary="surface to lock pointer to" />
///       <arg name="pointer" type="object" interface="wl_pointer" summary="the pointer that should be locked" />
///       <arg name="region" type="object" interface="wl_region" allow-null="true" summary="region of surface" />
///       <arg name="lifetime" type="uint" enum="lifetime" summary="lock lifetime" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpPointerConstraintsV1LockPointerRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `id`.
    pub id: u32,
    /// surface to lock pointer to.
    pub surface: u32,
    /// the pointer that should be locked.
    pub pointer: u32,
    /// region of surface.
    pub region: Option<u32>,
    /// lock lifetime.
    pub lifetime: ZwpPointerConstraintsV1Lifetime,
}
impl ZwpPointerConstraintsV1LockPointerRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpPointerConstraintsV1LockPointerRequest {
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
impl InterfaceReader for ZwpPointerConstraintsV1LockPointerRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::PointerConstraints(
                PointerConstraintsObjectInteface::ZwpPointerConstraintsV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = reader.next_u32().await?;
        let surface = reader.next_u32().await?;
        let pointer = reader.next_u32().await?;
        let region = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        let lifetime = ZwpPointerConstraintsV1Lifetime(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::PointerConstraints(
                PointerConstraintsObjectInteface::ZwpLockedPointerV1 {
                    version: interface.version().min(1),
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
            pointer,
            region,
            lifetime,
        })
    }
}
impl CommandWriter for ZwpPointerConstraintsV1LockPointerRequest {
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
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "zwp_pointer_constraints_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::PointerConstraints(
                PointerConstraintsObjectInteface::ZwpPointerConstraintsV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::PointerConstraints(
                PointerConstraintsObjectInteface::ZwpLockedPointerV1 {
                    version: interface.version().min(1),
                },
            );
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_u32(self.surface).await?;
        writer.next_u32(self.pointer).await?;
        writer.next_u32(self.region.unwrap_or(0)).await?;
        writer.next_u32(self.lifetime.0).await?;
        Ok(())
    }
}
