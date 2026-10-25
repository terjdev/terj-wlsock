use super::ObjectInteface as InputMethodObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_input_method_context_v1.modifiers`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="modifiers">
///       <description summary="forward modifiers event">
///     Forward a wl_keyboard::modifiers event to the client that was not
///     processed by the input method itself.  Should be used when filtering
///     key events with grab_keyboard. The arguments should be the ones
///     from the wl_keyboard::modifiers event.
///       </description>
///       <arg name="serial" type="uint" summary="serial from wl_keyboard::modifiers" />
///       <arg name="mods_depressed" type="uint" summary="mods_depressed from wl_keyboard::modifiers" />
///       <arg name="mods_latched" type="uint" summary="mods_latched from wl_keyboard::modifiers" />
///       <arg name="mods_locked" type="uint" summary="mods_locked from wl_keyboard::modifiers" />
///       <arg name="group" type="uint" summary="group from wl_keyboard::modifiers" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpInputMethodContextV1ModifiersRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// serial from wl_keyboard::modifiers.
    pub serial: u32,
    /// mods_depressed from wl_keyboard::modifiers.
    pub mods_depressed: u32,
    /// mods_latched from wl_keyboard::modifiers.
    pub mods_latched: u32,
    /// mods_locked from wl_keyboard::modifiers.
    pub mods_locked: u32,
    /// group from wl_keyboard::modifiers.
    pub group: u32,
}
impl ZwpInputMethodContextV1ModifiersRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 11;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpInputMethodContextV1ModifiersRequest {
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
impl InterfaceReader for ZwpInputMethodContextV1ModifiersRequest {
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
        let mods_depressed = reader.next_u32().await?;
        let mods_latched = reader.next_u32().await?;
        let mods_locked = reader.next_u32().await?;
        let group = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            serial,
            mods_depressed,
            mods_latched,
            mods_locked,
            group,
        })
    }
}
impl CommandWriter for ZwpInputMethodContextV1ModifiersRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4), Some(4), Some(4)])
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
        writer.next_u32(self.mods_depressed).await?;
        writer.next_u32(self.mods_latched).await?;
        writer.next_u32(self.mods_locked).await?;
        writer.next_u32(self.group).await?;
        Ok(())
    }
}
