use super::ObjectInteface as XwaylandKeyboardGrabObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_xwayland_keyboard_grab_manager_v1.grab_keyboard`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="grab_keyboard">
///       <description summary="grab the keyboard to a surface">
///     The grab_keyboard request asks for a grab of the keyboard, forcing
///     the keyboard focus for the given seat upon the given surface.
///
///     The protocol provides no guarantee that the grab is ever satisfied,
///     and does not require the compositor to send an error if the grab
///     cannot ever be satisfied. It is thus possible to request a keyboard
///     grab that will never be effective.
///
///     The protocol:
///
///     * does not guarantee that the grab itself is applied for a surface,
///       the grab request may be silently ignored by the compositor,
///     * does not guarantee that any events are sent to this client even
///       if the grab is applied to a surface,
///     * does not guarantee that events sent to this client are exhaustive,
///       a compositor may filter some events for its own consumption,
///     * does not guarantee that events sent to this client are continuous,
///       a compositor may change and reroute keyboard events while the grab
///       is nominally active.
///       </description>
///
///       <arg name="id" type="new_id" interface="zwp_xwayland_keyboard_grab_v1" />
///       <arg name="surface" type="object" interface="wl_surface" summary="surface to report keyboard events to" />
///       <arg name="seat" type="object" interface="wl_seat" summary="the seat for which the keyboard should be grabbed" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpXwaylandKeyboardGrabManagerV1GrabKeyboardRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `id`.
    pub id: u32,
    /// surface to report keyboard events to.
    pub surface: u32,
    /// the seat for which the keyboard should be grabbed.
    pub seat: u32,
}
impl ZwpXwaylandKeyboardGrabManagerV1GrabKeyboardRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpXwaylandKeyboardGrabManagerV1GrabKeyboardRequest {
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
impl InterfaceReader for ZwpXwaylandKeyboardGrabManagerV1GrabKeyboardRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XwaylandKeyboardGrab(
                XwaylandKeyboardGrabObjectInteface::ZwpXwaylandKeyboardGrabManagerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = reader.next_u32().await?;
        let surface = reader.next_u32().await?;
        let seat = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::XwaylandKeyboardGrab(
                XwaylandKeyboardGrabObjectInteface::ZwpXwaylandKeyboardGrabV1 {
                    version: interface.version().min(1),
                },
            );
            Self::reader_state(state)?
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            surface,
            seat,
        })
    }
}
impl CommandWriter for ZwpXwaylandKeyboardGrabManagerV1GrabKeyboardRequest {
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
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "zwp_xwayland_keyboard_grab_manager_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::XwaylandKeyboardGrab(
                XwaylandKeyboardGrabObjectInteface::ZwpXwaylandKeyboardGrabManagerV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::XwaylandKeyboardGrab(
                XwaylandKeyboardGrabObjectInteface::ZwpXwaylandKeyboardGrabV1 {
                    version: interface.version().min(1),
                },
            );
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_u32(self.surface).await?;
        writer.next_u32(self.seat).await?;
        Ok(())
    }
}
