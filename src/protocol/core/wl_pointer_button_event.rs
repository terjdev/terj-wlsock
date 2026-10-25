use super::WlPointerButtonState;
use crate::protocol::ClientState;
use crate::protocol::core::ObjectInteface as CoreObjectInteface;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_pointer.button`.
///
/// Opcode 3; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="button">
///       <description summary="pointer button event">
///         Mouse button click and release notifications.
///
///         The location of the click is given by the last motion, warp or
///         enter event.
///         The time argument is a timestamp with millisecond
///         granularity, with an undefined base.
///
///         The button is a button code as defined in the Linux kernel's
///         linux/input-event-codes.h header file, e.g. BTN_LEFT.
///
///         Any 16-bit button code value is reserved for future additions to the
///         kernel's event code list. All other button codes above 0xFFFF are
///         currently undefined but may be used in future versions of this
///         protocol.
///       </description>
///       <arg name="serial" type="uint" summary="serial number of the button event" />
///       <arg name="time" type="uint" summary="timestamp with millisecond granularity" />
///       <arg name="button" type="uint" summary="button that produced the event" />
///       <arg name="state" type="uint" enum="button_state" summary="physical state of the button" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlPointerButtonEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `serial`: serial number of the button event.
    pub serial: u32,
    /// Argument `time`: timestamp with millisecond granularity.
    pub time: u32,
    /// Argument `button`: button that produced the event.
    pub button: u32,
    /// Argument `state`: physical state of the button.
    pub state: WlPointerButtonState,
}
impl WlPointerButtonEvent {
    /// Opcode in the event namespace of interface wl_pointer.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlPointerButtonEvent {
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
        Self::writer_event_interface(state, self.object_id, "wl_pointer", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        writer.next_u32(self.time).await?;
        writer.next_u32(self.button).await?;
        writer.next_u32(self.state.0).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlPointerButtonEvent {
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

impl InterfaceReader for WlPointerButtonEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Core(CoreObjectInteface::WlPointer { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let serial = reader.next_u32().await?;
        let time = reader.next_u32().await?;
        let button = reader.next_u32().await?;
        let state = WlPointerButtonState::read(object_id, opcode, reader, state).await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            serial,
            time,
            button,
            state,
        })
    }
}
