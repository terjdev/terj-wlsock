use super::ObjectInteface as XxCutoutsObjectInteface;
use super::XxCutoutsV1CornerPosition;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xx_cutouts_v1.cutout_corner`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="cutout_corner">
///       <description summary="A cutout corner">
///         The cutout_corner event describes a rounded corner in
///         surface-local coordinates. The area towards the screen edge is
///         the cutout corner part.
///       </description>
///       <arg name="position" type="uint" enum="corner_position" summary="The position of the described corner" />
///       <arg name="radius" type="uint" summary="The corner's radius" />
///       <arg name="id" type="uint" summary="An identifier identifying the physical element" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XxCutoutsV1CutoutCornerEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// The position of the described corner.
    pub position: XxCutoutsV1CornerPosition,
    /// The corner's radius.
    pub radius: u32,
    /// An identifier identifying the physical element.
    pub id: u32,
}
impl XxCutoutsV1CutoutCornerEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxCutoutsV1CutoutCornerEvent {
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
impl InterfaceReader for XxCutoutsV1CutoutCornerEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxCutouts(XxCutoutsObjectInteface::XxCutoutsV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let position = XxCutoutsV1CornerPosition(reader.next_u32().await?);
        let radius = reader.next_u32().await?;
        let id = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            position,
            radius,
            id,
        })
    }
}
impl CommandWriter for XxCutoutsV1CutoutCornerEvent {
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
        let interface =
            Self::writer_event_interface(state, self.object_id, "xx_cutouts_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XxCutouts(XxCutoutsObjectInteface::XxCutoutsV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.position.0).await?;
        writer.next_u32(self.radius).await?;
        writer.next_u32(self.id).await?;
        Ok(())
    }
}
