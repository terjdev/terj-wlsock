use super::ObjectInteface as XdgOutputObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zxdg_output_v1.logical_size`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="logical_size">
///       <description summary="size of the output in the global compositor space">
///     The logical_size event describes the size of the output in the
///     global compositor space.
///
///     Most regular Wayland clients should not pay attention to the
///     logical size and would rather rely on xdg_shell interfaces.
///
///     Some clients such as Xwayland, however, need this to configure
///     their surfaces in the global compositor space as the compositor
///     may apply a different scale from what is advertised by the output
///     scaling property (to achieve fractional scaling, for example).
///
///     For example, for a wl_output mode 3840×2160 and a scale factor 2:
///
///     - A compositor not scaling the monitor viewport in its compositing space
///       will advertise a logical size of 3840×2160,
///
///     - A compositor scaling the monitor viewport with scale factor 2 will
///       advertise a logical size of 1920×1080,
///
///     - A compositor scaling the monitor viewport using a fractional scale of
///       1.5 will advertise a logical size of 2560×1440.
///
///     For example, for a wl_output mode 1920×1080 and a 90 degree rotation,
///     the compositor will advertise a logical size of 1080x1920.
///
///     The logical_size event is sent after creating an xdg_output
///     (see xdg_output_manager.get_xdg_output) and whenever the logical
///     size of the output changes, either as a result of a change in the
///     applied scale or because of a change in the corresponding output
///     mode(see wl_output.mode) or transform (see wl_output.transform).
///       </description>
///       <arg name="width" type="int" summary="width in global compositor space" />
///       <arg name="height" type="int" summary="height in global compositor space" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZxdgOutputV1LogicalSizeEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// width in global compositor space.
    pub width: i32,
    /// height in global compositor space.
    pub height: i32,
}
impl ZxdgOutputV1LogicalSizeEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZxdgOutputV1LogicalSizeEvent {
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
impl InterfaceReader for ZxdgOutputV1LogicalSizeEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgOutput(XdgOutputObjectInteface::ZxdgOutputV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let width = reader.next_i32().await?;
        let height = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            width,
            height,
        })
    }
}
impl CommandWriter for ZxdgOutputV1LogicalSizeEvent {
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
            Self::writer_event_interface(state, self.object_id, "zxdg_output_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XdgOutput(XdgOutputObjectInteface::ZxdgOutputV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.width).await?;
        writer.next_i32(self.height).await?;
        Ok(())
    }
}
