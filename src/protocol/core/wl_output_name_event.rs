use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_output.name`.
///
/// Opcode 4; available since version 4. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="name" since="4">
///       <description summary="name of this output">
///         Many compositors will assign user-friendly names to their outputs, show
///         them to the user, allow the user to refer to an output, etc. The client
///         may wish to know this name as well to offer the user similar behaviors.
///
///         The name is a UTF-8 string with no convention defined for its contents.
///         Each name is unique among all wl_output globals. The name is only
///         guaranteed to be unique for the compositor instance.
///
///         The same output name is used for all clients for a given wl_output
///         global. Thus, the name can be shared across processes to refer to a
///         specific wl_output global.
///
///         The name is not guaranteed to be persistent across sessions, thus cannot
///         be used to reliably identify an output in e.g. configuration files.
///
///         Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do
///         not assume that the name is a reflection of an underlying DRM connector,
///         X11 connection, etc.
///
///         The name event is sent after binding the output object. This event is
///         only sent once per output object, and the name does not change over the
///         lifetime of the wl_output global.
///
///         Compositors may re-use the same output name if the wl_output global is
///         destroyed and re-created later. Compositors should avoid re-using the
///         same name if possible.
///
///         The name event will be followed by a done event.
///       </description>
///       <arg name="name" type="string" summary="output name" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlOutputNameEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `name`: output name.
    pub name: String,
}
impl WlOutputNameEvent {
    /// Opcode in the event namespace of interface wl_output.
    pub const OPCODE: u16 = 4;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 4;
}
impl CommandWriter for WlOutputNameEvent {
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
        Self::writer_event_interface(state, self.object_id, "wl_output", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.name) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.name).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlOutputNameEvent {
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

impl InterfaceReader for WlOutputNameEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(interface, ObjectInteface::WlOutput { .. })
            || opcode != Self::OPCODE
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
