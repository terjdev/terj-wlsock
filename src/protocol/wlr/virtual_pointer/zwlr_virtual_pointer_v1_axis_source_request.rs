use crate::protocol::core::WlPointerAxisSource;
use crate::protocol::wlr::virtual_pointer::ObjectInteface as WlrVirtualPointerObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_virtual_pointer_v1.axis_source`.
///
/// Opcode 5; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="axis_source">
///       <description summary="axis source event">
///         Source information for scroll and other axis.
///       </description>
///       <arg name="axis_source" type="uint" enum="wl_pointer.axis_source" summary="source of the axis event" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrVirtualPointerV1AxisSourceRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// source of the axis event.
    pub axis_source: WlPointerAxisSource,
}
impl ZwlrVirtualPointerV1AxisSourceRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 5;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrVirtualPointerV1AxisSourceRequest {
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
impl InterfaceReader for ZwlrVirtualPointerV1AxisSourceRequest {
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
        let axis_source = WlPointerAxisSource(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            axis_source,
        })
    }
}
impl CommandWriter for ZwlrVirtualPointerV1AxisSourceRequest {
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
        Self::writer_request_interface(
            state,
            self.object_id,
            "zwlr_virtual_pointer_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.axis_source.0).await?;
        Ok(())
    }
}
