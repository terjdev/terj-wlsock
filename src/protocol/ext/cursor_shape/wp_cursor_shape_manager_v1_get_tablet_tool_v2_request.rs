use crate::protocol::ext::cursor_shape::ObjectInteface as CursorShapeObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_cursor_shape_manager_v1.get_tablet_tool_v2`.
///
/// Opcode 2; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="get_tablet_tool_v2">
///       <description summary="manage the cursor shape of a tablet tool device">
///         Obtain a wp_cursor_shape_device_v1 for a zwp_tablet_tool_v2 object.
///
///         When the zwp_tablet_tool_v2 is removed, the wp_cursor_shape_device_v1
///         object becomes inert.
///       </description>
///       <arg name="cursor_shape_device" type="new_id" interface="wp_cursor_shape_device_v1" />
///       <arg name="tablet_tool" type="object" interface="zwp_tablet_tool_v2" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpCursorShapeManagerV1GetTabletToolV2Request {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `cursor_shape_device`.
    pub cursor_shape_device: u32,
    /// Wire argument `tablet_tool`.
    pub tablet_tool: u32,
}
impl WpCursorShapeManagerV1GetTabletToolV2Request {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpCursorShapeManagerV1GetTabletToolV2Request {
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
impl InterfaceReader for WpCursorShapeManagerV1GetTabletToolV2Request {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::CursorShape(CursorShapeObjectInteface::WpCursorShapeManagerV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let cursor_shape_device = reader.next_u32().await?;
        let tablet_tool = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("wp_cursor_shape_device_v1", interface.version().min(2))
                    .ok_or(FdReaderError::InvalidProtocol)?;
            Self::reader_state(state)?
                .register_object(cursor_shape_device, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            cursor_shape_device,
            tablet_tool,
        })
    }
}
impl CommandWriter for WpCursorShapeManagerV1GetTabletToolV2Request {
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
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "wp_cursor_shape_manager_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("wp_cursor_shape_device_v1", interface.version().min(2))
                    .ok_or(FdWriterError::InvalidProtocol)?;
            Self::writer_state(state)?
                .register_object(self.cursor_shape_device, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.cursor_shape_device).await?;
        writer.next_u32(self.tablet_tool).await?;
        Ok(())
    }
}
