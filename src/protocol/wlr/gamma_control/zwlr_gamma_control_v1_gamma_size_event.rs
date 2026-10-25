use super::ObjectInteface as WlrGammaControlObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_gamma_control_v1.gamma_size`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="gamma_size">
///       <description summary="size of gamma ramps">
///         Advertise the size of each gamma ramp.
///
///         This event is sent immediately when the gamma control object is created.
///       </description>
///       <arg name="size" type="uint" summary="number of elements in a ramp" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrGammaControlV1GammaSizeEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// number of elements in a ramp.
    pub size: u32,
}
impl ZwlrGammaControlV1GammaSizeEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrGammaControlV1GammaSizeEvent {
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
impl InterfaceReader for ZwlrGammaControlV1GammaSizeEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrGammaControl(
                WlrGammaControlObjectInteface::ZwlrGammaControlV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let size = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, size })
    }
}
impl CommandWriter for ZwlrGammaControlV1GammaSizeEvent {
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
            "zwlr_gamma_control_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::WlrGammaControl(
                WlrGammaControlObjectInteface::ZwlrGammaControlV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.size).await?;
        Ok(())
    }
}
