use super::ObjectInteface as WlrOutputPowerManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_output_power_v1.failed`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="failed">
///       <description summary="object no longer valid">
///         This event indicates that the output power management mode control
///         is no longer valid. This can happen for a number of reasons,
///         including:
///         - The output doesn't support power management
///         - Another client already has exclusive power management mode control
///           for this output
///         - The output disappeared
///
///         Upon receiving this event, the client should destroy this object.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrOutputPowerV1FailedEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl ZwlrOutputPowerV1FailedEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrOutputPowerV1FailedEvent {
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
impl InterfaceReader for ZwlrOutputPowerV1FailedEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrOutputPowerManagement(
                WlrOutputPowerManagementObjectInteface::ZwlrOutputPowerV1 { .. }
            )
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
impl CommandWriter for ZwlrOutputPowerV1FailedEvent {
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
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "zwlr_output_power_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::WlrOutputPowerManagement(
                WlrOutputPowerManagementObjectInteface::ZwlrOutputPowerV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        Ok(())
    }
}
