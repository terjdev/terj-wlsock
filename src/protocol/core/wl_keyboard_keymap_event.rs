use super::WlKeyboardKeymapFormat;
use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_keyboard.keymap`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="keymap">
///       <description summary="keyboard mapping">
///         This event provides a file descriptor to the client which can be
///         memory-mapped in read-only mode to provide a keyboard mapping
///         description.
///
///         From version 7 onwards, the fd must be mapped with MAP_PRIVATE by
///         the recipient, as MAP_SHARED may fail.
///       </description>
///       <arg name="format" type="uint" enum="keymap_format" summary="keymap format" />
///       <arg name="fd" type="fd" summary="keymap file descriptor" />
///       <arg name="size" type="uint" summary="keymap size, in bytes" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlKeyboardKeymapEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `format`: keymap format.
    pub format: WlKeyboardKeymapFormat,
    /// Argument `fd`: keymap file descriptor.
    pub fd: std::os::fd::OwnedFd,
    /// Argument `size`: keymap size, in bytes.
    pub size: u32,
}
impl WlKeyboardKeymapEvent {
    /// Opcode in the event namespace of interface wl_keyboard.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlKeyboardKeymapEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(0), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_event_interface(state, self.object_id, "wl_keyboard", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.format.0).await?;
        writer.next_fd(self.fd).await?;
        writer.next_u32(self.size).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlKeyboardKeymapEvent {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Self::reader_event_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_keyboard",
            Self::SINCE,
        )?;
        let format = WlKeyboardKeymapFormat::read(object_id, opcode, reader, state).await?;
        let size = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        let fd = reader.next_fd().ok_or(FdReaderError::InvalidProtocol)?;
        Ok(Self {
            object_id,
            format,
            fd,
            size,
        })
    }
}
