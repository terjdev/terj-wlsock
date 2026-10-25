use super::ObjectInteface as XxFractionalScaleObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xx_fractional_scale_v2.scale_factor`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="scale_factor">
///       <description summary="set the compositor coordinate space scale factor">
///         This event sets a scale factor for the associated wl_surface that
///         describes the coordinate system the compositor will use for events
///         following xx_fractional_scale_v2.scale_factor.
///
///         The scale factor is encoded in a 8.24 fixed point format.
///
///         The compositor must not send a scale of zero.
///
///         The client should re-render and commit a new buffer with the new scale
///         as soon as possible, in order to avoid artifacts caused by the mismatch
///         in compositor and client scales.
///       </description>
///       <arg name="scale_8_24" type="uint" summary="surface scale factor" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XxFractionalScaleV2ScaleFactorEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// surface scale factor.
    pub scale_8_24: u32,
}
impl XxFractionalScaleV2ScaleFactorEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxFractionalScaleV2ScaleFactorEvent {
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
impl InterfaceReader for XxFractionalScaleV2ScaleFactorEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxFractionalScale(
                XxFractionalScaleObjectInteface::XxFractionalScaleV2 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let scale_8_24 = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            scale_8_24,
        })
    }
}
impl CommandWriter for XxFractionalScaleV2ScaleFactorEvent {
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
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "xx_fractional_scale_v2",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::XxFractionalScale(
                XxFractionalScaleObjectInteface::XxFractionalScaleV2 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.scale_8_24).await?;
        Ok(())
    }
}
