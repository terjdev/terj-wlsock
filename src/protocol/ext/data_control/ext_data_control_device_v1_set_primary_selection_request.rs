use crate::protocol::ext::data_control::ObjectInteface as DataControlObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `ext_data_control_device_v1.set_primary_selection`.
///
/// Opcode 2; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_primary_selection">
///       <description summary="copy data to the primary selection">
///         This request asks the compositor to set the primary selection to the
///         data from the source on behalf of the client.
///
///         The given source may not be used in any further set_selection or
///         set_primary_selection requests. Attempting to use a previously used
///         source triggers the used_source protocol error.
///
///         To unset the primary selection, set the source to NULL.
///
///         The compositor will ignore this request if it does not support primary
///         selection.
///       </description>
///       <arg name="source" type="object" interface="ext_data_control_source_v1" allow-null="true" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ExtDataControlDeviceV1SetPrimarySelectionRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `source`.
    pub source: Option<u32>,
}
impl ExtDataControlDeviceV1SetPrimarySelectionRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ExtDataControlDeviceV1SetPrimarySelectionRequest {
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
impl InterfaceReader for ExtDataControlDeviceV1SetPrimarySelectionRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::DataControl(DataControlObjectInteface::ExtDataControlDeviceV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let source = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, source })
    }
}
impl CommandWriter for ExtDataControlDeviceV1SetPrimarySelectionRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(
            state,
            self.object_id,
            "ext_data_control_device_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.source.unwrap_or(0)).await?;
        Ok(())
    }
}
