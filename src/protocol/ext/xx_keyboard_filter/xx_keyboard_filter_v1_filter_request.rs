use super::ObjectInteface as XxKeyboardFilterObjectInteface;
use super::XxKeyboardFilterV1FilterAction;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_keyboard_filter_v1.filter`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="filter">
///       <description summary="decide the processing of a keyboard event">
///         This request controls the filtering of keyboard input events before reaching the focused surface.
///
///         Usage:
///
///         While xx_keyboard_filter is intercepting, the compositor must send every intercepted event to its bound wl_keyboard, and hold a copy of it in an internal queue.
///         When the client responds with the .filter request, the compositor either removes the event from the queue (filter_action.consume), or sends the copy to the original wl_keyboard objects (filter_action.passthrough).
///
///         The compositor must process .filter the oldest event in the queue before processing more recent ones.
///         For this reason, the client sets the argument "serial" to the serial of the corresponding event it received.
///
///         Exceptions:
///
///         If the event is other than wl_keyboard.key or contains no serial, it cannot be filtered. The xx_keyboard_filter client must not respond to it with .filter request. When such an event is oldest in the queue, the compositor must proceed as if the event had received a "passthrough" reply.
///
///         As of wl_keyboard v10 and keyboard_filter_v1, the only event that can be filtered is the wl_keyboard.key event.
///
///         Sequence:
///
///         The wl_keyboard begins to receive events after input_method.activate is committed.
///         The valid serial is the serial of the oldest wl_keyboard event which has been sent after input_method.activate but which hasn't yet received a .filter confirmation.
///         The compositor may raise the invalid_serial error in response to events with serials it had not issued.
///         The compositor must ignore events with all other serials. (Particularly, this means events with repeating serials are accepted normally and are not ignored).
///         Events must be filtered in order of arrival.
///       </description>
///       <arg name="serial" type="uint" />
///       <arg name="action" type="uint" enum="filter_action" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxKeyboardFilterV1FilterRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `serial`.
    pub serial: u32,
    /// Wire argument `action`.
    pub action: XxKeyboardFilterV1FilterAction,
}
impl XxKeyboardFilterV1FilterRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxKeyboardFilterV1FilterRequest {
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
impl InterfaceReader for XxKeyboardFilterV1FilterRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxKeyboardFilter(
                XxKeyboardFilterObjectInteface::XxKeyboardFilterV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let serial = reader.next_u32().await?;
        let action = XxKeyboardFilterV1FilterAction(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            serial,
            action,
        })
    }
}
impl CommandWriter for XxKeyboardFilterV1FilterRequest {
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
            "xx_keyboard_filter_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::XxKeyboardFilter(
                XxKeyboardFilterObjectInteface::XxKeyboardFilterV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        writer.next_u32(self.action.0).await?;
        Ok(())
    }
}
