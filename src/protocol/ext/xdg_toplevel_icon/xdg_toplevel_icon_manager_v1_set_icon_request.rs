use crate::protocol::ext::xdg_toplevel_icon::ObjectInteface as XdgToplevelIconObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_toplevel_icon_manager_v1.set_icon`.
///
/// Opcode 2; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_icon">
///       <description summary="set an icon on a toplevel window">
///         This request assigns the icon 'icon' to 'toplevel', or clears the
///         toplevel icon if 'icon' was null.
///         This state is double-buffered and is applied on the next
///         wl_surface.commit of the toplevel.
///
///         After making this call, the xdg_toplevel_icon_v1 provided as 'icon'
///         can be destroyed by the client without 'toplevel' losing its icon.
///         The xdg_toplevel_icon_v1 is immutable from this point, and any
///         future attempts to change it must raise the
///         'xdg_toplevel_icon_v1.immutable' protocol error.
///
///         The compositor must set the toplevel icon from either the pixel data
///         the icon provides, or by loading a stock icon using the icon name.
///         See the description of 'xdg_toplevel_icon_v1' for details.
///
///         If 'icon' is set to null, the icon of the respective toplevel is reset
///         to its default icon (usually the icon of the application, derived from
///         its desktop-entry file, or a placeholder icon).
///         If this request is passed an icon with no pixel buffers or icon name
///         assigned, the icon must be reset just like if 'icon' was null.
///       </description>
///       <arg name="toplevel" type="object" interface="xdg_toplevel" summary="the toplevel to act on" />
///       <arg name="icon" type="object" interface="xdg_toplevel_icon_v1" allow-null="true" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgToplevelIconManagerV1SetIconRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the toplevel to act on.
    pub toplevel: u32,
    /// Wire argument `icon`.
    pub icon: Option<u32>,
}
impl XdgToplevelIconManagerV1SetIconRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgToplevelIconManagerV1SetIconRequest {
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
impl InterfaceReader for XdgToplevelIconManagerV1SetIconRequest {
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
        let toplevel = reader.next_u32().await?;
        let icon = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            toplevel,
            icon,
        })
    }
}
impl CommandWriter for XdgToplevelIconManagerV1SetIconRequest {
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
        Self::writer_request_interface(
            state,
            self.object_id,
            "xdg_toplevel_icon_manager_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.toplevel).await?;
        writer.next_u32(self.icon.unwrap_or(0)).await?;
        Ok(())
    }
}
