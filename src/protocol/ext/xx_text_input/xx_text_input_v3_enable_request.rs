use super::ObjectInteface as XxTextInputObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_text_input_v3.enable`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="enable">
///       <description summary="Request text input to be enabled">
///         Requests text input on the surface previously obtained from the enter
///         event.
///
///         This request must be issued every time the active text input changes
///         to a new one, including within the current surface. Use
///         xx_text_input_v3.disable when there is no longer any input focus on
///         the current surface.
///
///         Clients must not enable more than one text input on the single seat
///         and should disable the current text input before enabling the new one.
///         At most one instance of text input may be in enabled state per instance,
///         Requests to enable the another text input when some text input is active
///         must be ignored by compositor.
///
///         This request resets all state associated with previous enable, disable,
///         set_surrounding_text, set_text_change_cause, set_content_type, and
///         set_cursor_rectangle requests, as well as the state associated with
///         preedit_string, commit_string, delete_surrounding_text, and action
///         events.
///
///         The set_surrounding_text, set_content_type and set_cursor_rectangle
///         requests must follow if the text input supports the necessary
///         functionality.
///
///         State set with this request is double-buffered. It will get applied on
///         the next xx_text_input_v3.commit request, and stay valid until the
///         next committed enable or disable request.
///
///         The changes must be applied by the compositor after issuing a
///         xx_text_input_v3.commit request.
///       </description>
///     </request>
/// ```
#[derive(Debug)]
pub struct XxTextInputV3EnableRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl XxTextInputV3EnableRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxTextInputV3EnableRequest {
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
impl InterfaceReader for XxTextInputV3EnableRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxTextInput(XxTextInputObjectInteface::XxTextInputV3 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id })
    }
}
impl CommandWriter for XxTextInputV3EnableRequest {
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
        let interface =
            Self::writer_request_interface(state, self.object_id, "xx_text_input_v3", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XxTextInput(XxTextInputObjectInteface::XxTextInputV3 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        Ok(())
    }
}
