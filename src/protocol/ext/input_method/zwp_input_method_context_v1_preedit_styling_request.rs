use super::ObjectInteface as InputMethodObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_input_method_context_v1.preedit_styling`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="preedit_styling">
///       <description summary="pre-edit styling">
///     Set the styling information on composing text. The style is applied for
///     length in bytes from index relative to the beginning of
///     the composing text (as byte offset). Multiple styles can
///     be applied to a composing text.
///
///     This request should be sent before sending a preedit_string request.
///       </description>
///       <arg name="index" type="uint" />
///       <arg name="length" type="uint" />
///       <arg name="style" type="uint" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpInputMethodContextV1PreeditStylingRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `index`.
    pub index: u32,
    /// Wire argument `length`.
    pub length: u32,
    /// Wire argument `style`.
    pub style: u32,
}
impl ZwpInputMethodContextV1PreeditStylingRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpInputMethodContextV1PreeditStylingRequest {
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
impl InterfaceReader for ZwpInputMethodContextV1PreeditStylingRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::InputMethod(InputMethodObjectInteface::ZwpInputMethodContextV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let index = reader.next_u32().await?;
        let length = reader.next_u32().await?;
        let style = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            index,
            length,
            style,
        })
    }
}
impl CommandWriter for ZwpInputMethodContextV1PreeditStylingRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "zwp_input_method_context_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::InputMethod(InputMethodObjectInteface::ZwpInputMethodContextV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.index).await?;
        writer.next_u32(self.length).await?;
        writer.next_u32(self.style).await?;
        Ok(())
    }
}
