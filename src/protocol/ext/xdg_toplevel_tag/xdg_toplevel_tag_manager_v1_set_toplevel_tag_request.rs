use crate::protocol::ext::xdg_toplevel_tag::ObjectInteface as XdgToplevelTagObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_toplevel_tag_manager_v1.set_toplevel_tag`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_toplevel_tag">
///       <description summary="set tag">
///         Set a tag for a toplevel. The tag may be shown to the user in UI, so
///         it's preferable for it to be human readable, but it must be suitable
///         for configuration files and should not be translated.
///         Suitable tags would for example be "main window", "settings",
///         "e-mail composer" or similar.
///
///         The tag does not need to be unique across applications, and the client
///         may set the same tag for multiple windows, for example if the user has
///         opened the same UI twice. How the potentially resulting conflicts are
///         handled is compositor policy.
///
///         The client should set the tag as part of the initial commit on the
///         associated toplevel, but it may set it at any time afterwards as well,
///         for example if the purpose of the toplevel changes.
///       </description>
///       <arg name="toplevel" type="object" interface="xdg_toplevel" />
///       <arg name="tag" type="string" summary="untranslated tag" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgToplevelTagManagerV1SetToplevelTagRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `toplevel`.
    pub toplevel: u32,
    /// untranslated tag.
    pub tag: String,
}
impl XdgToplevelTagManagerV1SetToplevelTagRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgToplevelTagManagerV1SetToplevelTagRequest {
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
impl InterfaceReader for XdgToplevelTagManagerV1SetToplevelTagRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgToplevelTag(
                XdgToplevelTagObjectInteface::XdgToplevelTagManagerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let toplevel = reader.next_u32().await?;
        let tag = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            toplevel,
            tag,
        })
    }
}
impl CommandWriter for XdgToplevelTagManagerV1SetToplevelTagRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), FdWriter::encoded_string_size(Some(&self.tag))])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(
            state,
            self.object_id,
            "xdg_toplevel_tag_manager_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.tag) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.toplevel).await?;
        writer.next_string(&self.tag).await?;
        Ok(())
    }
}
