use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_data_device_manager.dnd_action`.
///
/// The numeric representation preserves unknown values without losing information.
/// The bitmask supports combining values with `|`.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="dnd_action" bitfield="true" since="3">
///       <description summary="drag and drop actions">
///         This is a bitmask of the available/preferred actions in a
///         drag-and-drop operation.
///
///         In the compositor, the selected action is a result of matching the
///         actions offered by the source and destination sides.  "action" events
///         with a "none" action will be sent to both source and destination if
///         there is no match. All further checks will effectively happen on
///         (source actions ∩ destination actions).
///
///         In addition, compositors may also pick different actions in
///         reaction to key modifiers being pressed. One common design that
///         is used in major toolkits (and the behavior recommended for
///         compositors) is:
///
///         - If no modifiers are pressed, the first match (in bit order)
///           will be used.
///         - Pressing Shift selects "move", if enabled in the mask.
///         - Pressing Control selects "copy", if enabled in the mask.
///
///         Behavior beyond that is considered implementation-dependent.
///         Compositors may for example bind other modifiers (like Alt/Meta)
///         or drags initiated with other buttons than BTN_LEFT to specific
///         actions (e.g. "ask").
///       </description>
///       <entry name="none" value="0" summary="no action" />
///       <entry name="copy" value="1" summary="copy action" />
///       <entry name="move" value="2" summary="move action" />
///       <entry name="ask" value="4" summary="ask action" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlDataDeviceManagerDndAction(pub u32);
impl WlDataDeviceManagerDndAction {
    /// no action; XML since=1.
    /// ```xml
    /// <entry name="none" value="0" summary="no action" />
    /// ```
    pub const NONE: Self = Self(0);
    /// copy action; XML since=1.
    /// ```xml
    /// <entry name="copy" value="1" summary="copy action" />
    /// ```
    pub const COPY: Self = Self(1);
    /// move action; XML since=1.
    /// ```xml
    /// <entry name="move" value="2" summary="move action" />
    /// ```
    pub const MOVE: Self = Self(2);
    /// ask action; XML since=1.
    /// ```xml
    /// <entry name="ask" value="4" summary="ask action" />
    /// ```
    pub const ASK: Self = Self(4);
}
impl std::ops::BitOr for WlDataDeviceManagerDndAction {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl std::ops::BitAnd for WlDataDeviceManagerDndAction {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl CommandReader<Self> for WlDataDeviceManagerDndAction {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlDataDeviceManagerDndAction {
    fn object_id(&self) -> u32 {
        0
    }
    fn opcode(&self) -> u16 {
        0
    }
    fn payload_size(&self) -> usize {
        4
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        writer.next_u32(self.0).await
    }
}
