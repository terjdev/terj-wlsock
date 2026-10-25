use super::ObjectInteface as PointerConstraintsObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_locked_pointer_v1.set_cursor_position_hint`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_cursor_position_hint">
///       <description summary="set the pointer cursor position hint">
///     Set the cursor position hint relative to the top left corner of the
///     surface.
///
///     If the client is drawing its own cursor, it should update the position
///     hint to the position of its own cursor. A compositor may use this
///     information to warp the pointer upon unlock in order to avoid pointer
///     jumps.
///
///     The cursor position hint is double-buffered state, see
///     wl_surface.commit.
///       </description>
///       <arg name="surface_x" type="fixed" summary="surface-local x coordinate" />
///       <arg name="surface_y" type="fixed" summary="surface-local y coordinate" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpLockedPointerV1SetCursorPositionHintRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// surface-local x coordinate.
    pub surface_x: f64,
    /// surface-local y coordinate.
    pub surface_y: f64,
}
impl ZwpLockedPointerV1SetCursorPositionHintRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpLockedPointerV1SetCursorPositionHintRequest {
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
impl InterfaceReader for ZwpLockedPointerV1SetCursorPositionHintRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::PointerConstraints(
                PointerConstraintsObjectInteface::ZwpLockedPointerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let surface_x = reader.next_fixed().await?;
        let surface_y = reader.next_fixed().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            surface_x,
            surface_y,
        })
    }
}
impl CommandWriter for ZwpLockedPointerV1SetCursorPositionHintRequest {
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
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "zwp_locked_pointer_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::PointerConstraints(
                PointerConstraintsObjectInteface::ZwpLockedPointerV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.surface_x) {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.surface_y) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_fixed(self.surface_x).await?;
        writer.next_fixed(self.surface_y).await?;
        Ok(())
    }
}
