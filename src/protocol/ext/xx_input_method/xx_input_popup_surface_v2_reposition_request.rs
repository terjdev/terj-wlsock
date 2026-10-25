use super::ObjectInteface as XxInputMethodObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_input_popup_surface_v2.reposition`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="reposition">
///       <description summary="recalculate the popup's location">
///         Reposition an already-mapped popup. The popup will be placed given the
///         details in the passed input_popup_positioner object.
///
///         The request is processed immediately, without the need to issue wl_surface.commit, but the actual repositioning takes place later, after .ack_configure.
///
///         The compositor should reply with a configure sequence including:
///         - input_popup_surface.start_configure,
///         - input_popup_surface.repositioned, including the token passed in this request.
///
///         This will discard any parameters set by the previous positioner.
///
///         If multiple .reposition requests are sent before the .repositioned event is submitted as part of a configure sequence, the compositor may ignore all
///         but the last one.
///
///         The new popup position will not take
///         effect until the corresponding configure sequence is acknowledged by the
///         client. See input_popup_surface.repositioned for details.
///
///         The token itself is opaque, and has no other special meaning.
///       </description>
///       <arg name="positioner" type="object" interface="xx_input_popup_positioner_v1" />
///       <arg name="token" type="uint" summary="reposition request token" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxInputPopupSurfaceV2RepositionRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `positioner`.
    pub positioner: u32,
    /// reposition request token.
    pub token: u32,
}
impl XxInputPopupSurfaceV2RepositionRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxInputPopupSurfaceV2RepositionRequest {
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
impl InterfaceReader for XxInputPopupSurfaceV2RepositionRequest {
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
        let positioner = reader.next_u32().await?;
        let token = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            positioner,
            token,
        })
    }
}
impl CommandWriter for XxInputPopupSurfaceV2RepositionRequest {
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
        writer.next_u32(self.positioner).await?;
        writer.next_u32(self.token).await?;
        Ok(())
    }
}
