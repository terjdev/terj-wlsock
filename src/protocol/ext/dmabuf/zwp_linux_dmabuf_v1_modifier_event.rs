use crate::protocol::ext::dmabuf::ObjectInteface as DmabufObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_linux_dmabuf_v1.modifier`.
///
/// Opcode 1; available since version 3.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// Sending or receiving this event is rejected at version 4 and above.
/// ```xml
/// <event name="modifier" since="3" deprecated-since="4">
///       <description summary="supported buffer format modifier">
///         This event advertises the formats that the server supports, along with
///         the modifiers supported for each format. All the supported modifiers
///         for all the supported formats are advertised once when the client
///         binds to this interface. A roundtrip after binding guarantees that
///         the client has received all supported format-modifier pairs.
///
///         For legacy support, DRM_FORMAT_MOD_INVALID (that is, modifier_hi ==
///         0x00ffffff and modifier_lo == 0xffffffff) is allowed in this event.
///         It indicates that the server can support the format with an implicit
///         modifier. When a plane has DRM_FORMAT_MOD_INVALID as its modifier, it
///         is as if no explicit modifier is specified. The effective modifier
///         will be derived from the dmabuf.
///
///         A compositor that sends valid modifiers and DRM_FORMAT_MOD_INVALID for
///         a given format supports both explicit modifiers and implicit modifiers.
///
///         For the definition of the format and modifier codes, see the
///         zwp_linux_buffer_params_v1::create and zwp_linux_buffer_params_v1::add
///         requests.
///
///         Starting version 4, the modifier event is deprecated and must not be
///         sent by compositors. Instead, use get_default_feedback or
///         get_surface_feedback.
///       </description>
///       <arg name="format" type="uint" summary="DRM_FORMAT code" />
///       <arg name="modifier_hi" type="uint" summary="high 32 bits of layout modifier" />
///       <arg name="modifier_lo" type="uint" summary="low 32 bits of layout modifier" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpLinuxDmabufV1ModifierEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// DRM_FORMAT code.
    pub format: u32,
    /// high 32 bits of layout modifier.
    pub modifier_hi: u32,
    /// low 32 bits of layout modifier.
    pub modifier_lo: u32,
}
impl ZwpLinuxDmabufV1ModifierEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 3;
    /// First interface version prohibiting this event.
    pub const DEPRECATED_SINCE: u32 = 4;
}
impl CommandReader<Self> for ZwpLinuxDmabufV1ModifierEvent {
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
impl InterfaceReader for ZwpLinuxDmabufV1ModifierEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Dmabuf(DmabufObjectInteface::ZwpLinuxDmabufV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
            || interface.version() >= Self::DEPRECATED_SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let format = reader.next_u32().await?;
        let modifier_hi = reader.next_u32().await?;
        let modifier_lo = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            format,
            modifier_hi,
            modifier_lo,
        })
    }
}
impl CommandWriter for ZwpLinuxDmabufV1ModifierEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "zwp_linux_dmabuf_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota()
            || interface.version() >= Self::DEPRECATED_SINCE
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.format).await?;
        writer.next_u32(self.modifier_hi).await?;
        writer.next_u32(self.modifier_lo).await?;
        Ok(())
    }
}
