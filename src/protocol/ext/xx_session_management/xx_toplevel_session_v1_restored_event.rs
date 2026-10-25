use super::ObjectInteface as XxSessionManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xx_toplevel_session_v1.restored`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="restored">
///       <description summary="a toplevel's session has been restored">
///         The "restored" event is emitted prior to the first
///         xdg_toplevel.configure for the toplevel. It will only be emitted after
///         xx_session_v1.restore_toplevel, and the initial empty surface state has
///         been applied, and it indicates that the surface's session is being
///         restored with this configure event.
///       </description>
///       <arg name="surface" type="object" interface="xdg_toplevel" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XxToplevelSessionV1RestoredEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `surface`.
    pub surface: u32,
}
impl XxToplevelSessionV1RestoredEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxToplevelSessionV1RestoredEvent {
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
impl InterfaceReader for XxToplevelSessionV1RestoredEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxSessionManagement(
                XxSessionManagementObjectInteface::XxToplevelSessionV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let surface = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, surface })
    }
}
impl CommandWriter for XxToplevelSessionV1RestoredEvent {
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
            "xx_toplevel_session_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::XxSessionManagement(
                XxSessionManagementObjectInteface::XxToplevelSessionV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.surface).await?;
        Ok(())
    }
}
