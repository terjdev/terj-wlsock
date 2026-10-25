use super::ObjectInteface as WlrOutputManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_output_head_v1.name`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="name">
///       <description summary="head name">
///         This event describes the head name.
///
///         The naming convention is compositor defined, but limited to alphanumeric
///         characters and dashes (-). Each name is unique among all wlr_output_head
///         objects, but if a wlr_output_head object is destroyed the same name may
///         be reused later. The names will also remain consistent across sessions
///         with the same hardware and software configuration.
///
///         Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do
///         not assume that the name is a reflection of an underlying DRM
///         connector, X11 connection, etc.
///
///         If this head matches a wl_output, the wl_output.name event must report
///         the same name.
///
///         The name event is sent after a wlr_output_head object is created. This
///         event is only sent once per object, and the name does not change over
///         the lifetime of the wlr_output_head object.
///       </description>
///       <arg name="name" type="string" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrOutputHeadV1NameEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `name`.
    pub name: String,
}
impl ZwlrOutputHeadV1NameEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrOutputHeadV1NameEvent {
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
impl InterfaceReader for ZwlrOutputHeadV1NameEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputHeadV1 { .. }
            )
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
impl CommandWriter for ZwlrOutputHeadV1NameEvent {
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
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "zwlr_output_head_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputHeadV1 { .. }
            )
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
