use super::ObjectInteface as XxInputMethodObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xx_input_popup_surface_v2.repositioned`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="repositioned">
///       <description summary="signal the completion of a reposition request">
///         The compositor sends the .repositioned event in response to the .reposition request to notify about its completion.
///
///         The new geometry of the popup can be communicated using additional events within a configure sequence including:
///         - input_popup_surface.start_configure, and
///         - the .anchor_position event to update the relative position to the anchor.
///
///         When responding to a .reposition request, the token argument is the token passed in the that request.
///
///         This event is sent as part of a configure sequence.
///         State set by this event is conceptually double-buffered.
///         Every argument overwrites its previous value. The state change should get applied atomically with the next input_method.done event.
///
///         The client should optionally update the content of the popup, but must
///         acknowledge the new popup configuration for the new position to take
///         effect. See input_popup_surface.ack_configure for details.
///       </description>
///       <arg name="token" type="uint" summary="reposition request token" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XxInputPopupSurfaceV2RepositionedEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// reposition request token.
    pub token: u32,
}
impl XxInputPopupSurfaceV2RepositionedEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxInputPopupSurfaceV2RepositionedEvent {
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
impl InterfaceReader for XxInputPopupSurfaceV2RepositionedEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxInputMethod(
                XxInputMethodObjectInteface::XxInputPopupSurfaceV2 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let token = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, token })
    }
}
impl CommandWriter for XxInputPopupSurfaceV2RepositionedEvent {
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
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "xx_input_popup_surface_v2",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::XxInputMethod(
                XxInputMethodObjectInteface::XxInputPopupSurfaceV2 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.token).await?;
        Ok(())
    }
}
