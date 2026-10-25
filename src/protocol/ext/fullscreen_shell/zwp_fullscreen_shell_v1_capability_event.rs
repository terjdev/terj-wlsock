use super::ObjectInteface as FullscreenShellObjectInteface;
use super::ZwpFullscreenShellV1Capability;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_fullscreen_shell_v1.capability`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="capability">
///       <description summary="advertises a capability of the compositor">
///     Advertises a single capability of the compositor.
///
///     When the wl_fullscreen_shell interface is bound, this event is emitted
///     once for each capability advertised.  Valid capabilities are given by
///     the wl_fullscreen_shell.capability enum.  If clients want to take
///     advantage of any of these capabilities, they should use a
///     wl_display.sync request immediately after binding to ensure that they
///     receive all the capability events.
///       </description>
///       <arg name="capability" type="uint" enum="capability" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpFullscreenShellV1CapabilityEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `capability`.
    pub capability: ZwpFullscreenShellV1Capability,
}
impl ZwpFullscreenShellV1CapabilityEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpFullscreenShellV1CapabilityEvent {
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
impl InterfaceReader for ZwpFullscreenShellV1CapabilityEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::FullscreenShell(
                FullscreenShellObjectInteface::ZwpFullscreenShellV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let capability = ZwpFullscreenShellV1Capability(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            capability,
        })
    }
}
impl CommandWriter for ZwpFullscreenShellV1CapabilityEvent {
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
            "zwp_fullscreen_shell_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::FullscreenShell(
                FullscreenShellObjectInteface::ZwpFullscreenShellV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.capability.0).await?;
        Ok(())
    }
}
