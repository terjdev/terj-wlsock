use super::ObjectInteface as XdgOutputObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zxdg_output_v1.description`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="description" since="2">
///       <description summary="human-readable description of this output">
///     Many compositors can produce human-readable descriptions of their
///     outputs.  The client may wish to know this description as well, to
///     communicate the user for various purposes.
///
///     The description is a UTF-8 string with no convention defined for its
///     contents. Examples might include 'Foocorp 11" Display' or 'Virtual X11
///     output via :1'.
///
///     The description event is sent after creating an xdg_output (see
///     xdg_output_manager.get_xdg_output) and whenever the description
///     changes. The description is optional, and may not be sent at all.
///
///     For objects of version 2 and lower, this event is only sent once per
///     xdg_output, and the description does not change over the lifetime of
///     the wl_output global.
///
///     This event is deprecated, instead clients should use
///     wl_output.description. Compositors must still support this event.
///       </description>
///       <arg name="description" type="string" summary="output description" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZxdgOutputV1DescriptionEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// output description.
    pub description: String,
}
impl ZxdgOutputV1DescriptionEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 4;
    /// Minimum interface version.
    pub const SINCE: u32 = 2;
}
impl CommandReader<Self> for ZxdgOutputV1DescriptionEvent {
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
impl InterfaceReader for ZxdgOutputV1DescriptionEvent {
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
        let description = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            description,
        })
    }
}
impl CommandWriter for ZxdgOutputV1DescriptionEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_string_size(Some(&self.description))])
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
        if !FdWriter::is_valid_string(&self.description) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.description).await?;
        Ok(())
    }
}
