use super::ObjectInteface as XdgOutputObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zxdg_output_v1.name`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="name" since="2">
///       <description summary="name of this output">
///     Many compositors will assign names to their outputs, show them to the
///     user, allow them to be configured by name, etc. The client may wish to
///     know this name as well to offer the user similar behaviors.
///
///     The naming convention is compositor defined, but limited to
///     alphanumeric characters and dashes (-). Each name is unique among all
///     wl_output globals, but if a wl_output global is destroyed the same name
///     may be reused later. The names will also remain consistent across
///     sessions with the same hardware and software configuration.
///
///     Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do
///     not assume that the name is a reflection of an underlying DRM
///     connector, X11 connection, etc.
///
///     The name event is sent after creating an xdg_output (see
///     xdg_output_manager.get_xdg_output). This event is only sent once per
///     xdg_output, and the name does not change over the lifetime of the
///     wl_output global.
///
///         This event is deprecated, instead clients should use wl_output.name.
///         Compositors must still support this event.
///       </description>
///       <arg name="name" type="string" summary="output name" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZxdgOutputV1NameEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// output name.
    pub name: String,
}
impl ZxdgOutputV1NameEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version.
    pub const SINCE: u32 = 2;
}
impl CommandReader<Self> for ZxdgOutputV1NameEvent {
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
impl InterfaceReader for ZxdgOutputV1NameEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgOutput(XdgOutputObjectInteface::ZxdgOutputV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let name = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, name })
    }
}
impl CommandWriter for ZxdgOutputV1NameEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_string_size(Some(&self.name))])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface =
            Self::writer_event_interface(state, self.object_id, "zxdg_output_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XdgOutput(XdgOutputObjectInteface::ZxdgOutputV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.name) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.name).await?;
        Ok(())
    }
}
