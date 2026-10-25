use super::ObjectInteface as InputMethodObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_input_method_context_v1.key`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="key">
///       <description summary="forward key event">
///     Forward a wl_keyboard::key event to the client that was not processed
///     by the input method itself. Should be used when filtering key events
///     with grab_keyboard.  The arguments should be the ones from the
///     wl_keyboard::key event.
///
///     For generating custom key events use the keysym request instead.
///       </description>
///       <arg name="serial" type="uint" summary="serial from wl_keyboard::key" />
///       <arg name="time" type="uint" summary="time from wl_keyboard::key" />
///       <arg name="key" type="uint" summary="key from wl_keyboard::key" />
///       <arg name="state" type="uint" summary="state from wl_keyboard::key" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpInputMethodContextV1KeyRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// serial from wl_keyboard::key.
    pub serial: u32,
    /// time from wl_keyboard::key.
    pub time: u32,
    /// key from wl_keyboard::key.
    pub key: u32,
    /// state from wl_keyboard::key.
    pub state: u32,
}
impl ZwpInputMethodContextV1KeyRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 10;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpInputMethodContextV1KeyRequest {
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
impl InterfaceReader for ZwpInputMethodContextV1KeyRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::InputMethod(InputMethodObjectInteface::ZwpInputMethodContextV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let serial = reader.next_u32().await?;
        let time = reader.next_u32().await?;
        let key = reader.next_u32().await?;
        let state = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            serial,
            time,
            key,
            state,
        })
    }
}
impl CommandWriter for ZwpInputMethodContextV1KeyRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "zwp_input_method_context_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::InputMethod(InputMethodObjectInteface::ZwpInputMethodContextV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        writer.next_u32(self.time).await?;
        writer.next_u32(self.key).await?;
        writer.next_u32(self.state).await?;
        Ok(())
    }
}
