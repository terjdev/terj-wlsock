use crate::protocol::ext::dmabuf::ObjectInteface as DmabufObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_linux_buffer_params_v1.set_sampling_device`.
///
/// Opcode 4; available since version 6.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_sampling_device" since="6">
///       <description summary="set the target device of the wl_buffer">
///         Set the device the compositor should import the dmabufs to for sampling
///         in the next create or create_immed request.
///
///         To avoid race conditions when the compositor removes a device from the
///         tranches, it is not a protocol error if the device hasn't been advertised
///         by the compositor in a tranche with the sampling flag, but the import is
///         likely to fail in that case.
///
///         If the client doesn't know a suitable target device, it shouldn't set one,
///         and the compositor should attempt import on all devices it supports.
///
///         If the array is too small to contain a dev_t or larger than required, the
///         invalid_dev_t_size error will be emitted.
///       </description>
///       <arg name="device" type="array" summary="device dev_t value" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpLinuxBufferParamsV1SetSamplingDeviceRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// device dev_t value.
    pub device: Vec<u8>,
}
impl ZwpLinuxBufferParamsV1SetSamplingDeviceRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 4;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 6;
}
impl CommandReader<Self> for ZwpLinuxBufferParamsV1SetSamplingDeviceRequest {
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
impl InterfaceReader for ZwpLinuxBufferParamsV1SetSamplingDeviceRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Dmabuf(DmabufObjectInteface::ZwpLinuxBufferParamsV1 { .. })
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
impl CommandWriter for ZwpLinuxBufferParamsV1SetSamplingDeviceRequest {
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
        Self::writer_request_interface(
            state,
            self.object_id,
            "zwp_linux_buffer_params_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_array(&self.device).await?;
        Ok(())
    }
}
