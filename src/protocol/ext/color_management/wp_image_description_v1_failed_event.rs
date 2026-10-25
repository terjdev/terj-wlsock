use super::WpImageDescriptionV1Cause;
use crate::protocol::ext::color_management::ObjectInteface as ColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wp_image_description_v1.failed`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="failed">
///       <description summary="graceful error on creating the image description">
///         If creating a wp_image_description_v1 object fails for a reason that is
///         not defined as a protocol error, this event is sent.
///
///         The requests that create image description objects define whether and
///         when this can occur. Only such creation requests can trigger this event.
///         This event cannot be triggered after the image description was
///         successfully formed.
///
///         Once this event has been sent, the wp_image_description_v1 object will
///         never become ready and it can only be destroyed.
///       </description>
///
///       <arg name="cause" type="uint" enum="cause" summary="generic reason" />
///       <arg name="msg" type="string" summary="ad hoc human-readable explanation" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WpImageDescriptionV1FailedEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// generic reason.
    pub cause: WpImageDescriptionV1Cause,
    /// ad hoc human-readable explanation.
    pub msg: String,
}
impl WpImageDescriptionV1FailedEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpImageDescriptionV1FailedEvent {
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
impl InterfaceReader for WpImageDescriptionV1FailedEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::ColorManagement(
                ColorManagementObjectInteface::WpImageDescriptionV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let cause = WpImageDescriptionV1Cause(reader.next_u32().await?);
        let msg = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            cause,
            msg,
        })
    }
}
impl CommandWriter for WpImageDescriptionV1FailedEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), FdWriter::encoded_string_size(Some(&self.msg))])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_event_interface(
            state,
            self.object_id,
            "wp_image_description_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.msg) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.cause.0).await?;
        writer.next_string(&self.msg).await?;
        Ok(())
    }
}
