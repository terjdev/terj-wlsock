use super::ObjectInteface as XxHotkeyObjectInteface;
use super::XxHotkeyManagerV1Modifiers;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_hotkey_v1.set_button_trigger`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_button_trigger">
///       <description summary="trigger on a pointer button">
///         Describe the trigger as a pointer button pressed while the given
///         modifiers are held. Replaces any trigger previously described. Takes
///         effect on the next commit.
///
///         The button is a button code in the same space as wl_pointer.button.
///
///         The press and the matching release are consumed and must not be
///         delivered to the surface under the pointer.
///       </description>
///       <arg name="button" type="uint" summary="button code, as in wl_pointer.button" />
///       <arg name="modifiers" type="uint" enum="xx_hotkey_manager_v1.modifiers" summary="bitmask of required modifiers" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxHotkeyV1SetButtonTriggerRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// button code, as in wl_pointer.button.
    pub button: u32,
    /// bitmask of required modifiers.
    pub modifiers: XxHotkeyManagerV1Modifiers,
}
impl XxHotkeyV1SetButtonTriggerRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 4;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxHotkeyV1SetButtonTriggerRequest {
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
impl InterfaceReader for XxHotkeyV1SetButtonTriggerRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxHotkey(XxHotkeyObjectInteface::XxHotkeyV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let button = reader.next_u32().await?;
        let modifiers = XxHotkeyManagerV1Modifiers(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            button,
            modifiers,
        })
    }
}
impl CommandWriter for XxHotkeyV1SetButtonTriggerRequest {
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
        let interface =
            Self::writer_request_interface(state, self.object_id, "xx_hotkey_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XxHotkey(XxHotkeyObjectInteface::XxHotkeyV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.button).await?;
        writer.next_u32(self.modifiers.0).await?;
        Ok(())
    }
}
