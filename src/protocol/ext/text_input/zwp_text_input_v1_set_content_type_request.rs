use super::ObjectInteface as TextInputObjectInteface;
use super::ZwpTextInputV1ContentHint;
use super::ZwpTextInputV1ContentPurpose;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_text_input_v1.set_content_type`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_content_type">
///       <description summary="set content purpose and hint">
///     Sets the content purpose and content hint. While the purpose is the
///     basic purpose of an input field, the hint flags allow to modify some
///     of the behavior.
///
///     When no content type is explicitly set, a normal content purpose with
///     default hints (auto completion, auto correction, auto capitalization)
///     should be assumed.
///       </description>
///       <arg name="hint" type="uint" enum="content_hint" />
///       <arg name="purpose" type="uint" enum="content_purpose" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpTextInputV1SetContentTypeRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `hint`.
    pub hint: ZwpTextInputV1ContentHint,
    /// Wire argument `purpose`.
    pub purpose: ZwpTextInputV1ContentPurpose,
}
impl ZwpTextInputV1SetContentTypeRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 6;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTextInputV1SetContentTypeRequest {
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
impl InterfaceReader for ZwpTextInputV1SetContentTypeRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::TextInput(TextInputObjectInteface::ZwpTextInputV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let hint = ZwpTextInputV1ContentHint(reader.next_u32().await?);
        let purpose = ZwpTextInputV1ContentPurpose(reader.next_u32().await?);
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
impl CommandWriter for ZwpTextInputV1SetContentTypeRequest {
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
            "zwp_text_input_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::TextInput(TextInputObjectInteface::ZwpTextInputV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.hint.0).await?;
        writer.next_u32(self.purpose.0).await?;
        Ok(())
    }
}
