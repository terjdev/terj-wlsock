use super::ObjectInteface as InputTimestampsObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_input_timestamps_v1.timestamp`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="timestamp">
///       <description summary="high-resolution timestamp event">
///         The timestamp event is associated with the first subsequent input event
///         carrying a timestamp which belongs to the set of input events this
///         object is subscribed to.
///
///         The timestamp provided by this event is a high-resolution version of
///         the timestamp argument of the associated input event. The provided
///         timestamp is in the same clock domain and is at least as accurate as
///         the associated input event timestamp.
///
///         The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,
///         each component being an unsigned 32-bit value. Whole seconds are in
///         tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,
///         and the additional fractional part in tv_nsec as nanoseconds. Hence,
///         for valid timestamps tv_nsec must be in [0, 999999999].
///       </description>
///       <arg name="tv_sec_hi" type="uint" summary="high 32 bits of the seconds part of the timestamp" />
///       <arg name="tv_sec_lo" type="uint" summary="low 32 bits of the seconds part of the timestamp" />
///       <arg name="tv_nsec" type="uint" summary="nanoseconds part of the timestamp" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpInputTimestampsV1TimestampEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// high 32 bits of the seconds part of the timestamp.
    pub tv_sec_hi: u32,
    /// low 32 bits of the seconds part of the timestamp.
    pub tv_sec_lo: u32,
    /// nanoseconds part of the timestamp.
    pub tv_nsec: u32,
}
impl ZwpInputTimestampsV1TimestampEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpInputTimestampsV1TimestampEvent {
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
impl InterfaceReader for ZwpInputTimestampsV1TimestampEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::InputTimestamps(
                InputTimestampsObjectInteface::ZwpInputTimestampsV1 { .. }
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
impl CommandWriter for ZwpInputTimestampsV1TimestampEvent {
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
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "zwp_input_timestamps_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::InputTimestamps(
                InputTimestampsObjectInteface::ZwpInputTimestampsV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.tv_sec_hi).await?;
        writer.next_u32(self.tv_sec_lo).await?;
        writer.next_u32(self.tv_nsec).await?;
        Ok(())
    }
}
