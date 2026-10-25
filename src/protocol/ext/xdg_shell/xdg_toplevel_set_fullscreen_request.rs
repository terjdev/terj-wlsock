use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_toplevel.set_fullscreen`.
///
/// Opcode 11; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_fullscreen">
///       <description summary="set the window as fullscreen on an output">
///     Make the surface fullscreen.
///
///     After requesting that the surface should be fullscreened, the
///     compositor will respond by emitting a configure event. Whether the
///     client is actually put into a fullscreen state is subject to compositor
///     policies. The client must also acknowledge the configure when
///     committing the new content (see ack_configure).
///
///     The output passed by the request indicates the client's preference as
///     to which display it should be set fullscreen on. If this value is NULL,
///     it's up to the compositor to choose which display will be used to map
///     this surface.
///
///     If the surface doesn't cover the whole output, the compositor will
///     position the surface in the center of the output and compensate with
///     border fill covering the rest of the output. The content of the
///     border fill is undefined, but should be assumed to be in some way that
///     attempts to blend into the surrounding area (e.g. solid black).
///
///     If the fullscreened surface is not opaque, the compositor must make
///     sure that other screen content not part of the same surface tree (made
///     up of subsurfaces, popups or similarly coupled surfaces) are not
///     visible below the fullscreened surface.
///       </description>
///       <arg name="output" type="object" interface="wl_output" allow-null="true" summary="preferred output to place surface on" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgToplevelSetFullscreenRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// preferred output to place surface on.
    pub output: Option<u32>,
}
impl XdgToplevelSetFullscreenRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 11;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgToplevelSetFullscreenRequest {
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
impl InterfaceReader for XdgToplevelSetFullscreenRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgShell(XdgShellObjectInteface::XdgToplevel { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let output = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, output })
    }
}
impl CommandWriter for XdgToplevelSetFullscreenRequest {
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
        Self::writer_request_interface(state, self.object_id, "xdg_toplevel", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.output.unwrap_or(0)).await?;
        Ok(())
    }
}
