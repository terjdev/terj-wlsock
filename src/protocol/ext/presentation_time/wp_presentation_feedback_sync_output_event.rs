use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wp_presentation_feedback.sync_output`.
///
/// Opcode 0; available since version 1.
/// Arguments follow XML wire order. Receiver metadata and payload framing are checked.
/// Compositor resources, object arguments, and presentation policy belong to the caller.
/// ```xml
/// <event name="sync_output">
///       <description summary="presentation synchronized to this output">
///         As presentation can be synchronized to only one output at a
///         time, this event tells which output it was. This event is only
///         sent prior to the presented event.
///
///         As clients may bind to the same global wl_output multiple
///         times, this event is sent for each bound instance that matches
///         the synchronized output. If a client has not bound to the
///         right wl_output global at all, this event is not sent.
///       </description>
///       <arg name="output" type="object" interface="wl_output" summary="presentation output" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WpPresentationFeedbackSyncOutputEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `output`: presentation output.
    /// Object implementing `wl_output`.
    pub output: u32,
}
impl WpPresentationFeedbackSyncOutputEvent {
    /// Opcode in the event namespace of `wp_presentation_feedback`.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpPresentationFeedbackSyncOutputEvent {
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

impl InterfaceReader for WpPresentationFeedbackSyncOutputEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(interface, ObjectInteface::WpPresentationFeedback { .. })
            || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let output = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, output })
    }
}

impl CommandWriter for WpPresentationFeedbackSyncOutputEvent {
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
            "wp_presentation_feedback",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.output).await?;
        Ok(())
    }
}
