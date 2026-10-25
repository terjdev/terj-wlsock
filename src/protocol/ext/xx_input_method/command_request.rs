use super::ObjectInteface as XxInputMethodObjectInteface;
use super::{
    XxInputMethodManagerV2DestroyRequest, XxInputMethodManagerV2GetInputMethodRequest,
    XxInputMethodManagerV2GetPositionerRequest, XxInputMethodV1CommitRequest,
    XxInputMethodV1CommitStringRequest, XxInputMethodV1DeleteSurroundingTextRequest,
    XxInputMethodV1DestroyRequest, XxInputMethodV1GetInputPopupSurfaceRequest,
    XxInputMethodV1MoveCursorRequest, XxInputMethodV1PerformActionRequest,
    XxInputMethodV1SetPreeditStringRequest, XxInputPopupPositionerV1DestroyRequest,
    XxInputPopupPositionerV1SetAnchorRequest,
    XxInputPopupPositionerV1SetConstraintAdjustmentRequest,
    XxInputPopupPositionerV1SetGravityRequest, XxInputPopupPositionerV1SetOffsetRequest,
    XxInputPopupPositionerV1SetReactiveRequest, XxInputPopupPositionerV1SetSizeRequest,
    XxInputPopupSurfaceV2AckConfigureRequest, XxInputPopupSurfaceV2DestroyRequest,
    XxInputPopupSurfaceV2RepositionRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches xx input method requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [XxInputMethodV1PerformActionRequest].
    XxInputMethodV1PerformAction(XxInputMethodV1PerformActionRequest),
    /// Message [XxInputMethodV1CommitStringRequest].
    XxInputMethodV1CommitString(XxInputMethodV1CommitStringRequest),
    /// Message [XxInputMethodV1SetPreeditStringRequest].
    XxInputMethodV1SetPreeditString(XxInputMethodV1SetPreeditStringRequest),
    /// Message [XxInputMethodV1DeleteSurroundingTextRequest].
    XxInputMethodV1DeleteSurroundingText(XxInputMethodV1DeleteSurroundingTextRequest),
    /// Message [XxInputMethodV1MoveCursorRequest].
    XxInputMethodV1MoveCursor(XxInputMethodV1MoveCursorRequest),
    /// Message [XxInputMethodV1CommitRequest].
    XxInputMethodV1Commit(XxInputMethodV1CommitRequest),
    /// Message [XxInputMethodV1GetInputPopupSurfaceRequest].
    XxInputMethodV1GetInputPopupSurface(XxInputMethodV1GetInputPopupSurfaceRequest),
    /// Message [XxInputMethodV1DestroyRequest].
    XxInputMethodV1Destroy(XxInputMethodV1DestroyRequest),
    /// Message [XxInputPopupSurfaceV2AckConfigureRequest].
    XxInputPopupSurfaceV2AckConfigure(XxInputPopupSurfaceV2AckConfigureRequest),
    /// Message [XxInputPopupSurfaceV2RepositionRequest].
    XxInputPopupSurfaceV2Reposition(XxInputPopupSurfaceV2RepositionRequest),
    /// Message [XxInputPopupSurfaceV2DestroyRequest].
    XxInputPopupSurfaceV2Destroy(XxInputPopupSurfaceV2DestroyRequest),
    /// Message [XxInputPopupPositionerV1DestroyRequest].
    XxInputPopupPositionerV1Destroy(XxInputPopupPositionerV1DestroyRequest),
    /// Message [XxInputPopupPositionerV1SetSizeRequest].
    XxInputPopupPositionerV1SetSize(XxInputPopupPositionerV1SetSizeRequest),
    /// Message [XxInputPopupPositionerV1SetAnchorRequest].
    XxInputPopupPositionerV1SetAnchor(XxInputPopupPositionerV1SetAnchorRequest),
    /// Message [XxInputPopupPositionerV1SetGravityRequest].
    XxInputPopupPositionerV1SetGravity(XxInputPopupPositionerV1SetGravityRequest),
    /// Message [XxInputPopupPositionerV1SetConstraintAdjustmentRequest].
    XxInputPopupPositionerV1SetConstraintAdjustment(
        XxInputPopupPositionerV1SetConstraintAdjustmentRequest,
    ),
    /// Message [XxInputPopupPositionerV1SetOffsetRequest].
    XxInputPopupPositionerV1SetOffset(XxInputPopupPositionerV1SetOffsetRequest),
    /// Message [XxInputPopupPositionerV1SetReactiveRequest].
    XxInputPopupPositionerV1SetReactive(XxInputPopupPositionerV1SetReactiveRequest),
    /// Message [XxInputMethodManagerV2GetInputMethodRequest].
    XxInputMethodManagerV2GetInputMethod(XxInputMethodManagerV2GetInputMethodRequest),
    /// Message [XxInputMethodManagerV2GetPositionerRequest].
    XxInputMethodManagerV2GetPositioner(XxInputMethodManagerV2GetPositionerRequest),
    /// Message [XxInputMethodManagerV2DestroyRequest].
    XxInputMethodManagerV2Destroy(XxInputMethodManagerV2DestroyRequest),
}
impl CommandReader<Self> for CommandRequest {
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
impl InterfaceReader for CommandRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        match interface{
ObjectInteface::XxInputMethod(XxInputMethodObjectInteface::XxInputMethodV1{..})=>match opcode{XxInputMethodV1PerformActionRequest::OPCODE=>Ok(Self::XxInputMethodV1PerformAction(XxInputMethodV1PerformActionRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),XxInputMethodV1CommitStringRequest::OPCODE=>Ok(Self::XxInputMethodV1CommitString(XxInputMethodV1CommitStringRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),XxInputMethodV1SetPreeditStringRequest::OPCODE=>Ok(Self::XxInputMethodV1SetPreeditString(XxInputMethodV1SetPreeditStringRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),XxInputMethodV1DeleteSurroundingTextRequest::OPCODE=>Ok(Self::XxInputMethodV1DeleteSurroundingText(XxInputMethodV1DeleteSurroundingTextRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),XxInputMethodV1MoveCursorRequest::OPCODE=>Ok(Self::XxInputMethodV1MoveCursor(XxInputMethodV1MoveCursorRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),XxInputMethodV1CommitRequest::OPCODE=>Ok(Self::XxInputMethodV1Commit(XxInputMethodV1CommitRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),XxInputMethodV1GetInputPopupSurfaceRequest::OPCODE=>Ok(Self::XxInputMethodV1GetInputPopupSurface(XxInputMethodV1GetInputPopupSurfaceRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),XxInputMethodV1DestroyRequest::OPCODE=>Ok(Self::XxInputMethodV1Destroy(XxInputMethodV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::XxInputMethod(XxInputMethodObjectInteface::XxInputPopupSurfaceV2{..})=>match opcode{XxInputPopupSurfaceV2AckConfigureRequest::OPCODE=>Ok(Self::XxInputPopupSurfaceV2AckConfigure(XxInputPopupSurfaceV2AckConfigureRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),XxInputPopupSurfaceV2RepositionRequest::OPCODE=>Ok(Self::XxInputPopupSurfaceV2Reposition(XxInputPopupSurfaceV2RepositionRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),XxInputPopupSurfaceV2DestroyRequest::OPCODE=>Ok(Self::XxInputPopupSurfaceV2Destroy(XxInputPopupSurfaceV2DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::XxInputMethod(XxInputMethodObjectInteface::XxInputPopupPositionerV1{..})=>match opcode{XxInputPopupPositionerV1DestroyRequest::OPCODE=>Ok(Self::XxInputPopupPositionerV1Destroy(XxInputPopupPositionerV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),XxInputPopupPositionerV1SetSizeRequest::OPCODE=>Ok(Self::XxInputPopupPositionerV1SetSize(XxInputPopupPositionerV1SetSizeRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),XxInputPopupPositionerV1SetAnchorRequest::OPCODE=>Ok(Self::XxInputPopupPositionerV1SetAnchor(XxInputPopupPositionerV1SetAnchorRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),XxInputPopupPositionerV1SetGravityRequest::OPCODE=>Ok(Self::XxInputPopupPositionerV1SetGravity(XxInputPopupPositionerV1SetGravityRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),XxInputPopupPositionerV1SetConstraintAdjustmentRequest::OPCODE=>Ok(Self::XxInputPopupPositionerV1SetConstraintAdjustment(XxInputPopupPositionerV1SetConstraintAdjustmentRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),XxInputPopupPositionerV1SetOffsetRequest::OPCODE=>Ok(Self::XxInputPopupPositionerV1SetOffset(XxInputPopupPositionerV1SetOffsetRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),XxInputPopupPositionerV1SetReactiveRequest::OPCODE=>Ok(Self::XxInputPopupPositionerV1SetReactive(XxInputPopupPositionerV1SetReactiveRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::XxInputMethod(XxInputMethodObjectInteface::XxInputMethodManagerV2{..})=>match opcode{XxInputMethodManagerV2GetInputMethodRequest::OPCODE=>Ok(Self::XxInputMethodManagerV2GetInputMethod(XxInputMethodManagerV2GetInputMethodRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),XxInputMethodManagerV2GetPositionerRequest::OPCODE=>Ok(Self::XxInputMethodManagerV2GetPositioner(XxInputMethodManagerV2GetPositionerRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),XxInputMethodManagerV2DestroyRequest::OPCODE=>Ok(Self::XxInputMethodManagerV2Destroy(XxInputMethodManagerV2DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),_=>Err(FdReaderError::InvalidProtocol),},
_=>Err(FdReaderError::InvalidProtocol),}
    }
}
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::XxInputMethodV1PerformAction(m) => m.object_id(),
            Self::XxInputMethodV1CommitString(m) => m.object_id(),
            Self::XxInputMethodV1SetPreeditString(m) => m.object_id(),
            Self::XxInputMethodV1DeleteSurroundingText(m) => m.object_id(),
            Self::XxInputMethodV1MoveCursor(m) => m.object_id(),
            Self::XxInputMethodV1Commit(m) => m.object_id(),
            Self::XxInputMethodV1GetInputPopupSurface(m) => m.object_id(),
            Self::XxInputMethodV1Destroy(m) => m.object_id(),
            Self::XxInputPopupSurfaceV2AckConfigure(m) => m.object_id(),
            Self::XxInputPopupSurfaceV2Reposition(m) => m.object_id(),
            Self::XxInputPopupSurfaceV2Destroy(m) => m.object_id(),
            Self::XxInputPopupPositionerV1Destroy(m) => m.object_id(),
            Self::XxInputPopupPositionerV1SetSize(m) => m.object_id(),
            Self::XxInputPopupPositionerV1SetAnchor(m) => m.object_id(),
            Self::XxInputPopupPositionerV1SetGravity(m) => m.object_id(),
            Self::XxInputPopupPositionerV1SetConstraintAdjustment(m) => m.object_id(),
            Self::XxInputPopupPositionerV1SetOffset(m) => m.object_id(),
            Self::XxInputPopupPositionerV1SetReactive(m) => m.object_id(),
            Self::XxInputMethodManagerV2GetInputMethod(m) => m.object_id(),
            Self::XxInputMethodManagerV2GetPositioner(m) => m.object_id(),
            Self::XxInputMethodManagerV2Destroy(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XxInputMethodV1PerformAction(m) => m.opcode(),
            Self::XxInputMethodV1CommitString(m) => m.opcode(),
            Self::XxInputMethodV1SetPreeditString(m) => m.opcode(),
            Self::XxInputMethodV1DeleteSurroundingText(m) => m.opcode(),
            Self::XxInputMethodV1MoveCursor(m) => m.opcode(),
            Self::XxInputMethodV1Commit(m) => m.opcode(),
            Self::XxInputMethodV1GetInputPopupSurface(m) => m.opcode(),
            Self::XxInputMethodV1Destroy(m) => m.opcode(),
            Self::XxInputPopupSurfaceV2AckConfigure(m) => m.opcode(),
            Self::XxInputPopupSurfaceV2Reposition(m) => m.opcode(),
            Self::XxInputPopupSurfaceV2Destroy(m) => m.opcode(),
            Self::XxInputPopupPositionerV1Destroy(m) => m.opcode(),
            Self::XxInputPopupPositionerV1SetSize(m) => m.opcode(),
            Self::XxInputPopupPositionerV1SetAnchor(m) => m.opcode(),
            Self::XxInputPopupPositionerV1SetGravity(m) => m.opcode(),
            Self::XxInputPopupPositionerV1SetConstraintAdjustment(m) => m.opcode(),
            Self::XxInputPopupPositionerV1SetOffset(m) => m.opcode(),
            Self::XxInputPopupPositionerV1SetReactive(m) => m.opcode(),
            Self::XxInputMethodManagerV2GetInputMethod(m) => m.opcode(),
            Self::XxInputMethodManagerV2GetPositioner(m) => m.opcode(),
            Self::XxInputMethodManagerV2Destroy(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XxInputMethodV1PerformAction(m) => m.payload_size(),
            Self::XxInputMethodV1CommitString(m) => m.payload_size(),
            Self::XxInputMethodV1SetPreeditString(m) => m.payload_size(),
            Self::XxInputMethodV1DeleteSurroundingText(m) => m.payload_size(),
            Self::XxInputMethodV1MoveCursor(m) => m.payload_size(),
            Self::XxInputMethodV1Commit(m) => m.payload_size(),
            Self::XxInputMethodV1GetInputPopupSurface(m) => m.payload_size(),
            Self::XxInputMethodV1Destroy(m) => m.payload_size(),
            Self::XxInputPopupSurfaceV2AckConfigure(m) => m.payload_size(),
            Self::XxInputPopupSurfaceV2Reposition(m) => m.payload_size(),
            Self::XxInputPopupSurfaceV2Destroy(m) => m.payload_size(),
            Self::XxInputPopupPositionerV1Destroy(m) => m.payload_size(),
            Self::XxInputPopupPositionerV1SetSize(m) => m.payload_size(),
            Self::XxInputPopupPositionerV1SetAnchor(m) => m.payload_size(),
            Self::XxInputPopupPositionerV1SetGravity(m) => m.payload_size(),
            Self::XxInputPopupPositionerV1SetConstraintAdjustment(m) => m.payload_size(),
            Self::XxInputPopupPositionerV1SetOffset(m) => m.payload_size(),
            Self::XxInputPopupPositionerV1SetReactive(m) => m.payload_size(),
            Self::XxInputMethodManagerV2GetInputMethod(m) => m.payload_size(),
            Self::XxInputMethodManagerV2GetPositioner(m) => m.payload_size(),
            Self::XxInputMethodManagerV2Destroy(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XxInputMethodV1PerformAction(m) => m.write(writer, state).await,
            Self::XxInputMethodV1CommitString(m) => m.write(writer, state).await,
            Self::XxInputMethodV1SetPreeditString(m) => m.write(writer, state).await,
            Self::XxInputMethodV1DeleteSurroundingText(m) => m.write(writer, state).await,
            Self::XxInputMethodV1MoveCursor(m) => m.write(writer, state).await,
            Self::XxInputMethodV1Commit(m) => m.write(writer, state).await,
            Self::XxInputMethodV1GetInputPopupSurface(m) => m.write(writer, state).await,
            Self::XxInputMethodV1Destroy(m) => m.write(writer, state).await,
            Self::XxInputPopupSurfaceV2AckConfigure(m) => m.write(writer, state).await,
            Self::XxInputPopupSurfaceV2Reposition(m) => m.write(writer, state).await,
            Self::XxInputPopupSurfaceV2Destroy(m) => m.write(writer, state).await,
            Self::XxInputPopupPositionerV1Destroy(m) => m.write(writer, state).await,
            Self::XxInputPopupPositionerV1SetSize(m) => m.write(writer, state).await,
            Self::XxInputPopupPositionerV1SetAnchor(m) => m.write(writer, state).await,
            Self::XxInputPopupPositionerV1SetGravity(m) => m.write(writer, state).await,
            Self::XxInputPopupPositionerV1SetConstraintAdjustment(m) => {
                m.write(writer, state).await
            }
            Self::XxInputPopupPositionerV1SetOffset(m) => m.write(writer, state).await,
            Self::XxInputPopupPositionerV1SetReactive(m) => m.write(writer, state).await,
            Self::XxInputMethodManagerV2GetInputMethod(m) => m.write(writer, state).await,
            Self::XxInputMethodManagerV2GetPositioner(m) => m.write(writer, state).await,
            Self::XxInputMethodManagerV2Destroy(m) => m.write(writer, state).await,
        }
    }
}
