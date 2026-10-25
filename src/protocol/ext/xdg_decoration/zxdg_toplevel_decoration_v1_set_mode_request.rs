use super::ObjectInteface as XdgDecorationObjectInteface;
use super::ZxdgToplevelDecorationV1Mode;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zxdg_toplevel_decoration_v1.set_mode`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_mode">
///       <description summary="set the decoration mode">
///         Set the toplevel surface decoration mode. This informs the compositor
///         that the client prefers the provided decoration mode.
///
///         After requesting a decoration mode, the compositor will respond by
///         emitting an xdg_surface.configure event. The client should then update
///         its content, drawing it without decorations if the received mode is
///         server-side decorations. The client must also acknowledge the configure
///         when committing the new content (see xdg_surface.ack_configure).
///
///         The compositor can decide not to use the client's mode and enforce a
///         different mode instead.
///
///         Clients whose decoration mode depend on the xdg_toplevel state may send
///         a set_mode request in response to an xdg_surface.configure event and wait
///         for the next xdg_surface.configure event to prevent unwanted state.
///         Such clients are responsible for preventing configure loops and must
///         make sure not to send multiple successive set_mode requests with the
///         same decoration mode.
///
///         If an invalid mode is supplied by the client, the invalid_mode protocol
///         error is raised by the compositor.
///       </description>
///       <arg name="mode" type="uint" enum="mode" summary="the decoration mode" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZxdgToplevelDecorationV1SetModeRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the decoration mode.
    pub mode: ZxdgToplevelDecorationV1Mode,
}
impl ZxdgToplevelDecorationV1SetModeRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZxdgToplevelDecorationV1SetModeRequest {
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
impl InterfaceReader for ZxdgToplevelDecorationV1SetModeRequest {
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
impl CommandWriter for ZxdgToplevelDecorationV1SetModeRequest {
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
        let interface = Self::writer_request_interface(
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
