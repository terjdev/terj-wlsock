use super::ObjectInteface as PrimarySelectionObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_primary_selection_device_v1.set_selection`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_selection">
///       <description summary="set the primary selection">
///         Replaces the current selection. The previous owner of the primary
///         selection will receive a wp_primary_selection_source.cancelled event.
///
///         To unset the selection, set the source to NULL.
///       </description>
///       <arg name="source" type="object" interface="zwp_primary_selection_source_v1" allow-null="true" />
///       <arg name="serial" type="uint" summary="serial of the event that triggered this request" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpPrimarySelectionDeviceV1SetSelectionRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `source`.
    pub source: Option<u32>,
    /// serial of the event that triggered this request.
    pub serial: u32,
}
impl ZwpPrimarySelectionDeviceV1SetSelectionRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpPrimarySelectionDeviceV1SetSelectionRequest {
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
impl InterfaceReader for ZwpPrimarySelectionDeviceV1SetSelectionRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::PrimarySelection(
                PrimarySelectionObjectInteface::ZwpPrimarySelectionDeviceV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let source = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        let serial = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            source,
            serial,
        })
    }
}
impl CommandWriter for ZwpPrimarySelectionDeviceV1SetSelectionRequest {
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
            "zwp_primary_selection_device_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::PrimarySelection(
                PrimarySelectionObjectInteface::ZwpPrimarySelectionDeviceV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.source.unwrap_or(0)).await?;
        writer.next_u32(self.serial).await?;
        Ok(())
    }
}
