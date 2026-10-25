use super::ObjectInteface as XdgOutputObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zxdg_output_v1.logical_position`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="logical_position">
///       <description summary="position of the output within the global compositor space">
///     The position event describes the location of the wl_output within
///     the global compositor space.
///
///     The logical_position event is sent after creating an xdg_output
///     (see xdg_output_manager.get_xdg_output) and whenever the location
///     of the output changes within the global compositor space.
///       </description>
///       <arg name="x" type="int" summary="x position within the global compositor space" />
///       <arg name="y" type="int" summary="y position within the global compositor space" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZxdgOutputV1LogicalPositionEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// x position within the global compositor space.
    pub x: i32,
    /// y position within the global compositor space.
    pub y: i32,
}
impl ZxdgOutputV1LogicalPositionEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZxdgOutputV1LogicalPositionEvent {
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
impl InterfaceReader for ZxdgOutputV1LogicalPositionEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgOutput(XdgOutputObjectInteface::ZxdgOutputV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let x = reader.next_i32().await?;
        let y = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, x, y })
    }
}
impl CommandWriter for ZxdgOutputV1LogicalPositionEvent {
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
        let interface =
            Self::writer_event_interface(state, self.object_id, "zxdg_output_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XdgOutput(XdgOutputObjectInteface::ZxdgOutputV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.x).await?;
        writer.next_i32(self.y).await?;
        Ok(())
    }
}
