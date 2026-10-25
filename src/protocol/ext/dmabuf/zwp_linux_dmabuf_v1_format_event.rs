use crate::protocol::ext::dmabuf::ObjectInteface as DmabufObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_linux_dmabuf_v1.format`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// Sending or receiving this event is rejected at version 4 and above.
/// ```xml
/// <event name="format" deprecated-since="4">
///       <description summary="supported buffer format">
///         This event advertises one buffer format that the server supports.
///         All the supported formats are advertised once when the client
///         binds to this interface. A roundtrip after binding guarantees
///         that the client has received all supported formats.
///
///         For the definition of the format codes, see the
///         zwp_linux_buffer_params_v1::create request.
///
///         Starting version 4, the format event is deprecated and must not be
///         sent by compositors. Instead, use get_default_feedback or
///         get_surface_feedback.
///       </description>
///       <arg name="format" type="uint" summary="DRM_FORMAT code" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpLinuxDmabufV1FormatEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// DRM_FORMAT code.
    pub format: u32,
}
impl ZwpLinuxDmabufV1FormatEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
    /// First interface version prohibiting this event.
    pub const DEPRECATED_SINCE: u32 = 4;
}
impl CommandReader<Self> for ZwpLinuxDmabufV1FormatEvent {
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
impl InterfaceReader for ZwpLinuxDmabufV1FormatEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Dmabuf(DmabufObjectInteface::ZwpLinuxDmabufV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
            || interface.version() >= Self::DEPRECATED_SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let format = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, format })
    }
}
impl CommandWriter for ZwpLinuxDmabufV1FormatEvent {
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
            "zwp_linux_dmabuf_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota()
            || interface.version() >= Self::DEPRECATED_SINCE
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.format).await?;
        Ok(())
    }
}
