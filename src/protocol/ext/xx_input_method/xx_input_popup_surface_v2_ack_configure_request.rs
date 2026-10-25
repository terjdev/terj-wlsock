use super::ObjectInteface as XxInputMethodObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_input_popup_surface_v2.ack_configure`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="ack_configure">
///       <description summary="acknowledge a configure sequence">
///         This request notifies the compositor that the client updated its surface in response to a configure sequence.
///
///         The purpose of this request is to synchronize the updates of the surface geometry with the surface contents.
///         For example, when the compositor assigns a size larger than previously, the client must fill the additional space before the popup gets displayed to the user with the new size. When the compositor receives .ack_configure, it can proceed to draw the new size.
///
///         .ack_configure should be sent after every submitted configure sequence, passing along the serial received in it.
///
///         An .ack_configure request is conceptually double-buffered.
///         Every request overrides the previous one. The request takes effect once the .commit request is sent on the corresponding surface.
///
///         If the client receives multiple configure sequences before it
///         can respond to one, it may acknowledge only the last configure sequence by using its serial in the .ack_configure request.
///
///         Committing an .ack_configure request consumes the serial number sent with
///         the request, as well as serial numbers sent by all configure sequences
///         submitted on this input_popup_surface prior to the configure sequence referenced by
///         the committed serial.
///
///         Committing this request with a serial that, for this surface, never appeared in a submitted configure sequence, or one that was already committed before, raises an invalid_serial
///         error.
///       </description>
///       <arg name="serial" type="uint" summary="the serial from the configure sequence" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxInputPopupSurfaceV2AckConfigureRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the serial from the configure sequence.
    pub serial: u32,
}
impl XxInputPopupSurfaceV2AckConfigureRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxInputPopupSurfaceV2AckConfigureRequest {
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
impl InterfaceReader for XxInputPopupSurfaceV2AckConfigureRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxInputMethod(
                XxInputMethodObjectInteface::XxInputPopupSurfaceV2 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let serial = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, serial })
    }
}
impl CommandWriter for XxInputPopupSurfaceV2AckConfigureRequest {
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
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "xx_input_popup_surface_v2",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::XxInputMethod(
                XxInputMethodObjectInteface::XxInputPopupSurfaceV2 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        Ok(())
    }
}
