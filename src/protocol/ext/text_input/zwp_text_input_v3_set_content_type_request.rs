use super::ObjectInteface as TextInputObjectInteface;
use super::ZwpTextInputV3ContentHint;
use super::ZwpTextInputV3ContentPurpose;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_text_input_v3.set_content_type`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_content_type">
///       <description summary="set content purpose and hint">
///         Sets the content purpose and content hint. While the purpose is the
///         basic purpose of an input field, the hint flags allow to modify some of
///         the behavior.
///
///         Values set with this request are double-buffered. They will get applied
///         on the next zwp_text_input_v3.commit request.
///         Subsequent attempts to update them may have no effect. The values
///         remain valid until the next committed enable or disable request.
///
///         The initial value for hint is none, and the initial value for purpose
///         is normal.
///       </description>
///       <arg name="hint" type="uint" enum="content_hint" />
///       <arg name="purpose" type="uint" enum="content_purpose" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpTextInputV3SetContentTypeRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `hint`.
    pub hint: ZwpTextInputV3ContentHint,
    /// Wire argument `purpose`.
    pub purpose: ZwpTextInputV3ContentPurpose,
}
impl ZwpTextInputV3SetContentTypeRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 5;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTextInputV3SetContentTypeRequest {
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
impl InterfaceReader for ZwpTextInputV3SetContentTypeRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::TextInput(TextInputObjectInteface::ZwpTextInputV3 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let hint = ZwpTextInputV3ContentHint(reader.next_u32().await?);
        let purpose = ZwpTextInputV3ContentPurpose(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            hint,
            purpose,
        })
    }
}
impl CommandWriter for ZwpTextInputV3SetContentTypeRequest {
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
            "zwp_text_input_v3",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::TextInput(TextInputObjectInteface::ZwpTextInputV3 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.hint.0).await?;
        writer.next_u32(self.purpose.0).await?;
        Ok(())
    }
}
