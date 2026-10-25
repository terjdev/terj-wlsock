use super::ObjectInteface as WlrOutputManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_output_head_v1.serial_number`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="serial_number" since="2">
///       <description summary="head serial number">
///         This event describes the serial number of the head.
///
///         Together with the make and model events the purpose is to allow clients
///         to recognize heads from previous sessions and for example load head-
///         specific configurations back.
///
///         It is not guaranteed this event will be ever sent. A reason for that
///         can be that the compositor does not have information about the serial
///         number of the head or the definition of a serial number is not sensible
///         in the current setup. Clients can still try to identify the head by
///         available information from other events but should be aware that there
///         is an increased risk of false positives.
///
///         If sent, the serial number event is sent after a wlr_output_head object
///         is created and only sent once per object. The serial number does not
///         change over the lifetime of the wlr_output_head object.
///
///         It is not recommended to display the serial_number string in UI to
///         users. For that the string provided by the description event should be
///         preferred.
///       </description>
///       <arg name="serial_number" type="string" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrOutputHeadV1SerialNumberEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `serial_number`.
    pub serial_number: String,
}
impl ZwlrOutputHeadV1SerialNumberEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 12;
    /// Minimum interface version.
    pub const SINCE: u32 = 2;
}
impl CommandReader<Self> for ZwlrOutputHeadV1SerialNumberEvent {
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
impl InterfaceReader for ZwlrOutputHeadV1SerialNumberEvent {
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
        let serial_number = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            serial_number,
        })
    }
}
impl CommandWriter for ZwlrOutputHeadV1SerialNumberEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_string_size(Some(&self.serial_number))])
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
        if !FdWriter::is_valid_string(&self.serial_number) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.serial_number).await?;
        Ok(())
    }
}
