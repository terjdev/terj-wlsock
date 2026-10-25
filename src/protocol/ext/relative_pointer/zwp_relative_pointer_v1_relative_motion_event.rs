use super::ObjectInteface as RelativePointerObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_relative_pointer_v1.relative_motion`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="relative_motion">
///       <description summary="relative pointer motion">
///     Relative x/y pointer motion from the pointer of the seat associated with
///     this object.
///
///     A relative motion is in the same dimension as regular wl_pointer motion
///     events, except they do not represent an absolute position. For example,
///     moving a pointer from (x, y) to (x', y') would have the equivalent
///     relative motion (x' - x, y' - y). If a pointer motion caused the
///     absolute pointer position to be clipped by for example the edge of the
///     monitor, the relative motion is unaffected by the clipping and will
///     represent the unclipped motion.
///
///     This event also contains non-accelerated motion deltas. The
///     non-accelerated delta is, when applicable, the regular pointer motion
///     delta as it was before having applied motion acceleration and other
///     transformations such as normalization.
///
///     Note that the non-accelerated delta does not represent 'raw' events as
///     they were read from some device. Pointer motion acceleration is device-
///     and configuration-specific and non-accelerated deltas and accelerated
///     deltas may have the same value on some devices.
///
///     Relative motions are not coupled to wl_pointer.motion events, and can be
///     sent in combination with such events, but also independently. There may
///     also be scenarios where wl_pointer.motion is sent, but there is no
///     relative motion. The order of an absolute and relative motion event
///     originating from the same physical motion is not guaranteed.
///
///     If the client needs button events or focus state, it can receive them
///     from a wl_pointer object of the same seat that the wp_relative_pointer
///     object is associated with.
///       </description>
///       <arg name="utime_hi" type="uint" summary="high 32 bits of a 64 bit timestamp with microsecond granularity" />
///       <arg name="utime_lo" type="uint" summary="low 32 bits of a 64 bit timestamp with microsecond granularity" />
///       <arg name="dx" type="fixed" summary="the x component of the motion vector" />
///       <arg name="dy" type="fixed" summary="the y component of the motion vector" />
///       <arg name="dx_unaccel" type="fixed" summary="the x component of the unaccelerated motion vector" />
///       <arg name="dy_unaccel" type="fixed" summary="the y component of the unaccelerated motion vector" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpRelativePointerV1RelativeMotionEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// high 32 bits of a 64 bit timestamp with microsecond granularity.
    pub utime_hi: u32,
    /// low 32 bits of a 64 bit timestamp with microsecond granularity.
    pub utime_lo: u32,
    /// the x component of the motion vector.
    pub dx: f64,
    /// the y component of the motion vector.
    pub dy: f64,
    /// the x component of the unaccelerated motion vector.
    pub dx_unaccel: f64,
    /// the y component of the unaccelerated motion vector.
    pub dy_unaccel: f64,
}
impl ZwpRelativePointerV1RelativeMotionEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpRelativePointerV1RelativeMotionEvent {
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
impl InterfaceReader for ZwpRelativePointerV1RelativeMotionEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::RelativePointer(
                RelativePointerObjectInteface::ZwpRelativePointerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let utime_hi = reader.next_u32().await?;
        let utime_lo = reader.next_u32().await?;
        let dx = reader.next_fixed().await?;
        let dy = reader.next_fixed().await?;
        let dx_unaccel = reader.next_fixed().await?;
        let dy_unaccel = reader.next_fixed().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            utime_hi,
            utime_lo,
            dx,
            dy,
            dx_unaccel,
            dy_unaccel,
        })
    }
}
impl CommandWriter for ZwpRelativePointerV1RelativeMotionEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4), Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "zwp_relative_pointer_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::RelativePointer(
                RelativePointerObjectInteface::ZwpRelativePointerV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.dx) {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.dy) {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.dx_unaccel) {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.dy_unaccel) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.utime_hi).await?;
        writer.next_u32(self.utime_lo).await?;
        writer.next_fixed(self.dx).await?;
        writer.next_fixed(self.dy).await?;
        writer.next_fixed(self.dx_unaccel).await?;
        writer.next_fixed(self.dy_unaccel).await?;
        Ok(())
    }
}
