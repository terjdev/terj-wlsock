use crate::protocol::ext::xdg_toplevel_icon::ObjectInteface as XdgToplevelIconObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xdg_toplevel_icon_manager_v1.icon_size`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="icon_size">
///       <description summary="describes a supported &amp; preferred icon size">
///         This event indicates an icon size the compositor prefers to be
///         available if the client has scalable icons and can render to any size.
///
///         When the 'xdg_toplevel_icon_manager_v1' object is created, the
///         compositor may send one or more 'icon_size' events to describe the list
///         of preferred icon sizes. If the compositor has no size preference, it
///         may not send any 'icon_size' event, and it is up to the client to
///         decide a suitable icon size.
///
///         A sequence of 'icon_size' events must be finished with a 'done' event.
///         If the compositor has no size preferences, it must still send the
///         'done' event, without any preceding 'icon_size' events.
///       </description>
///       <arg name="size" type="int" summary="the edge size of the square icon in surface-local coordinates, e.g. 64" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XdgToplevelIconManagerV1IconSizeEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the edge size of the square icon in surface-local coordinates, e.g. 64.
    pub size: i32,
}
impl XdgToplevelIconManagerV1IconSizeEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgToplevelIconManagerV1IconSizeEvent {
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
impl InterfaceReader for XdgToplevelIconManagerV1IconSizeEvent {
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
                XdgToplevelIconObjectInteface::XdgToplevelIconManagerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let size = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, size })
    }
}
impl CommandWriter for XdgToplevelIconManagerV1IconSizeEvent {
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
        Self::writer_event_interface(
            state,
            self.object_id,
            "xdg_toplevel_icon_manager_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.size).await?;
        Ok(())
    }
}
