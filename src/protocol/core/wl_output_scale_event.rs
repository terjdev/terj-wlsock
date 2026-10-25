use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_output.scale`.
///
/// Opcode 3; available since version 2. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="scale" since="2">
///       <description summary="output scaling properties">
///         This event contains scaling geometry information
///         that is not in the geometry event. It may be sent after
///         binding the output object or if the output scale changes
///         later. The compositor will emit a non-zero, positive
///         value for scale. If it is not sent, the client should
///         assume a scale of 1.
///
///         A scale larger than 1 means that the compositor will
///         automatically scale surface buffers by this amount
///         when rendering. This is used for very high resolution
///         displays where applications rendering at the native
///         resolution would be too small to be legible.
///
///         Clients should use wl_surface.preferred_buffer_scale
///         instead of this event to find the preferred buffer
///         scale to use for a surface.
///
///         The scale event will be followed by a done event.
///       </description>
///       <arg name="factor" type="int" summary="scaling factor of output" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlOutputScaleEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `factor`: scaling factor of output.
    pub factor: i32,
}
impl WlOutputScaleEvent {
    /// Opcode in the event namespace of interface wl_output.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 2;
}
impl CommandWriter for WlOutputScaleEvent {
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
        Self::writer_event_interface(state, self.object_id, "wl_output", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.factor).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlOutputScaleEvent {
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

impl InterfaceReader for WlOutputScaleEvent {
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
        let factor = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, factor })
    }
}
