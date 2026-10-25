use crate::protocol::ext::linux_drm_syncobj::ObjectInteface as LinuxDrmSyncobjObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_linux_drm_syncobj_surface_v1.set_release_point`.
///
/// Opcode 2; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_release_point">
///       <description summary="set the release timeline point">
///         Set the timeline point that must be signalled by the compositor when it
///         has finished its usage of the buffer attached with wl_surface.attach
///         for the relevant commit.
///
///         Once the timeline point is signaled, and assuming the associated buffer
///         is not pending release from other wl_surface.commit requests, no
///         additional explicit or implicit synchronization with the compositor is
///         required to safely re-use the buffer.
///
///         Note that clients cannot rely on the release point being always
///         signaled after the acquire point: compositors may release buffers
///         without ever reading from them. In addition, the compositor may use
///         different presentation paths for different commits, which may have
///         different release behavior. As a result, the compositor may signal the
///         release points in a different order than the client committed them.
///
///         Because signaling a timeline point also signals every previous point,
///         it is generally not safe to use the same timeline object for the
///         release points of multiple buffers. The out-of-order signaling
///         described above may lead to a release point being signaled before the
///         compositor has finished reading. To avoid this, it is strongly
///         recommended that each buffer should use a separate timeline for its
///         release points.
///
///         The 64-bit unsigned value combined from point_hi and point_lo is the
///         point value.
///
///         The release point is double-buffered state, and will be applied on the
///         next wl_surface.commit request for the associated surface. Thus, it
///         applies only to the buffer that is attached to the surface at commit
///         time.
///
///         If a release point has already been attached during the same commit
///         cycle, the new point replaces the old one.
///
///         If the associated wl_surface was destroyed, a no_surface error is
///         raised.
///
///         If at surface commit time there is a pending release timeline point set
///         but no pending buffer attached, a no_buffer error is raised. If at
///         surface commit time there is a pending buffer attached but no pending
///         release timeline point set, the no_release_point protocol error is
///         raised.
///       </description>
///       <arg name="timeline" type="object" interface="wp_linux_drm_syncobj_timeline_v1" />
///       <arg name="point_hi" type="uint" summary="high 32 bits of the point value" />
///       <arg name="point_lo" type="uint" summary="low 32 bits of the point value" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpLinuxDrmSyncobjSurfaceV1SetReleasePointRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `timeline`.
    pub timeline: u32,
    /// high 32 bits of the point value.
    pub point_hi: u32,
    /// low 32 bits of the point value.
    pub point_lo: u32,
}
impl WpLinuxDrmSyncobjSurfaceV1SetReleasePointRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpLinuxDrmSyncobjSurfaceV1SetReleasePointRequest {
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
impl InterfaceReader for WpLinuxDrmSyncobjSurfaceV1SetReleasePointRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::LinuxDrmSyncobj(
                LinuxDrmSyncobjObjectInteface::WpLinuxDrmSyncobjSurfaceV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let timeline = reader.next_u32().await?;
        let point_hi = reader.next_u32().await?;
        let point_lo = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            timeline,
            point_hi,
            point_lo,
        })
    }
}
impl CommandWriter for WpLinuxDrmSyncobjSurfaceV1SetReleasePointRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(
            state,
            self.object_id,
            "wp_linux_drm_syncobj_surface_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.timeline).await?;
        writer.next_u32(self.point_hi).await?;
        writer.next_u32(self.point_lo).await?;
        Ok(())
    }
}
