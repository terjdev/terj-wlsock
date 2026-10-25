use super::ObjectInteface as XxCutoutsObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xx_cutouts_v1.configure`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="configure">
///       <description summary="notify cutout changes">
///         The configure event marks the end of a configure sequence. A
///         configure sequence is a set of zero or more cutout events and
///         the final xx_cutout.configure event.
///
///         In the case of a xdg_toplevel clients should arrange their
///         surface for the new cutouts, and then send an
///         xdg_surface.ack_configure request at some point before
///         committing the new surface. See xdg_surface.configure and
///         xdg_surface.ack_configure in the xdg_shell protocol for
///         details.
///
///         If the cutout sequence consists of only a configure event and
///         contains no cutout or corner events this indicates that the
///         surface isn't overlapping with any cutouts or corners.
///
///         If the client receives multiple configure events before it can
///         respond to one, it is free to discard all but the last event
///         it received.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct XxCutoutsV1ConfigureEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl XxCutoutsV1ConfigureEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxCutoutsV1ConfigureEvent {
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
impl InterfaceReader for XxCutoutsV1ConfigureEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxCutouts(XxCutoutsObjectInteface::XxCutoutsV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id })
    }
}
impl CommandWriter for XxCutoutsV1ConfigureEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface =
            Self::writer_event_interface(state, self.object_id, "xx_cutouts_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XxCutouts(XxCutoutsObjectInteface::XxCutoutsV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        Ok(())
    }
}
