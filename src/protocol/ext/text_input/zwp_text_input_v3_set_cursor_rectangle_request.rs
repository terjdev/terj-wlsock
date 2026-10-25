use super::ObjectInteface as TextInputObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_text_input_v3.set_cursor_rectangle`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_cursor_rectangle">
///       <description summary="set cursor position">
///         Marks an area around the cursor as a x, y, width, height rectangle in
///         surface local coordinates.
///
///         Allows the compositor to put a window with word suggestions near the
///         cursor, without obstructing the text being input.
///
///         If the client is unaware of the position of edited text, it should not
///         issue this request, to signify lack of support to the compositor.
///
///         Values set with this request are double-buffered. They will get applied
///         on the next zwp_text_input_v3.commit request, and stay valid until the
///         next committed enable or disable request.
///
///         The initial values describing a cursor rectangle are empty. That means
///         the text input does not support describing the cursor area. If the
///         empty values get applied, subsequent attempts to change them may have
///         no effect.
///
///         As of version 2, the zwp_text_input_v3.commit request does not apply
///         values sent with this request. Instead, it stores them in a separate
///         "committed" area. The committed values, if still valid, get applied on
///         the next wl_surface.commit request on the surface with text-input focus.
///         Both committed and applied values get invalidated on:
///
///         - the next committed enable or disable request, or
///         - a change of the focused surface of the text-input (leave or enter events).
///
///         This double stage application allows the compositor to position
///         the input method popup in the same frame as the contents
///         of the text on the surface are updated.
///       </description>
///       <arg name="x" type="int" />
///       <arg name="y" type="int" />
///       <arg name="width" type="int" />
///       <arg name="height" type="int" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpTextInputV3SetCursorRectangleRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `x`.
    pub x: i32,
    /// Wire argument `y`.
    pub y: i32,
    /// Wire argument `width`.
    pub width: i32,
    /// Wire argument `height`.
    pub height: i32,
}
impl ZwpTextInputV3SetCursorRectangleRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 6;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTextInputV3SetCursorRectangleRequest {
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
impl InterfaceReader for ZwpTextInputV3SetCursorRectangleRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::TextInput(TextInputObjectInteface::ZwpTextInputV3 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let x = reader.next_i32().await?;
        let y = reader.next_i32().await?;
        let width = reader.next_i32().await?;
        let height = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            x,
            y,
            width,
            height,
        })
    }
}
impl CommandWriter for ZwpTextInputV3SetCursorRectangleRequest {
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
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "zwp_text_input_v3",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::TextInput(TextInputObjectInteface::ZwpTextInputV3 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.x).await?;
        writer.next_i32(self.y).await?;
        writer.next_i32(self.width).await?;
        writer.next_i32(self.height).await?;
        Ok(())
    }
}
