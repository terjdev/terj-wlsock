use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_popup.grab`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="grab">
///       <description summary="make the popup take an explicit grab">
///     This request makes the created popup take an explicit grab. An explicit
///     grab will be dismissed when the user dismisses the popup, or when the
///     client destroys the xdg_popup. This can be done by the user clicking
///     outside the surface, using the keyboard, or even locking the screen
///     through closing the lid or a timeout.
///
///     If the compositor denies the grab, the popup will be immediately
///     dismissed.
///
///     This request must be used in response to some sort of user action like a
///     button press, key press, or touch down event. The serial number of the
///     event should be passed as 'serial'.
///
///     The parent of a grabbing popup must either be an xdg_toplevel surface or
///     another xdg_popup with an explicit grab. If the parent is another
///     xdg_popup it means that the popups are nested, with this popup now being
///     the topmost popup.
///
///     Nested popups must be destroyed in the reverse order they were created
///     in, e.g. the only popup you are allowed to destroy at all times is the
///     topmost one.
///
///     When compositors choose to dismiss a popup, they may dismiss every
///     nested grabbing popup as well. When a compositor dismisses popups, it
///     will follow the same dismissing order as required from the client.
///
///     If the topmost grabbing popup is destroyed, the grab will be returned to
///     the parent of the popup, if that parent previously had an explicit grab.
///
///     If the parent is a grabbing popup which has already been dismissed, this
///     popup will be immediately dismissed. If the parent is a popup that did
///     not take an explicit grab, an error will be raised.
///
///     During a popup grab, the client owning the grab will receive pointer
///     and touch events for all their surfaces as normal (similar to an
///     "owner-events" grab in X11 parlance), while the top most grabbing popup
///     will always have keyboard focus.
///       </description>
///       <arg name="seat" type="object" interface="wl_seat" summary="the wl_seat of the user event" />
///       <arg name="serial" type="uint" summary="the serial of the user event" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgPopupGrabRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the wl_seat of the user event.
    pub seat: u32,
    /// the serial of the user event.
    pub serial: u32,
}
impl XdgPopupGrabRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgPopupGrabRequest {
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
impl InterfaceReader for XdgPopupGrabRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgShell(XdgShellObjectInteface::XdgPopup { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let seat = reader.next_u32().await?;
        let serial = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            seat,
            serial,
        })
    }
}
impl CommandWriter for XdgPopupGrabRequest {
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
        Self::writer_request_interface(state, self.object_id, "xdg_popup", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.seat).await?;
        writer.next_u32(self.serial).await?;
        Ok(())
    }
}
