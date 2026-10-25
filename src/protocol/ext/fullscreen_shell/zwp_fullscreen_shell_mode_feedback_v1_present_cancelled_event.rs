use super::ObjectInteface as FullscreenShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_fullscreen_shell_mode_feedback_v1.present_cancelled`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="present_cancelled" type="destructor">
///       <description summary="mode switch cancelled">
///     This event indicates that the attempted mode switch operation was
///     cancelled.  Most likely this is because the client requested a
///     second mode switch before the first one completed.
///
///     Upon receiving this event, the client should destroy the
///     wl_fullscreen_shell_mode_feedback object.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpFullscreenShellModeFeedbackV1PresentCancelledEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl ZwpFullscreenShellModeFeedbackV1PresentCancelledEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpFullscreenShellModeFeedbackV1PresentCancelledEvent {
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
impl InterfaceReader for ZwpFullscreenShellModeFeedbackV1PresentCancelledEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::FullscreenShell(
                FullscreenShellObjectInteface::ZwpFullscreenShellModeFeedbackV1 { .. }
            )
        ) || opcode != Self::OPCODE
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
impl CommandWriter for ZwpFullscreenShellModeFeedbackV1PresentCancelledEvent {
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
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "zwp_fullscreen_shell_mode_feedback_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::FullscreenShell(
                FullscreenShellObjectInteface::ZwpFullscreenShellModeFeedbackV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        Self::writer_state(state)?.release_server_id(self.object_id);
        Ok(())
    }
}
