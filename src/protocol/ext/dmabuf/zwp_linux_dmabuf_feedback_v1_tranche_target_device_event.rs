use crate::protocol::ext::dmabuf::ObjectInteface as DmabufObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_linux_dmabuf_feedback_v1.tranche_target_device`.
///
/// Opcode 4; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="tranche_target_device">
///       <description summary="target device">
///         This event advertises the target device that the server prefers to use
///         for a buffer created given this tranche. The advertised target device
///         may be different for each preference tranche, and may change over time.
///
///         There is exactly one target device per tranche.
///
///         The target device may be a scan-out device, for example if the
///         compositor prefers to directly scan-out a buffer created given this
///         tranche. The target device may be a rendering device, for example if
///         the compositor prefers to texture from said buffer.
///
///         The client can use this hint to allocate the buffer in a way that makes
///         it accessible from the target device, ideally directly. The buffer must
///         still be accessible from a device with the sampling flag, either through
///         direct import or a potentially more expensive fallback path. If the
///         buffer can't be directly imported for sampling, then clients must be
///         prepared for the compositor changing the tranche priority or making
///         wl_buffer creation fail (see the zwp_linux_buffer_params_v1.create and
///         create_immed requests for details).
///
///         If the device is a DRM node, the DRM node type (primary vs. render) is
///         unspecified. Clients must not rely on the compositor sending a
///         particular node type. Clients cannot check two devices for equality by
///         comparing the dev_t value.
///
///         This event is tied to a preference tranche, see the tranche_done event.
///       </description>
///       <arg name="device" type="array" summary="device dev_t value" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpLinuxDmabufFeedbackV1TrancheTargetDeviceEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// device dev_t value.
    pub device: Vec<u8>,
}
impl ZwpLinuxDmabufFeedbackV1TrancheTargetDeviceEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 4;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpLinuxDmabufFeedbackV1TrancheTargetDeviceEvent {
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
impl InterfaceReader for ZwpLinuxDmabufFeedbackV1TrancheTargetDeviceEvent {
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
impl CommandWriter for ZwpLinuxDmabufFeedbackV1TrancheTargetDeviceEvent {
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
        Self::writer_event_interface(
            state,
            self.object_id,
            "zwp_linux_dmabuf_feedback_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_array(&self.device).await?;
        Ok(())
    }
}
