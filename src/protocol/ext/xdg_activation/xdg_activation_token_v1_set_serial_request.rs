use crate::protocol::ext::xdg_activation::ObjectInteface as XdgActivationObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_activation_token_v1.set_serial`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_serial">
///       <description summary="specifies the seat and serial of the activating event">
///         Provides information about the seat and serial event that requested the
///         token.
///
///         The serial can come from an input or focus event. For instance, if a
///         click triggers the launch of a third-party client, the launcher client
///         should send a set_serial request with the serial and seat from the
///         wl_pointer.button event.
///
///         Some compositors might refuse to activate toplevels when the token
///         doesn't have a valid and recent enough event serial.
///
///         Must be sent before commit. This information is optional.
///       </description>
///       <arg name="serial" type="uint" summary="the serial of the event that triggered the activation" />
///       <arg name="seat" type="object" interface="wl_seat" summary="the wl_seat of the event" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgActivationTokenV1SetSerialRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the serial of the event that triggered the activation.
    pub serial: u32,
    /// the wl_seat of the event.
    pub seat: u32,
}
impl XdgActivationTokenV1SetSerialRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgActivationTokenV1SetSerialRequest {
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
impl InterfaceReader for XdgActivationTokenV1SetSerialRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgActivation(XdgActivationObjectInteface::XdgActivationTokenV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let serial = reader.next_u32().await?;
        let seat = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            serial,
            seat,
        })
    }
}
impl CommandWriter for XdgActivationTokenV1SetSerialRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(
            state,
            self.object_id,
            "xdg_activation_token_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        writer.next_u32(self.seat).await?;
        Ok(())
    }
}
