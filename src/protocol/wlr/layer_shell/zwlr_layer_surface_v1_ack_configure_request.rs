use crate::protocol::wlr::layer_shell::ObjectInteface as WlrLayerShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_layer_surface_v1.ack_configure`.
///
/// Opcode 6; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="ack_configure">
///       <description summary="ack a configure event">
///         When a configure event is received, if a client commits the
///         surface in response to the configure event, then the client
///         must make an ack_configure request sometime before the commit
///         request, passing along the serial of the configure event.
///
///         If the client receives multiple configure events before it
///         can respond to one, it only has to ack the last configure event.
///
///         A client is not required to commit immediately after sending
///         an ack_configure request - it may even ack_configure several times
///         before its next surface commit.
///
///         A client may send multiple ack_configure requests before committing, but
///         only the last request sent before a commit indicates which configure
///         event the client really is responding to.
///       </description>
///       <arg name="serial" type="uint" summary="the serial from the configure event" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrLayerSurfaceV1AckConfigureRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the serial from the configure event.
    pub serial: u32,
}
impl ZwlrLayerSurfaceV1AckConfigureRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 6;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrLayerSurfaceV1AckConfigureRequest {
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
impl InterfaceReader for ZwlrLayerSurfaceV1AckConfigureRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrLayerShell(WlrLayerShellObjectInteface::ZwlrLayerSurfaceV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let serial = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, serial })
    }
}
impl CommandWriter for ZwlrLayerSurfaceV1AckConfigureRequest {
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
        Self::writer_request_interface(
            state,
            self.object_id,
            "zwlr_layer_surface_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        Ok(())
    }
}
