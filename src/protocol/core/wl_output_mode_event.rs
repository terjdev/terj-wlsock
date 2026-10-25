use super::WlOutputMode;
use crate::protocol::ClientState;
use crate::protocol::core::ObjectInteface as CoreObjectInteface;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_output.mode`.
///
/// Opcode 1; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="mode">
///       <description summary="advertise available modes for the output">
///         The mode event describes an available mode for the output.
///
///         The event is sent when binding to the output object and there
///         will always be one mode, the current mode.  The event is sent
///         again if an output changes mode, for the mode that is now
///         current.  In other words, the current mode is always the last
///         mode that was received with the current flag set.
///
///         Non-current modes are deprecated. A compositor can decide to only
///         advertise the current mode and never send other modes. Clients
///         should not rely on non-current modes.
///
///         The size of a mode is given in physical hardware units of
///         the output device. This is not necessarily the same as
///         the output size in the global compositor space. For instance,
///         the output may be scaled, as described in wl_output.scale,
///         or transformed, as described in wl_output.transform. Clients
///         willing to retrieve the output size in the global compositor
///         space should use xdg_output.logical_size instead.
///
///         The vertical refresh rate can be set to zero if it doesn't make
///         sense for this output (e.g. for virtual outputs).
///
///         The mode event will be followed by a done event (starting from
///         version 2).
///
///         Clients should not use the refresh rate to schedule frames. Instead,
///         they should use the wl_surface.frame event or the presentation-time
///         protocol.
///
///         Note: this information is not always meaningful for all outputs. Some
///         compositors, such as those exposing virtual outputs, might fake the
///         refresh rate or the size.
///       </description>
///       <arg name="flags" type="uint" enum="mode" summary="bitfield of mode flags" />
///       <arg name="width" type="int" summary="width of the mode in hardware units" />
///       <arg name="height" type="int" summary="height of the mode in hardware units" />
///       <arg name="refresh" type="int" summary="vertical refresh rate in mHz" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlOutputModeEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `flags`: bitfield of mode flags.
    pub flags: WlOutputMode,
    /// Argument `width`: width of the mode in hardware units.
    pub width: i32,
    /// Argument `height`: height of the mode in hardware units.
    pub height: i32,
    /// Argument `refresh`: vertical refresh rate in mHz.
    pub refresh: i32,
}
impl WlOutputModeEvent {
    /// Opcode in the event namespace of interface wl_output.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlOutputModeEvent {
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
        Self::writer_event_interface(state, self.object_id, "wl_output", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.flags.0).await?;
        writer.next_i32(self.width).await?;
        writer.next_i32(self.height).await?;
        writer.next_i32(self.refresh).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlOutputModeEvent {
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

impl InterfaceReader for WlOutputModeEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Core(CoreObjectInteface::WlOutput { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let flags = WlOutputMode::read(object_id, opcode, reader, state).await?;
        let width = reader.next_i32().await?;
        let height = reader.next_i32().await?;
        let refresh = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            flags,
            width,
            height,
            refresh,
        })
    }
}
