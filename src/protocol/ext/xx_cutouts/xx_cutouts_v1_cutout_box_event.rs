use super::ObjectInteface as XxCutoutsObjectInteface;
use super::XxCutoutsV1Type;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xx_cutouts_v1.cutout_box`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="cutout_box">
///       <description summary="A rectangular cutout region">
///         The cutout_box event describes a rectangular cutout area in
///         surface-local coordinates.
///
///         This can be an approximation of e.g. a circular camera notch.
///       </description>
///       <arg name="x" type="int" summary="x coordinate of the box's top left corner" />
///       <arg name="y" type="int" summary="y coordinate of the box's top left corner" />
///       <arg name="width" type="int" />
///       <arg name="height" type="int" />
///       <arg name="type" type="uint" enum="type" summary="The type of cutout" />
///       <arg name="id" type="uint" summary="An identifier identifying the physical element" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XxCutoutsV1CutoutBoxEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// x coordinate of the box's top left corner.
    pub x: i32,
    /// y coordinate of the box's top left corner.
    pub y: i32,
    /// Wire argument `width`.
    pub width: i32,
    /// Wire argument `height`.
    pub height: i32,
    /// The type of cutout.
    pub r#type: XxCutoutsV1Type,
    /// An identifier identifying the physical element.
    pub id: u32,
}
impl XxCutoutsV1CutoutBoxEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxCutoutsV1CutoutBoxEvent {
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
impl InterfaceReader for XxCutoutsV1CutoutBoxEvent {
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
        let x = reader.next_i32().await?;
        let y = reader.next_i32().await?;
        let width = reader.next_i32().await?;
        let height = reader.next_i32().await?;
        let r#type = XxCutoutsV1Type(reader.next_u32().await?);
        let id = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            x,
            y,
            width,
            height,
            r#type,
            id,
        })
    }
}
impl CommandWriter for XxCutoutsV1CutoutBoxEvent {
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
        let interface =
            Self::writer_event_interface(state, self.object_id, "xx_cutouts_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XxCutouts(XxCutoutsObjectInteface::XxCutoutsV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.x).await?;
        writer.next_i32(self.y).await?;
        writer.next_i32(self.width).await?;
        writer.next_i32(self.height).await?;
        writer.next_u32(self.r#type.0).await?;
        writer.next_u32(self.id).await?;
        Ok(())
    }
}
