use crate::protocol::core::WlPointerAxis;
use crate::protocol::wlr::virtual_pointer::ObjectInteface as WlrVirtualPointerObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_virtual_pointer_v1.axis_discrete`.
///
/// Opcode 7; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="axis_discrete">
///       <description summary="axis click event">
///         Discrete step information for scroll and other axes.
///
///         This event allows the client to extend data normally sent using the axis
///         event with discrete value.
///       </description>
///       <arg name="time" type="uint" summary="timestamp with millisecond granularity" />
///       <arg name="axis" type="uint" enum="wl_pointer.axis" summary="axis type" />
///       <arg name="value" type="fixed" summary="length of vector in touchpad coordinates" />
///       <arg name="discrete" type="int" summary="number of steps" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrVirtualPointerV1AxisDiscreteRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// timestamp with millisecond granularity.
    pub time: u32,
    /// axis type.
    pub axis: WlPointerAxis,
    /// length of vector in touchpad coordinates.
    pub value: f64,
    /// number of steps.
    pub discrete: i32,
}
impl ZwlrVirtualPointerV1AxisDiscreteRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 7;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrVirtualPointerV1AxisDiscreteRequest {
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
impl InterfaceReader for ZwlrVirtualPointerV1AxisDiscreteRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrVirtualPointer(
                WlrVirtualPointerObjectInteface::ZwlrVirtualPointerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let time = reader.next_u32().await?;
        let axis = WlPointerAxis(reader.next_u32().await?);
        let value = reader.next_fixed().await?;
        let discrete = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            time,
            axis,
            value,
            discrete,
        })
    }
}
impl CommandWriter for ZwlrVirtualPointerV1AxisDiscreteRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(
            state,
            self.object_id,
            "zwlr_virtual_pointer_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.value) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.time).await?;
        writer.next_u32(self.axis.0).await?;
        writer.next_fixed(self.value).await?;
        writer.next_i32(self.discrete).await?;
        Ok(())
    }
}
