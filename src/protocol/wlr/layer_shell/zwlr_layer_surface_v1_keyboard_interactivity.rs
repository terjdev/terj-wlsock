use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwlr_layer_surface_v1.keyboard_interactivity`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="keyboard_interactivity">
///       <description summary="types of keyboard interaction possible for a layer shell surface">
///         Types of keyboard interaction possible for layer shell surfaces. The
///         rationale for this is twofold: (1) some applications are not interested
///         in keyboard events and not allowing them to be focused can improve the
///         desktop experience; (2) some applications will want to take exclusive
///         keyboard focus.
///       </description>
///
///       <entry name="none" value="0">
///         <description summary="no keyboard focus is possible">
///           This value indicates that this surface is not interested in keyboard
///           events and the compositor should never assign it the keyboard focus.
///
///           This is the default value, set for newly created layer shell surfaces.
///
///           This is useful for e.g. desktop widgets that display information or
///           only have interaction with non-keyboard input devices.
///         </description>
///       </entry>
///       <entry name="exclusive" value="1">
///         <description summary="request exclusive keyboard focus">
///           Request exclusive keyboard focus if this surface is above the shell surface layer.
///
///           For the top and overlay layers, the seat will always give
///           exclusive keyboard focus to the top-most layer which has keyboard
///           interactivity set to exclusive. If this layer contains multiple
///           surfaces with keyboard interactivity set to exclusive, the compositor
///           determines the one receiving keyboard events in an implementation-
///           defined manner. In this case, no guarantee is made when this surface
///           will receive keyboard focus (if ever).
///
///           For the bottom and background layers, the compositor is allowed to use
///           normal focus semantics.
///
///           This setting is mainly intended for applications that need to ensure
///           they receive all keyboard events, such as a lock screen or a password
///           prompt.
///         </description>
///       </entry>
///       <entry name="on_demand" value="2" since="4">
///         <description summary="request regular keyboard focus semantics">
///           This requests the compositor to allow this surface to be focused and
///           unfocused by the user in an implementation-defined manner. The user
///           should be able to unfocus this surface even regardless of the layer
///           it is on.
///
///           Typically, the compositor will want to use its normal mechanism to
///           manage keyboard focus between layer shell surfaces with this setting
///           and regular toplevels on the desktop layer (e.g. click to focus).
///           Nevertheless, it is possible for a compositor to require a special
///           interaction to focus or unfocus layer shell surfaces (e.g. requiring
///           a click even if focus follows the mouse normally, or providing a
///           keybinding to switch focus between layers).
///
///           This setting is mainly intended for desktop shell components (e.g.
///           panels) that allow keyboard interaction. Using this option can allow
///           implementing a desktop shell that can be fully usable without the
///           mouse.
///         </description>
///       </entry>
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwlrLayerSurfaceV1KeyboardInteractivity(pub u32);
impl ZwlrLayerSurfaceV1KeyboardInteractivity {
    /// no keyboard focus is possible; available since version 1.
    pub const NONE: Self = Self(0);
    /// request exclusive keyboard focus; available since version 1.
    pub const EXCLUSIVE: Self = Self(1);
    /// request regular keyboard focus semantics; available since version 4.
    pub const ON_DEMAND: Self = Self(2);
}
impl CommandReader<Self> for ZwlrLayerSurfaceV1KeyboardInteractivity {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwlrLayerSurfaceV1KeyboardInteractivity {
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
