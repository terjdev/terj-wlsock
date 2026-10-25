use crate::protocol::ext::xdg_toplevel_icon::ObjectInteface as XdgToplevelIconObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_toplevel_icon_v1.add_buffer`.
///
/// Opcode 2; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="add_buffer">
///       <description summary="add icon data from a pixel buffer">
///         This request adds pixel data supplied as wl_buffer to the icon.
///
///         The client should add pixel data for all icon sizes and scales that
///         it can provide, or which are explicitly requested by the compositor
///         via 'icon_size' events on xdg_toplevel_icon_manager_v1.
///
///         The wl_buffer supplying pixel data as 'buffer' must be backed by wl_shm
///         and must be a square (width and height being equal).
///         If any of these buffer requirements are not fulfilled, a 'invalid_buffer'
///         error must be raised.
///
///         If this icon instance already has a buffer of the same size and scale
///         from a previous 'add_buffer' request, data from the last request
///         overrides the preexisting pixel data.
///
///         The wl_buffer must be kept alive for as long as the xdg_toplevel_icon
///         it is associated with is not destroyed, otherwise a 'no_buffer' error
///         is raised. The buffer contents must not be modified after it was
///         assigned to the icon. As a result, the region of the wl_shm_pool's
///         backing storage used for the wl_buffer must not be modified after this
///         request is sent. The wl_buffer.release event is unused.
///
///         If this request is made after the icon has been assigned to a toplevel
///         via 'set_icon', an 'immutable' error must be raised.
///       </description>
///       <arg name="buffer" type="object" interface="wl_buffer" />
///       <arg name="scale" type="int" summary="the scaling factor of the icon, e.g. 1" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgToplevelIconV1AddBufferRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `buffer`.
    pub buffer: u32,
    /// the scaling factor of the icon, e.g. 1.
    pub scale: i32,
}
impl XdgToplevelIconV1AddBufferRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgToplevelIconV1AddBufferRequest {
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
impl InterfaceReader for XdgToplevelIconV1AddBufferRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgToplevelIcon(
                XdgToplevelIconObjectInteface::XdgToplevelIconV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let buffer = reader.next_u32().await?;
        let scale = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            buffer,
            scale,
        })
    }
}
impl CommandWriter for XdgToplevelIconV1AddBufferRequest {
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
        Self::writer_request_interface(state, self.object_id, "xdg_toplevel_icon_v1", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.buffer).await?;
        writer.next_i32(self.scale).await?;
        Ok(())
    }
}
