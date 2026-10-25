use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wp_presentation_feedback.discarded`.
///
/// Opcode 2; available since version 1.
/// Arguments follow XML wire order. Receiver metadata and payload framing are checked.
/// Compositor resources, object arguments, and presentation policy belong to the caller.
/// Ends the receiver lifetime; client IDs remain reserved until `wl_display.delete_id`.
/// ```xml
/// <event name="discarded" type="destructor">
///       <description summary="the content update was not displayed">
///         The content update was never displayed to the user.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct WpPresentationFeedbackDiscardedEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
}
impl WpPresentationFeedbackDiscardedEvent {
    /// Opcode in the event namespace of `wp_presentation_feedback`.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpPresentationFeedbackDiscardedEvent {
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

impl InterfaceReader for WpPresentationFeedbackDiscardedEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(interface, ObjectInteface::WpPresentationFeedback { .. })
            || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Self::reader_state(state)?.release_server_id(object_id);
        Ok(Self { object_id })
    }
}

impl CommandWriter for WpPresentationFeedbackDiscardedEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[])
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
        Self::writer_state(state)?.release_server_id(self.object_id);
        Ok(())
    }
}
