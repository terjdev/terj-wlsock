use super::ObjectInteface as XxInputMethodObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_input_method_v1.delete_surrounding_text`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="delete_surrounding_text">
///       <description summary="delete text">
///         Remove the surrounding text.
///
///         before_length and after_length are the number of bytes before and after
///         the current cursor index (excluding the preedit text) to delete.
///
///         If text is selected, it must be deleted.
///
///         If indices exceed the available text boundaries, they should be adjusted to fit in boundaries and deletion reattempted.
///         If indices do not lie on byte boundaries, then the text input client should delete at least that many bytes. In this case, the client decides the end point, but a character boundary same as when deleting using the keyboard is recommended.
///
///         If any preedit text is present, it is replaced with the cursor for the
///         purpose of this event. In effect before_length is counted from the
///         beginning of preedit text, and after_length from its end (see commit
///         event sequence).
///
///         Values set with this request are double-buffered. They must be applied
///         and reset to initial on the next xx_input_method_v1.commit request.
///
///         The initial values of both before_length and after_length are 0.
///       </description>
///       <arg name="before_length" type="uint" />
///       <arg name="after_length" type="uint" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxInputMethodV1DeleteSurroundingTextRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `before_length`.
    pub before_length: u32,
    /// Wire argument `after_length`.
    pub after_length: u32,
}
impl XxInputMethodV1DeleteSurroundingTextRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxInputMethodV1DeleteSurroundingTextRequest {
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
impl InterfaceReader for XxInputMethodV1DeleteSurroundingTextRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxInputMethod(XxInputMethodObjectInteface::XxInputMethodV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let before_length = reader.next_u32().await?;
        let after_length = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            before_length,
            after_length,
        })
    }
}
impl CommandWriter for XxInputMethodV1DeleteSurroundingTextRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "xx_input_method_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::XxInputMethod(XxInputMethodObjectInteface::XxInputMethodV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.before_length).await?;
        writer.next_u32(self.after_length).await?;
        Ok(())
    }
}
