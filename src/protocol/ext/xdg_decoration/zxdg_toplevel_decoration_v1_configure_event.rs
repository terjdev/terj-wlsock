use super::ObjectInteface as XdgDecorationObjectInteface;
use super::ZxdgToplevelDecorationV1Mode;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zxdg_toplevel_decoration_v1.configure`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="configure">
///       <description summary="notify a decoration mode change">
///         The configure event configures the effective decoration mode. The
///         configured state should not be applied immediately. Clients must send an
///         ack_configure in response to this event. See xdg_surface.configure and
///         xdg_surface.ack_configure for details.
///
///         A configure event can be sent at any time. The specified mode must be
///         obeyed by the client.
///       </description>
///       <arg name="mode" type="uint" enum="mode" summary="the decoration mode" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZxdgToplevelDecorationV1ConfigureEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the decoration mode.
    pub mode: ZxdgToplevelDecorationV1Mode,
}
impl ZxdgToplevelDecorationV1ConfigureEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZxdgToplevelDecorationV1ConfigureEvent {
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
impl InterfaceReader for ZxdgToplevelDecorationV1ConfigureEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgDecoration(
                XdgDecorationObjectInteface::ZxdgToplevelDecorationV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let mode = ZxdgToplevelDecorationV1Mode(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, mode })
    }
}
impl CommandWriter for ZxdgToplevelDecorationV1ConfigureEvent {
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
            "zxdg_toplevel_decoration_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::XdgDecoration(
                XdgDecorationObjectInteface::ZxdgToplevelDecorationV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.mode.0).await?;
        Ok(())
    }
}
