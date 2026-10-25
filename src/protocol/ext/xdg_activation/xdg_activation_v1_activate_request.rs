use crate::protocol::ext::xdg_activation::ObjectInteface as XdgActivationObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_activation_v1.activate`.
///
/// Opcode 2; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="activate">
///       <description summary="notify new interaction being available">
///         Requests surface activation. It's up to the compositor to display
///         this information as desired, for example by placing the surface above
///         the rest.
///
///         The compositor may know who requested this by checking the activation
///         token and might decide not to follow through with the activation if it's
///         considered unwanted.
///
///         Compositors can ignore unknown activation tokens when an invalid
///         token is passed.
///       </description>
///       <arg name="token" type="string" summary="the activation token of the initiating client" />
///       <arg name="surface" type="object" interface="wl_surface" summary="the wl_surface to activate" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgActivationV1ActivateRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the activation token of the initiating client.
    pub token: String,
    /// the wl_surface to activate.
    pub surface: u32,
}
impl XdgActivationV1ActivateRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgActivationV1ActivateRequest {
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
impl InterfaceReader for XdgActivationV1ActivateRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgActivation(XdgActivationObjectInteface::XdgActivationV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let token = reader.next_string().await?;
        let surface = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            token,
            surface,
        })
    }
}
impl CommandWriter for XdgActivationV1ActivateRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_string_size(Some(&self.token)), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(state, self.object_id, "xdg_activation_v1", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.token) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.token).await?;
        writer.next_u32(self.surface).await?;
        Ok(())
    }
}
