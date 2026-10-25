use crate::protocol::wlr::virtual_pointer::ObjectInteface as WlrVirtualPointerObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_virtual_pointer_manager_v1.create_virtual_pointer_with_output`.
///
/// Opcode 2; available since version 2.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="create_virtual_pointer_with_output" since="2">
///       <description summary="Create a new virtual pointer">
///         Creates a new virtual pointer. The seat and the output arguments are
///         optional. If the seat argument is set, the compositor should assign the
///         input device to the requested seat. If the output argument is set, the
///         compositor should map the input device to the requested output.
///       </description>
///       <arg name="seat" type="object" interface="wl_seat" allow-null="true" />
///       <arg name="output" type="object" interface="wl_output" allow-null="true" />
///       <arg name="id" type="new_id" interface="zwlr_virtual_pointer_v1" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrVirtualPointerManagerV1CreateVirtualPointerWithOutputRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `seat`.
    pub seat: Option<u32>,
    /// Wire argument `output`.
    pub output: Option<u32>,
    /// Wire argument `id`.
    pub id: u32,
}
impl ZwlrVirtualPointerManagerV1CreateVirtualPointerWithOutputRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 2;
}
impl CommandReader<Self> for ZwlrVirtualPointerManagerV1CreateVirtualPointerWithOutputRequest {
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
impl InterfaceReader for ZwlrVirtualPointerManagerV1CreateVirtualPointerWithOutputRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrVirtualPointer(
                WlrVirtualPointerObjectInteface::ZwlrVirtualPointerManagerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let seat = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        let output = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        let id = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("zwlr_virtual_pointer_v1", interface.version().min(2))
                    .ok_or(FdReaderError::InvalidProtocol)?;
            Self::reader_state(state)?
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            seat,
            output,
            id,
        })
    }
}
impl CommandWriter for ZwlrVirtualPointerManagerV1CreateVirtualPointerWithOutputRequest {
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
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "zwlr_virtual_pointer_manager_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("zwlr_virtual_pointer_v1", interface.version().min(2))
                    .ok_or(FdWriterError::InvalidProtocol)?;
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.seat.unwrap_or(0)).await?;
        writer.next_u32(self.output.unwrap_or(0)).await?;
        writer.next_u32(self.id).await?;
        Ok(())
    }
}
