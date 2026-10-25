use crate::protocol::ext::dmabuf::ObjectInteface as DmabufObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_linux_dmabuf_feedback_v1.main_device`.
///
/// Opcode 2; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// Sending or receiving this event is rejected at version 6 and above.
/// ```xml
/// <event name="main_device" deprecated-since="6">
///       <description summary="preferred main device">
///         This event advertises the main device that the server prefers to use
///         when direct scan-out to the target device isn't possible. The
///         advertised main device may be different for each
///         zwp_linux_dmabuf_feedback_v1 object, and may change over time.
///
///         There is exactly one main device. The compositor must send at least
///         one preference tranche with tranche_target_device equal to main_device.
///
///         Clients need to create buffers that the main device can import and
///         read from, otherwise creating the dmabuf wl_buffer will fail (see the
///         zwp_linux_buffer_params_v1.create and create_immed requests for details).
///         The main device will also likely be kept active by the compositor,
///         so clients can use it instead of waking up another device for power
///         savings.
///
///         In general the device is a DRM node. The DRM node type (primary vs.
///         render) is unspecified. Clients must not rely on the compositor sending
///         a particular node type. Clients cannot check two devices for equality
///         by comparing the dev_t value.
///
///         If explicit modifiers are not supported and the client performs buffer
///         allocations on a different device than the main device, then the client
///         must force the buffer to have a linear layout.
///
///         With version 6 and above, this event is no longer sent. Clients should
///         use a device with the sampling flag in the tranches instead.
///       </description>
///       <arg name="device" type="array" summary="device dev_t value" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpLinuxDmabufFeedbackV1MainDeviceEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// device dev_t value.
    pub device: Vec<u8>,
}
impl ZwpLinuxDmabufFeedbackV1MainDeviceEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
    /// First interface version prohibiting this event.
    pub const DEPRECATED_SINCE: u32 = 6;
}
impl CommandReader<Self> for ZwpLinuxDmabufFeedbackV1MainDeviceEvent {
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
impl InterfaceReader for ZwpLinuxDmabufFeedbackV1MainDeviceEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Dmabuf(DmabufObjectInteface::ZwpLinuxDmabufFeedbackV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
            || interface.version() >= Self::DEPRECATED_SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let device = reader.next_array().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, device })
    }
}
impl CommandWriter for ZwpLinuxDmabufFeedbackV1MainDeviceEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_array_size(self.device.len())])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "zwp_linux_dmabuf_feedback_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota()
            || interface.version() >= Self::DEPRECATED_SINCE
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_array(&self.device).await?;
        Ok(())
    }
}
