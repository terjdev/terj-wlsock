use super::ObjectInteface as TextInputObjectInteface;
use super::ZwpTextInputV3PreeditHint;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_text_input_v3.preedit_hint`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="preedit_hint" since="2">
///       <description summary="pre-edit">
///         Notify of contextual hints for the pre-edit string. This
///         event is always sent together with a zwp_text_input_v3.preedit_string
///         event.
///
///         The parameters start and end are counted in bytes relative to the
///         beginning of the text buffer submitted through
///         zwp_text_input_v3.preedit_string, and represent the substring in the
///         pre-edit text affected by the hint.
///
///         Multiple events may be submitted if the preedit string has different
///         sections. The extent of hints may overlap. The parts of the preedit
///         string that are not covered by any zwp_text_input_v3.preedit_hint event,
///         the text will be considered unhinted. This is also the case if no
///         preedit_hint event is sent.
///
///         Clients should provide recognizable visuals to these hints. if they are
///         unable to comply with this requisition, it may be preferable for them
///         keep the preedit_shown content hint disabled.
///
///         Values set with this event are double-buffered. They must be applied
///         and reset on the next zwp_text_input_v3.done event.
///       </description>
///       <arg name="start" type="uint" summary="starting point of the affected substring" />
///       <arg name="end" type="uint" summary="end point of the affected substring" />
///       <arg name="hint" type="uint" enum="preedit_hint" summary="hint to apply" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTextInputV3PreeditHintEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// starting point of the affected substring.
    pub start: u32,
    /// end point of the affected substring.
    pub end: u32,
    /// hint to apply.
    pub hint: ZwpTextInputV3PreeditHint,
}
impl ZwpTextInputV3PreeditHintEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 8;
    /// Minimum interface version.
    pub const SINCE: u32 = 2;
}
impl CommandReader<Self> for ZwpTextInputV3PreeditHintEvent {
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
impl InterfaceReader for ZwpTextInputV3PreeditHintEvent {
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
        let start = reader.next_u32().await?;
        let end = reader.next_u32().await?;
        let hint = ZwpTextInputV3PreeditHint(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            start,
            end,
            hint,
        })
    }
}
impl CommandWriter for ZwpTextInputV3PreeditHintEvent {
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
        let interface =
            Self::writer_event_interface(state, self.object_id, "zwp_text_input_v3", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::TextInput(TextInputObjectInteface::ZwpTextInputV3 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.start).await?;
        writer.next_u32(self.end).await?;
        writer.next_u32(self.hint.0).await?;
        Ok(())
    }
}
