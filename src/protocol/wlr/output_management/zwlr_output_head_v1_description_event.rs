use super::ObjectInteface as WlrOutputManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_output_head_v1.description`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="description">
///       <description summary="head description">
///         This event describes a human-readable description of the head.
///
///         The description is a UTF-8 string with no convention defined for its
///         contents. Examples might include 'Foocorp 11" Display' or 'Virtual X11
///         output via :1'. However, do not assume that the name is a reflection of
///         the make, model, serial of the underlying DRM connector or the display
///         name of the underlying X11 connection, etc.
///
///         If this head matches a wl_output, the wl_output.description event must
///         report the same name.
///
///         The description event is sent after a wlr_output_head object is created.
///         This event is only sent once per object, and the description does not
///         change over the lifetime of the wlr_output_head object.
///       </description>
///       <arg name="description" type="string" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrOutputHeadV1DescriptionEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `description`.
    pub description: String,
}
impl ZwlrOutputHeadV1DescriptionEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrOutputHeadV1DescriptionEvent {
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
impl InterfaceReader for ZwlrOutputHeadV1DescriptionEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputHeadV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let description = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            description,
        })
    }
}
impl CommandWriter for ZwlrOutputHeadV1DescriptionEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_string_size(Some(&self.description))])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "zwlr_output_head_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputHeadV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.description) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.description).await?;
        Ok(())
    }
}
