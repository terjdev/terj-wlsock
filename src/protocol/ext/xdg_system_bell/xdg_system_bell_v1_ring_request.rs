use crate::protocol::ext::xdg_system_bell::ObjectInteface as XdgSystemBellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_system_bell_v1.ring`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="ring">
///       <description summary="ring the system bell">
///     This requests rings the system bell on behalf of a client. How ringing
///     the bell is implemented is up to the compositor. It may be an audible
///     sound, a visual feedback of some kind, or any other thing including
///     nothing.
///
///         The passed surface should correspond to a toplevel like surface role,
///         or be null, meaning the client doesn't have a particular toplevel it
///         wants to associate the bell ringing with. See the xdg-shell protocol
///         extension for a toplevel like surface role.
///       </description>
///       <arg name="surface" type="object" interface="wl_surface" allow-null="true" summary="associated surface" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgSystemBellV1RingRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// associated surface.
    pub surface: Option<u32>,
}
impl XdgSystemBellV1RingRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgSystemBellV1RingRequest {
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
impl InterfaceReader for XdgSystemBellV1RingRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgSystemBell(XdgSystemBellObjectInteface::XdgSystemBellV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let surface = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, surface })
    }
}
impl CommandWriter for XdgSystemBellV1RingRequest {
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
        Self::writer_request_interface(state, self.object_id, "xdg_system_bell_v1", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.surface.unwrap_or(0)).await?;
        Ok(())
    }
}
