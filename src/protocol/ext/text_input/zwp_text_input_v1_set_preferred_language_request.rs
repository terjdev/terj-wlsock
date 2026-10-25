use super::ObjectInteface as TextInputObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_text_input_v1.set_preferred_language`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_preferred_language">
///       <description summary="sets preferred language">
///     Sets a specific language. This allows for example a virtual keyboard to
///     show a language specific layout. The "language" argument is an RFC-3066
///     format language tag.
///
///     It could be used for example in a word processor to indicate the
///     language of the currently edited document or in an instant message
///     application which tracks languages of contacts.
///       </description>
///       <arg name="language" type="string" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpTextInputV1SetPreferredLanguageRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `language`.
    pub language: String,
}
impl ZwpTextInputV1SetPreferredLanguageRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 8;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTextInputV1SetPreferredLanguageRequest {
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
impl InterfaceReader for ZwpTextInputV1SetPreferredLanguageRequest {
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
        let language = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            language,
        })
    }
}
impl CommandWriter for ZwpTextInputV1SetPreferredLanguageRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_string_size(Some(&self.language))])
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
        if !FdWriter::is_valid_string(&self.language) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.language).await?;
        Ok(())
    }
}
