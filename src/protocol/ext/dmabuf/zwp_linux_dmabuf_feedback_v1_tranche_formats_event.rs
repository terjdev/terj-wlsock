use crate::protocol::ext::dmabuf::ObjectInteface as DmabufObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_linux_dmabuf_feedback_v1.tranche_formats`.
///
/// Opcode 5; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="tranche_formats">
///       <description summary="supported buffer format modifiers">
///         This event advertises the format + modifier combinations that the
///         compositor supports.
///
///         It carries an array of indices, each referring to a format + modifier
///         pair in the last received format table (see the format_table event).
///         Each index is a 16-bit unsigned integer in native endianness.
///
///         For legacy support, DRM_FORMAT_MOD_INVALID is an allowed modifier.
///         It indicates that the server can support the format with an implicit
///         modifier. When a buffer has DRM_FORMAT_MOD_INVALID as its modifier, it
///         is as if no explicit modifier is specified. The effective modifier
///         will be derived from the dmabuf.
///
///         A compositor that sends valid modifiers and DRM_FORMAT_MOD_INVALID for
///         a given format supports both explicit modifiers and implicit modifiers.
///
///         Compositors must not send duplicate format + modifier pairs within the
///         same tranche or across two different tranches with the same target
///         device and flags.
///
///         This event is tied to a preference tranche, see the tranche_done event.
///
///         For the definition of the format and modifier codes, see the
///         zwp_linux_buffer_params_v1.create request.
///       </description>
///       <arg name="indices" type="array" summary="array of 16-bit indexes" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpLinuxDmabufFeedbackV1TrancheFormatsEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// array of 16-bit indexes.
    pub indices: Vec<u8>,
}
impl ZwpLinuxDmabufFeedbackV1TrancheFormatsEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 5;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpLinuxDmabufFeedbackV1TrancheFormatsEvent {
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
impl InterfaceReader for ZwpLinuxDmabufFeedbackV1TrancheFormatsEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Dmabuf(DmabufObjectInteface::ZwpLinuxDmabufFeedbackV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let indices = reader.next_array().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, indices })
    }
}
impl CommandWriter for ZwpLinuxDmabufFeedbackV1TrancheFormatsEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_array_size(self.indices.len())])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_event_interface(
            state,
            self.object_id,
            "zwp_linux_dmabuf_feedback_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_array(&self.indices).await?;
        Ok(())
    }
}
