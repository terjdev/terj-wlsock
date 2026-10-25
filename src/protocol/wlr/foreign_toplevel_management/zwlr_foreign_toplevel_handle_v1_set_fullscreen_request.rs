use super::ObjectInteface as WlrForeignToplevelManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_foreign_toplevel_handle_v1.set_fullscreen`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_fullscreen" since="2">
///       <description summary="request that the toplevel be fullscreened">
///         Requests that the toplevel be fullscreened on the given output. If the
///         fullscreen state and/or the outputs the toplevel is visible on actually
///         change, this will be indicated by the state and output_enter/leave
///         events.
///
///         The output parameter is only a hint to the compositor. Also, if output
///         is NULL, the compositor should decide which output the toplevel will be
///         fullscreened on, if at all.
///       </description>
///       <arg name="output" type="object" interface="wl_output" allow-null="true" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrForeignToplevelHandleV1SetFullscreenRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `output`.
    pub output: Option<u32>,
}
impl ZwlrForeignToplevelHandleV1SetFullscreenRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 8;
    /// Minimum interface version.
    pub const SINCE: u32 = 2;
}
impl CommandReader<Self> for ZwlrForeignToplevelHandleV1SetFullscreenRequest {
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
impl InterfaceReader for ZwlrForeignToplevelHandleV1SetFullscreenRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrForeignToplevelManagement(
                WlrForeignToplevelManagementObjectInteface::ZwlrForeignToplevelHandleV1 { .. }
            )
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
impl CommandWriter for ZwlrForeignToplevelHandleV1SetFullscreenRequest {
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
            "zwlr_foreign_toplevel_handle_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::WlrForeignToplevelManagement(
                WlrForeignToplevelManagementObjectInteface::ZwlrForeignToplevelHandleV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.output.unwrap_or(0)).await?;
        Ok(())
    }
}
