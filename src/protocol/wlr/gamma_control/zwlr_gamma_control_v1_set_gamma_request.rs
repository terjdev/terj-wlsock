use super::ObjectInteface as WlrGammaControlObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_gamma_control_v1.set_gamma`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_gamma">
///       <description summary="set the gamma table">
///         Set the gamma table. The file descriptor can be memory-mapped to provide
///         the raw gamma table, which contains successive gamma ramps for the red,
///         green and blue channels. Each gamma ramp is an array of 16-byte unsigned
///         integers which has the same length as the gamma size.
///
///         The file descriptor data must have the same length as three times the
///         gamma size.
///       </description>
///       <arg name="fd" type="fd" summary="gamma table file descriptor" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrGammaControlV1SetGammaRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// gamma table file descriptor.
    pub fd: std::os::fd::OwnedFd,
}
impl ZwlrGammaControlV1SetGammaRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrGammaControlV1SetGammaRequest {
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
impl InterfaceReader for ZwlrGammaControlV1SetGammaRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrGammaControl(
                WlrGammaControlObjectInteface::ZwlrGammaControlV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        let fd = reader.next_fd().ok_or(FdReaderError::InvalidProtocol)?;
        Ok(Self { object_id, fd })
    }
}
impl CommandWriter for ZwlrGammaControlV1SetGammaRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(0)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "zwlr_gamma_control_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::WlrGammaControl(
                WlrGammaControlObjectInteface::ZwlrGammaControlV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_fd(self.fd).await?;
        Ok(())
    }
}
