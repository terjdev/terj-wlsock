use super::ObjectInteface as XxHotkeyObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_hotkey_manager_v1.set_app_id`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_app_id">
///       <description summary="set the application identifier">
///         Identify the application, for use in compositor policy and user-facing
///         audit interfaces. It applies to every hotkey created by this manager.
///
///         Optional; if sent, it must not be empty, must be sent at most once, and
///         must be sent before the first commit, so that every policy decision
///         sees the same identity.
///
///         Where the application has a desktop entry, app_id should be its desktop
///         file ID as defined by the freedesktop.org Desktop Entry specification,
///         matching the convention used by xdg_toplevel.set_app_id.
///
///         It is advisory and may be spoofed; a compositor that needs a trustworthy
///         identity should obtain it through the security-context mechanism.
///       </description>
///       <arg name="app_id" type="string" summary="advisory application identifier (should be the desktop file ID)" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxHotkeyManagerV1SetAppIdRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// advisory application identifier (should be the desktop file ID).
    pub app_id: String,
}
impl XxHotkeyManagerV1SetAppIdRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxHotkeyManagerV1SetAppIdRequest {
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
impl InterfaceReader for XxHotkeyManagerV1SetAppIdRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxHotkey(XxHotkeyObjectInteface::XxHotkeyManagerV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let app_id = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, app_id })
    }
}
impl CommandWriter for XxHotkeyManagerV1SetAppIdRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_string_size(Some(&self.app_id))])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "xx_hotkey_manager_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::XxHotkey(XxHotkeyObjectInteface::XxHotkeyManagerV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.app_id) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.app_id).await?;
        Ok(())
    }
}
