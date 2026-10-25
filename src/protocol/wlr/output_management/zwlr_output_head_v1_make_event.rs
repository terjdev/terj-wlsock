use super::ObjectInteface as WlrOutputManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_output_head_v1.make`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="make" since="2">
///       <description summary="head manufacturer">
///         This event describes the manufacturer of the head.
///
///         Together with the model and serial_number events the purpose is to
///         allow clients to recognize heads from previous sessions and for example
///         load head-specific configurations back.
///
///         It is not guaranteed this event will be ever sent. A reason for that
///         can be that the compositor does not have information about the make of
///         the head or the definition of a make is not sensible in the current
///         setup, for example in a virtual session. Clients can still try to
///         identify the head by available information from other events but should
///         be aware that there is an increased risk of false positives.
///
///         If sent, the make event is sent after a wlr_output_head object is
///         created and only sent once per object. The make does not change over
///         the lifetime of the wlr_output_head object.
///
///         It is not recommended to display the make string in UI to users. For
///         that the string provided by the description event should be preferred.
///       </description>
///       <arg name="make" type="string" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrOutputHeadV1MakeEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `make`.
    pub make: String,
}
impl ZwlrOutputHeadV1MakeEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 10;
    /// Minimum interface version.
    pub const SINCE: u32 = 2;
}
impl CommandReader<Self> for ZwlrOutputHeadV1MakeEvent {
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
impl InterfaceReader for ZwlrOutputHeadV1MakeEvent {
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
        let make = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, make })
    }
}
impl CommandWriter for ZwlrOutputHeadV1MakeEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_string_size(Some(&self.make))])
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
        if !FdWriter::is_valid_string(&self.make) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.make).await?;
        Ok(())
    }
}
