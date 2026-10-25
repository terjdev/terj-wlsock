use super::ObjectInteface as XxInputMethodObjectInteface;
use super::XxInputPopupPositionerV1Anchor;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_input_popup_positioner_v1.set_anchor`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_anchor">
///       <description summary="set anchor rectangle anchor">
///         Defines the anchor point for the anchor rectangle. The specified anchor
///         is used to derive an anchor point that the popup surface will be
///         positioned relative to. If a corner anchor is set (e.g. 'top_left' or
///         'bottom_right'), the anchor point will be at the specified corner;
///         otherwise, the derived anchor point will be centered on the specified
///         edge, or in the center of the anchor rectangle if no edge is specified.
///       </description>
///       <arg name="anchor" type="uint" enum="anchor" summary="anchor" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxInputPopupPositionerV1SetAnchorRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// anchor.
    pub anchor: XxInputPopupPositionerV1Anchor,
}
impl XxInputPopupPositionerV1SetAnchorRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxInputPopupPositionerV1SetAnchorRequest {
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
impl InterfaceReader for XxInputPopupPositionerV1SetAnchorRequest {
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
                XxInputMethodObjectInteface::XxInputPopupPositionerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let anchor = XxInputPopupPositionerV1Anchor(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, anchor })
    }
}
impl CommandWriter for XxInputPopupPositionerV1SetAnchorRequest {
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
            "xx_input_popup_positioner_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::XxInputMethod(
                XxInputMethodObjectInteface::XxInputPopupPositionerV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.anchor.0).await?;
        Ok(())
    }
}
