use super::ObjectInteface as WlrExportDmabufObjectInteface;
use super::ZwlrExportDmabufFrameV1CancelReason;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_export_dmabuf_frame_v1.cancel`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="cancel">
///       <description summary="indicates the frame is no longer valid">
///         If the capture failed or if the frame is no longer valid after the
///         "frame" event has been emitted, this event will be used to inform the
///         client to scrap the frame.
///
///         If the failure is temporary, the client may capture again the same
///         source. If the failure is permanent, any further attempts to capture the
///         same source will fail again.
///
///         After receiving this event, the client should destroy this object.
///       </description>
///       <arg name="reason" type="uint" enum="cancel_reason" summary="indicates a reason for cancelling this frame capture" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrExportDmabufFrameV1CancelEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// indicates a reason for cancelling this frame capture.
    pub reason: ZwlrExportDmabufFrameV1CancelReason,
}
impl ZwlrExportDmabufFrameV1CancelEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrExportDmabufFrameV1CancelEvent {
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
impl InterfaceReader for ZwlrExportDmabufFrameV1CancelEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrExportDmabuf(
                WlrExportDmabufObjectInteface::ZwlrExportDmabufFrameV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let reason = ZwlrExportDmabufFrameV1CancelReason(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, reason })
    }
}
impl CommandWriter for ZwlrExportDmabufFrameV1CancelEvent {
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
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "zwlr_export_dmabuf_frame_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::WlrExportDmabuf(
                WlrExportDmabufObjectInteface::ZwlrExportDmabufFrameV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.reason.0).await?;
        Ok(())
    }
}
