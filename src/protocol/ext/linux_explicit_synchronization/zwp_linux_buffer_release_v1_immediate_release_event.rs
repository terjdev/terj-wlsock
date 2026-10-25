use super::ObjectInteface as LinuxExplicitSynchronizationObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_linux_buffer_release_v1.immediate_release`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="immediate_release" type="destructor">
///       <description summary="release buffer immediately">
///         Sent when the compositor has finalised its usage of the associated
///         buffer for the relevant commit, and either performed no operations
///         using it, or has a guarantee that all its operations on that buffer for
///         that commit have finished.
///
///         Once this event is received, and assuming the associated buffer is not
///         pending release from other wl_surface.commit requests, no additional
///         explicit or implicit synchronization is required to safely reuse or
///         destroy the buffer.
///
///         This event destroys the zwp_linux_buffer_release_v1 object.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpLinuxBufferReleaseV1ImmediateReleaseEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl ZwpLinuxBufferReleaseV1ImmediateReleaseEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpLinuxBufferReleaseV1ImmediateReleaseEvent {
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
impl InterfaceReader for ZwpLinuxBufferReleaseV1ImmediateReleaseEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::LinuxExplicitSynchronization(
                LinuxExplicitSynchronizationObjectInteface::ZwpLinuxBufferReleaseV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Self::reader_state(state)?.release_server_id(object_id);
        Ok(Self { object_id })
    }
}
impl CommandWriter for ZwpLinuxBufferReleaseV1ImmediateReleaseEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "zwp_linux_buffer_release_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::LinuxExplicitSynchronization(
                LinuxExplicitSynchronizationObjectInteface::ZwpLinuxBufferReleaseV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        Self::writer_state(state)?.release_server_id(self.object_id);
        Ok(())
    }
}
