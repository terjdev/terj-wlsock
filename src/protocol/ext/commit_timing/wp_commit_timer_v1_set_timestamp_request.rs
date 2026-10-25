use crate::protocol::ext::commit_timing::ObjectInteface as CommitTimingObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_commit_timer_v1.set_timestamp`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_timestamp">
///       <description summary="Specify time the following commit takes effect">
///         Provide a timing constraint for a surface content update.
///
///         A set_timestamp request may be made before a wl_surface.commit to
///         tell the compositor that the content is intended to be presented
///         as closely as possible to, but not before, the specified time.
///         The time is in the domain of the compositor's presentation clock.
///
///         An invalid_timestamp error will be generated for invalid tv_nsec.
///
///         If a timestamp already exists on the surface, a timestamp_exists
///         error is generated.
///
///         Requesting set_timestamp after the commit_timer object's surface is
///         destroyed will generate a "surface_destroyed" error.
///       </description>
///       <arg name="tv_sec_hi" type="uint" summary="high 32 bits of the seconds part of target time" />
///       <arg name="tv_sec_lo" type="uint" summary="low 32 bits of the seconds part of target time" />
///       <arg name="tv_nsec" type="uint" summary="nanoseconds part of target time" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpCommitTimerV1SetTimestampRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// high 32 bits of the seconds part of target time.
    pub tv_sec_hi: u32,
    /// low 32 bits of the seconds part of target time.
    pub tv_sec_lo: u32,
    /// nanoseconds part of target time.
    pub tv_nsec: u32,
}
impl WpCommitTimerV1SetTimestampRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpCommitTimerV1SetTimestampRequest {
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
impl InterfaceReader for WpCommitTimerV1SetTimestampRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::CommitTiming(CommitTimingObjectInteface::WpCommitTimerV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let tv_sec_hi = reader.next_u32().await?;
        let tv_sec_lo = reader.next_u32().await?;
        let tv_nsec = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
        })
    }
}
impl CommandWriter for WpCommitTimerV1SetTimestampRequest {
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
        Self::writer_request_interface(state, self.object_id, "wp_commit_timer_v1", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.tv_sec_hi).await?;
        writer.next_u32(self.tv_sec_lo).await?;
        writer.next_u32(self.tv_nsec).await?;
        Ok(())
    }
}
