use crate::protocol::ext::dmabuf::ObjectInteface as DmabufObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_linux_dmabuf_feedback_v1.format_table`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="format_table">
///       <description summary="format and modifier table">
///         This event provides a file descriptor which can be memory-mapped to
///         access the format and modifier table.
///
///         The table contains a tightly packed array of consecutive format +
///         modifier pairs. Each pair is 16 bytes wide. It contains a format as a
///         32-bit unsigned integer, followed by 4 bytes of unused padding, and a
///         modifier as a 64-bit unsigned integer. The native endianness is used.
///
///         The client must map the file descriptor in read-only private mode.
///
///         Compositors are not allowed to mutate the table file contents once this
///         event has been sent. Instead, compositors must create a new, separate
///         table file and re-send feedback parameters. Compositors are allowed to
///         store duplicate format + modifier pairs in the table.
///       </description>
///       <arg name="fd" type="fd" summary="table file descriptor" />
///       <arg name="size" type="uint" summary="table size, in bytes" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpLinuxDmabufFeedbackV1FormatTableEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// table file descriptor.
    pub fd: std::os::fd::OwnedFd,
    /// table size, in bytes.
    pub size: u32,
}
impl ZwpLinuxDmabufFeedbackV1FormatTableEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpLinuxDmabufFeedbackV1FormatTableEvent {
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
impl InterfaceReader for ZwpLinuxDmabufFeedbackV1FormatTableEvent {
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
        let size = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        let fd = reader.next_fd().ok_or(FdReaderError::InvalidProtocol)?;
        Ok(Self {
            object_id,
            fd,
            size,
        })
    }
}
impl CommandWriter for ZwpLinuxDmabufFeedbackV1FormatTableEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(0), Some(4)])
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
        writer.next_fd(self.fd).await?;
        writer.next_u32(self.size).await?;
        Ok(())
    }
}
