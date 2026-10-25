use crate::protocol::ext::xdg_toplevel_icon::ObjectInteface as XdgToplevelIconObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_toplevel_icon_v1.set_name`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_name">
///       <description summary="set an icon name">
///         This request assigns an icon name to this icon.
///         Any previously set name is overridden.
///
///         The compositor must resolve 'icon_name' according to the lookup rules
///         described in the XDG icon theme specification[1] using the
///         environment's current icon theme.
///
///         If the compositor does not support icon names or cannot resolve
///         'icon_name' according to the XDG icon theme specification it must
///         fall back to using pixel buffer data instead.
///
///         If this request is made after the icon has been assigned to a toplevel
///         via 'set_icon', an 'immutable' error must be raised.
///
///         [1]: https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html
///       </description>
///       <arg name="icon_name" type="string" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgToplevelIconV1SetNameRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `icon_name`.
    pub icon_name: String,
}
impl XdgToplevelIconV1SetNameRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgToplevelIconV1SetNameRequest {
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
impl InterfaceReader for XdgToplevelIconV1SetNameRequest {
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
        let icon_name = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            icon_name,
        })
    }
}
impl CommandWriter for XdgToplevelIconV1SetNameRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_string_size(Some(&self.icon_name))])
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
        if !FdWriter::is_valid_string(&self.icon_name) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.icon_name).await?;
        Ok(())
    }
}
