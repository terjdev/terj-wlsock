use crate::protocol::wlr::screencopy::ObjectInteface as WlrScreencopyObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_screencopy_frame_v1.ready`.
///
/// Opcode 2; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="ready">
///       <description summary="indicates frame is available for reading">
///         Called as soon as the frame is copied, indicating it is available
///         for reading. This event includes the time at which the presentation took place.
///
///         The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,
///         each component being an unsigned 32-bit value. Whole seconds are in
///         tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,
///         and the additional fractional part in tv_nsec as nanoseconds. Hence,
///         for valid timestamps tv_nsec must be in [0, 999999999]. The seconds part
///         may have an arbitrary offset at start.
///
///         After receiving this event, the client should destroy the object.
///       </description>
///       <arg name="tv_sec_hi" type="uint" summary="high 32 bits of the seconds part of the timestamp" />
///       <arg name="tv_sec_lo" type="uint" summary="low 32 bits of the seconds part of the timestamp" />
///       <arg name="tv_nsec" type="uint" summary="nanoseconds part of the timestamp" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrScreencopyFrameV1ReadyEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// high 32 bits of the seconds part of the timestamp.
    pub tv_sec_hi: u32,
    /// low 32 bits of the seconds part of the timestamp.
    pub tv_sec_lo: u32,
    /// nanoseconds part of the timestamp.
    pub tv_nsec: u32,
}
impl ZwlrScreencopyFrameV1ReadyEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrScreencopyFrameV1ReadyEvent {
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
impl InterfaceReader for ZwlrScreencopyFrameV1ReadyEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrScreencopy(
                WlrScreencopyObjectInteface::ZwlrScreencopyFrameV1 { .. }
            )
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
impl CommandWriter for ZwlrScreencopyFrameV1ReadyEvent {
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
        Self::writer_event_interface(
            state,
            self.object_id,
            "zwlr_screencopy_frame_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.tv_sec_hi).await?;
        writer.next_u32(self.tv_sec_lo).await?;
        writer.next_u32(self.tv_nsec).await?;
        Ok(())
    }
}
