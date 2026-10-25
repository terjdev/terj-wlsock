use crate::protocol::ext::color_management::ObjectInteface as ColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wp_image_description_v1.ready`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// Sending or receiving this event is rejected at version 2 and above.
/// ```xml
/// <event name="ready" deprecated-since="2">
///       <description summary="the object is ready to be used (32-bit)">
///         Starting from interface version 2, the 'ready2' event is sent instead
///         of this event.
///
///         For the definition of this event, see the 'ready2' event. The
///         difference to this event is as follows.
///
///         The id number is valid only as long as the protocol object is alive. If
///         all protocol objects referring to the same image description record are
///         destroyed, the id number may be recycled for a different image
///         description record.
///       </description>
///
///       <arg name="identity" type="uint" summary="the 32-bit image description id number" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WpImageDescriptionV1ReadyEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the 32-bit image description id number.
    pub identity: u32,
}
impl WpImageDescriptionV1ReadyEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
    /// First interface version prohibiting this event.
    pub const DEPRECATED_SINCE: u32 = 2;
}
impl CommandReader<Self> for WpImageDescriptionV1ReadyEvent {
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
impl InterfaceReader for WpImageDescriptionV1ReadyEvent {
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
            || interface.version() >= Self::DEPRECATED_SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let identity = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            identity,
        })
    }
}
impl CommandWriter for WpImageDescriptionV1ReadyEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "wp_image_description_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota()
            || interface.version() >= Self::DEPRECATED_SINCE
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.identity).await?;
        Ok(())
    }
}
