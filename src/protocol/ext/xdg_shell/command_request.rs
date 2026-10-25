use super::{
    XdgPopupDestroyRequest, XdgPopupGrabRequest, XdgPopupRepositionRequest,
    XdgPositionerDestroyRequest, XdgPositionerSetAnchorRectRequest, XdgPositionerSetAnchorRequest,
    XdgPositionerSetConstraintAdjustmentRequest, XdgPositionerSetGravityRequest,
    XdgPositionerSetOffsetRequest, XdgPositionerSetParentConfigureRequest,
    XdgPositionerSetParentSizeRequest, XdgPositionerSetReactiveRequest,
    XdgPositionerSetSizeRequest, XdgSurfaceAckConfigureRequest, XdgSurfaceDestroyRequest,
    XdgSurfaceGetPopupRequest, XdgSurfaceGetToplevelRequest, XdgSurfaceSetWindowGeometryRequest,
    XdgToplevelDestroyRequest, XdgToplevelMoveRequest, XdgToplevelResizeRequest,
    XdgToplevelSetAppIdRequest, XdgToplevelSetFullscreenRequest, XdgToplevelSetMaxSizeRequest,
    XdgToplevelSetMaximizedRequest, XdgToplevelSetMinSizeRequest, XdgToplevelSetMinimizedRequest,
    XdgToplevelSetParentRequest, XdgToplevelSetTitleRequest, XdgToplevelShowWindowMenuRequest,
    XdgToplevelUnsetFullscreenRequest, XdgToplevelUnsetMaximizedRequest,
    XdgWmBaseCreatePositionerRequest, XdgWmBaseDestroyRequest, XdgWmBaseGetXdgSurfaceRequest,
    XdgWmBasePongRequest,
};
use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches XDG shell requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [XdgWmBaseDestroyRequest].
    XdgWmBaseDestroy(XdgWmBaseDestroyRequest),
    /// Message [XdgWmBaseCreatePositionerRequest].
    XdgWmBaseCreatePositioner(XdgWmBaseCreatePositionerRequest),
    /// Message [XdgWmBaseGetXdgSurfaceRequest].
    XdgWmBaseGetXdgSurface(XdgWmBaseGetXdgSurfaceRequest),
    /// Message [XdgWmBasePongRequest].
    XdgWmBasePong(XdgWmBasePongRequest),
    /// Message [XdgPositionerDestroyRequest].
    XdgPositionerDestroy(XdgPositionerDestroyRequest),
    /// Message [XdgPositionerSetSizeRequest].
    XdgPositionerSetSize(XdgPositionerSetSizeRequest),
    /// Message [XdgPositionerSetAnchorRectRequest].
    XdgPositionerSetAnchorRect(XdgPositionerSetAnchorRectRequest),
    /// Message [XdgPositionerSetAnchorRequest].
    XdgPositionerSetAnchor(XdgPositionerSetAnchorRequest),
    /// Message [XdgPositionerSetGravityRequest].
    XdgPositionerSetGravity(XdgPositionerSetGravityRequest),
    /// Message [XdgPositionerSetConstraintAdjustmentRequest].
    XdgPositionerSetConstraintAdjustment(XdgPositionerSetConstraintAdjustmentRequest),
    /// Message [XdgPositionerSetOffsetRequest].
    XdgPositionerSetOffset(XdgPositionerSetOffsetRequest),
    /// Message [XdgPositionerSetReactiveRequest].
    XdgPositionerSetReactive(XdgPositionerSetReactiveRequest),
    /// Message [XdgPositionerSetParentSizeRequest].
    XdgPositionerSetParentSize(XdgPositionerSetParentSizeRequest),
    /// Message [XdgPositionerSetParentConfigureRequest].
    XdgPositionerSetParentConfigure(XdgPositionerSetParentConfigureRequest),
    /// Message [XdgSurfaceDestroyRequest].
    XdgSurfaceDestroy(XdgSurfaceDestroyRequest),
    /// Message [XdgSurfaceGetToplevelRequest].
    XdgSurfaceGetToplevel(XdgSurfaceGetToplevelRequest),
    /// Message [XdgSurfaceGetPopupRequest].
    XdgSurfaceGetPopup(XdgSurfaceGetPopupRequest),
    /// Message [XdgSurfaceSetWindowGeometryRequest].
    XdgSurfaceSetWindowGeometry(XdgSurfaceSetWindowGeometryRequest),
    /// Message [XdgSurfaceAckConfigureRequest].
    XdgSurfaceAckConfigure(XdgSurfaceAckConfigureRequest),
    /// Message [XdgToplevelDestroyRequest].
    XdgToplevelDestroy(XdgToplevelDestroyRequest),
    /// Message [XdgToplevelSetParentRequest].
    XdgToplevelSetParent(XdgToplevelSetParentRequest),
    /// Message [XdgToplevelSetTitleRequest].
    XdgToplevelSetTitle(XdgToplevelSetTitleRequest),
    /// Message [XdgToplevelSetAppIdRequest].
    XdgToplevelSetAppId(XdgToplevelSetAppIdRequest),
    /// Message [XdgToplevelShowWindowMenuRequest].
    XdgToplevelShowWindowMenu(XdgToplevelShowWindowMenuRequest),
    /// Message [XdgToplevelMoveRequest].
    XdgToplevelMove(XdgToplevelMoveRequest),
    /// Message [XdgToplevelResizeRequest].
    XdgToplevelResize(XdgToplevelResizeRequest),
    /// Message [XdgToplevelSetMaxSizeRequest].
    XdgToplevelSetMaxSize(XdgToplevelSetMaxSizeRequest),
    /// Message [XdgToplevelSetMinSizeRequest].
    XdgToplevelSetMinSize(XdgToplevelSetMinSizeRequest),
    /// Message [XdgToplevelSetMaximizedRequest].
    XdgToplevelSetMaximized(XdgToplevelSetMaximizedRequest),
    /// Message [XdgToplevelUnsetMaximizedRequest].
    XdgToplevelUnsetMaximized(XdgToplevelUnsetMaximizedRequest),
    /// Message [XdgToplevelSetFullscreenRequest].
    XdgToplevelSetFullscreen(XdgToplevelSetFullscreenRequest),
    /// Message [XdgToplevelUnsetFullscreenRequest].
    XdgToplevelUnsetFullscreen(XdgToplevelUnsetFullscreenRequest),
    /// Message [XdgToplevelSetMinimizedRequest].
    XdgToplevelSetMinimized(XdgToplevelSetMinimizedRequest),
    /// Message [XdgPopupDestroyRequest].
    XdgPopupDestroy(XdgPopupDestroyRequest),
    /// Message [XdgPopupGrabRequest].
    XdgPopupGrab(XdgPopupGrabRequest),
    /// Message [XdgPopupRepositionRequest].
    XdgPopupReposition(XdgPopupRepositionRequest),
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
        match interface {
            ObjectInteface::XdgShell(XdgShellObjectInteface::XdgWmBase { .. }) => match opcode {
                XdgWmBaseDestroyRequest::OPCODE => Ok(Self::XdgWmBaseDestroy(
                    XdgWmBaseDestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgWmBaseCreatePositionerRequest::OPCODE => Ok(Self::XdgWmBaseCreatePositioner(
                    XdgWmBaseCreatePositionerRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgWmBaseGetXdgSurfaceRequest::OPCODE => Ok(Self::XdgWmBaseGetXdgSurface(
                    XdgWmBaseGetXdgSurfaceRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgWmBasePongRequest::OPCODE => Ok(Self::XdgWmBasePong(
                    XdgWmBasePongRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XdgShell(XdgShellObjectInteface::XdgPositioner { .. }) => {
                match opcode {
                    XdgPositionerDestroyRequest::OPCODE => Ok(Self::XdgPositionerDestroy(
                        XdgPositionerDestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XdgPositionerSetSizeRequest::OPCODE => Ok(Self::XdgPositionerSetSize(
                        XdgPositionerSetSizeRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XdgPositionerSetAnchorRectRequest::OPCODE => {
                        Ok(Self::XdgPositionerSetAnchorRect(
                            XdgPositionerSetAnchorRectRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    XdgPositionerSetAnchorRequest::OPCODE => Ok(Self::XdgPositionerSetAnchor(
                        XdgPositionerSetAnchorRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XdgPositionerSetGravityRequest::OPCODE => Ok(Self::XdgPositionerSetGravity(
                        XdgPositionerSetGravityRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XdgPositionerSetConstraintAdjustmentRequest::OPCODE => {
                        Ok(Self::XdgPositionerSetConstraintAdjustment(
                            XdgPositionerSetConstraintAdjustmentRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    XdgPositionerSetOffsetRequest::OPCODE => Ok(Self::XdgPositionerSetOffset(
                        XdgPositionerSetOffsetRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XdgPositionerSetReactiveRequest::OPCODE => Ok(Self::XdgPositionerSetReactive(
                        XdgPositionerSetReactiveRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XdgPositionerSetParentSizeRequest::OPCODE => {
                        Ok(Self::XdgPositionerSetParentSize(
                            XdgPositionerSetParentSizeRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    XdgPositionerSetParentConfigureRequest::OPCODE => {
                        Ok(Self::XdgPositionerSetParentConfigure(
                            XdgPositionerSetParentConfigureRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::XdgShell(XdgShellObjectInteface::XdgSurface { .. }) => match opcode {
                XdgSurfaceDestroyRequest::OPCODE => Ok(Self::XdgSurfaceDestroy(
                    XdgSurfaceDestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgSurfaceGetToplevelRequest::OPCODE => Ok(Self::XdgSurfaceGetToplevel(
                    XdgSurfaceGetToplevelRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgSurfaceGetPopupRequest::OPCODE => Ok(Self::XdgSurfaceGetPopup(
                    XdgSurfaceGetPopupRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgSurfaceSetWindowGeometryRequest::OPCODE => {
                    Ok(Self::XdgSurfaceSetWindowGeometry(
                        XdgSurfaceSetWindowGeometryRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XdgSurfaceAckConfigureRequest::OPCODE => Ok(Self::XdgSurfaceAckConfigure(
                    XdgSurfaceAckConfigureRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XdgShell(XdgShellObjectInteface::XdgToplevel { .. }) => match opcode {
                XdgToplevelDestroyRequest::OPCODE => Ok(Self::XdgToplevelDestroy(
                    XdgToplevelDestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgToplevelSetParentRequest::OPCODE => Ok(Self::XdgToplevelSetParent(
                    XdgToplevelSetParentRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgToplevelSetTitleRequest::OPCODE => Ok(Self::XdgToplevelSetTitle(
                    XdgToplevelSetTitleRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgToplevelSetAppIdRequest::OPCODE => Ok(Self::XdgToplevelSetAppId(
                    XdgToplevelSetAppIdRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgToplevelShowWindowMenuRequest::OPCODE => Ok(Self::XdgToplevelShowWindowMenu(
                    XdgToplevelShowWindowMenuRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgToplevelMoveRequest::OPCODE => Ok(Self::XdgToplevelMove(
                    XdgToplevelMoveRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgToplevelResizeRequest::OPCODE => Ok(Self::XdgToplevelResize(
                    XdgToplevelResizeRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgToplevelSetMaxSizeRequest::OPCODE => Ok(Self::XdgToplevelSetMaxSize(
                    XdgToplevelSetMaxSizeRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgToplevelSetMinSizeRequest::OPCODE => Ok(Self::XdgToplevelSetMinSize(
                    XdgToplevelSetMinSizeRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgToplevelSetMaximizedRequest::OPCODE => Ok(Self::XdgToplevelSetMaximized(
                    XdgToplevelSetMaximizedRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgToplevelUnsetMaximizedRequest::OPCODE => Ok(Self::XdgToplevelUnsetMaximized(
                    XdgToplevelUnsetMaximizedRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgToplevelSetFullscreenRequest::OPCODE => Ok(Self::XdgToplevelSetFullscreen(
                    XdgToplevelSetFullscreenRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgToplevelUnsetFullscreenRequest::OPCODE => Ok(Self::XdgToplevelUnsetFullscreen(
                    XdgToplevelUnsetFullscreenRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgToplevelSetMinimizedRequest::OPCODE => Ok(Self::XdgToplevelSetMinimized(
                    XdgToplevelSetMinimizedRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XdgShell(XdgShellObjectInteface::XdgPopup { .. }) => match opcode {
                XdgPopupDestroyRequest::OPCODE => Ok(Self::XdgPopupDestroy(
                    XdgPopupDestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgPopupGrabRequest::OPCODE => Ok(Self::XdgPopupGrab(
                    XdgPopupGrabRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgPopupRepositionRequest::OPCODE => Ok(Self::XdgPopupReposition(
                    XdgPopupRepositionRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            _ => Err(FdReaderError::InvalidProtocol),
        }
    }
}
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::XdgWmBaseDestroy(message) => message.object_id(),
            Self::XdgWmBaseCreatePositioner(message) => message.object_id(),
            Self::XdgWmBaseGetXdgSurface(message) => message.object_id(),
            Self::XdgWmBasePong(message) => message.object_id(),
            Self::XdgPositionerDestroy(message) => message.object_id(),
            Self::XdgPositionerSetSize(message) => message.object_id(),
            Self::XdgPositionerSetAnchorRect(message) => message.object_id(),
            Self::XdgPositionerSetAnchor(message) => message.object_id(),
            Self::XdgPositionerSetGravity(message) => message.object_id(),
            Self::XdgPositionerSetConstraintAdjustment(message) => message.object_id(),
            Self::XdgPositionerSetOffset(message) => message.object_id(),
            Self::XdgPositionerSetReactive(message) => message.object_id(),
            Self::XdgPositionerSetParentSize(message) => message.object_id(),
            Self::XdgPositionerSetParentConfigure(message) => message.object_id(),
            Self::XdgSurfaceDestroy(message) => message.object_id(),
            Self::XdgSurfaceGetToplevel(message) => message.object_id(),
            Self::XdgSurfaceGetPopup(message) => message.object_id(),
            Self::XdgSurfaceSetWindowGeometry(message) => message.object_id(),
            Self::XdgSurfaceAckConfigure(message) => message.object_id(),
            Self::XdgToplevelDestroy(message) => message.object_id(),
            Self::XdgToplevelSetParent(message) => message.object_id(),
            Self::XdgToplevelSetTitle(message) => message.object_id(),
            Self::XdgToplevelSetAppId(message) => message.object_id(),
            Self::XdgToplevelShowWindowMenu(message) => message.object_id(),
            Self::XdgToplevelMove(message) => message.object_id(),
            Self::XdgToplevelResize(message) => message.object_id(),
            Self::XdgToplevelSetMaxSize(message) => message.object_id(),
            Self::XdgToplevelSetMinSize(message) => message.object_id(),
            Self::XdgToplevelSetMaximized(message) => message.object_id(),
            Self::XdgToplevelUnsetMaximized(message) => message.object_id(),
            Self::XdgToplevelSetFullscreen(message) => message.object_id(),
            Self::XdgToplevelUnsetFullscreen(message) => message.object_id(),
            Self::XdgToplevelSetMinimized(message) => message.object_id(),
            Self::XdgPopupDestroy(message) => message.object_id(),
            Self::XdgPopupGrab(message) => message.object_id(),
            Self::XdgPopupReposition(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XdgWmBaseDestroy(message) => message.opcode(),
            Self::XdgWmBaseCreatePositioner(message) => message.opcode(),
            Self::XdgWmBaseGetXdgSurface(message) => message.opcode(),
            Self::XdgWmBasePong(message) => message.opcode(),
            Self::XdgPositionerDestroy(message) => message.opcode(),
            Self::XdgPositionerSetSize(message) => message.opcode(),
            Self::XdgPositionerSetAnchorRect(message) => message.opcode(),
            Self::XdgPositionerSetAnchor(message) => message.opcode(),
            Self::XdgPositionerSetGravity(message) => message.opcode(),
            Self::XdgPositionerSetConstraintAdjustment(message) => message.opcode(),
            Self::XdgPositionerSetOffset(message) => message.opcode(),
            Self::XdgPositionerSetReactive(message) => message.opcode(),
            Self::XdgPositionerSetParentSize(message) => message.opcode(),
            Self::XdgPositionerSetParentConfigure(message) => message.opcode(),
            Self::XdgSurfaceDestroy(message) => message.opcode(),
            Self::XdgSurfaceGetToplevel(message) => message.opcode(),
            Self::XdgSurfaceGetPopup(message) => message.opcode(),
            Self::XdgSurfaceSetWindowGeometry(message) => message.opcode(),
            Self::XdgSurfaceAckConfigure(message) => message.opcode(),
            Self::XdgToplevelDestroy(message) => message.opcode(),
            Self::XdgToplevelSetParent(message) => message.opcode(),
            Self::XdgToplevelSetTitle(message) => message.opcode(),
            Self::XdgToplevelSetAppId(message) => message.opcode(),
            Self::XdgToplevelShowWindowMenu(message) => message.opcode(),
            Self::XdgToplevelMove(message) => message.opcode(),
            Self::XdgToplevelResize(message) => message.opcode(),
            Self::XdgToplevelSetMaxSize(message) => message.opcode(),
            Self::XdgToplevelSetMinSize(message) => message.opcode(),
            Self::XdgToplevelSetMaximized(message) => message.opcode(),
            Self::XdgToplevelUnsetMaximized(message) => message.opcode(),
            Self::XdgToplevelSetFullscreen(message) => message.opcode(),
            Self::XdgToplevelUnsetFullscreen(message) => message.opcode(),
            Self::XdgToplevelSetMinimized(message) => message.opcode(),
            Self::XdgPopupDestroy(message) => message.opcode(),
            Self::XdgPopupGrab(message) => message.opcode(),
            Self::XdgPopupReposition(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XdgWmBaseDestroy(message) => message.payload_size(),
            Self::XdgWmBaseCreatePositioner(message) => message.payload_size(),
            Self::XdgWmBaseGetXdgSurface(message) => message.payload_size(),
            Self::XdgWmBasePong(message) => message.payload_size(),
            Self::XdgPositionerDestroy(message) => message.payload_size(),
            Self::XdgPositionerSetSize(message) => message.payload_size(),
            Self::XdgPositionerSetAnchorRect(message) => message.payload_size(),
            Self::XdgPositionerSetAnchor(message) => message.payload_size(),
            Self::XdgPositionerSetGravity(message) => message.payload_size(),
            Self::XdgPositionerSetConstraintAdjustment(message) => message.payload_size(),
            Self::XdgPositionerSetOffset(message) => message.payload_size(),
            Self::XdgPositionerSetReactive(message) => message.payload_size(),
            Self::XdgPositionerSetParentSize(message) => message.payload_size(),
            Self::XdgPositionerSetParentConfigure(message) => message.payload_size(),
            Self::XdgSurfaceDestroy(message) => message.payload_size(),
            Self::XdgSurfaceGetToplevel(message) => message.payload_size(),
            Self::XdgSurfaceGetPopup(message) => message.payload_size(),
            Self::XdgSurfaceSetWindowGeometry(message) => message.payload_size(),
            Self::XdgSurfaceAckConfigure(message) => message.payload_size(),
            Self::XdgToplevelDestroy(message) => message.payload_size(),
            Self::XdgToplevelSetParent(message) => message.payload_size(),
            Self::XdgToplevelSetTitle(message) => message.payload_size(),
            Self::XdgToplevelSetAppId(message) => message.payload_size(),
            Self::XdgToplevelShowWindowMenu(message) => message.payload_size(),
            Self::XdgToplevelMove(message) => message.payload_size(),
            Self::XdgToplevelResize(message) => message.payload_size(),
            Self::XdgToplevelSetMaxSize(message) => message.payload_size(),
            Self::XdgToplevelSetMinSize(message) => message.payload_size(),
            Self::XdgToplevelSetMaximized(message) => message.payload_size(),
            Self::XdgToplevelUnsetMaximized(message) => message.payload_size(),
            Self::XdgToplevelSetFullscreen(message) => message.payload_size(),
            Self::XdgToplevelUnsetFullscreen(message) => message.payload_size(),
            Self::XdgToplevelSetMinimized(message) => message.payload_size(),
            Self::XdgPopupDestroy(message) => message.payload_size(),
            Self::XdgPopupGrab(message) => message.payload_size(),
            Self::XdgPopupReposition(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XdgWmBaseDestroy(message) => message.write(writer, state).await,
            Self::XdgWmBaseCreatePositioner(message) => message.write(writer, state).await,
            Self::XdgWmBaseGetXdgSurface(message) => message.write(writer, state).await,
            Self::XdgWmBasePong(message) => message.write(writer, state).await,
            Self::XdgPositionerDestroy(message) => message.write(writer, state).await,
            Self::XdgPositionerSetSize(message) => message.write(writer, state).await,
            Self::XdgPositionerSetAnchorRect(message) => message.write(writer, state).await,
            Self::XdgPositionerSetAnchor(message) => message.write(writer, state).await,
            Self::XdgPositionerSetGravity(message) => message.write(writer, state).await,
            Self::XdgPositionerSetConstraintAdjustment(message) => {
                message.write(writer, state).await
            }
            Self::XdgPositionerSetOffset(message) => message.write(writer, state).await,
            Self::XdgPositionerSetReactive(message) => message.write(writer, state).await,
            Self::XdgPositionerSetParentSize(message) => message.write(writer, state).await,
            Self::XdgPositionerSetParentConfigure(message) => message.write(writer, state).await,
            Self::XdgSurfaceDestroy(message) => message.write(writer, state).await,
            Self::XdgSurfaceGetToplevel(message) => message.write(writer, state).await,
            Self::XdgSurfaceGetPopup(message) => message.write(writer, state).await,
            Self::XdgSurfaceSetWindowGeometry(message) => message.write(writer, state).await,
            Self::XdgSurfaceAckConfigure(message) => message.write(writer, state).await,
            Self::XdgToplevelDestroy(message) => message.write(writer, state).await,
            Self::XdgToplevelSetParent(message) => message.write(writer, state).await,
            Self::XdgToplevelSetTitle(message) => message.write(writer, state).await,
            Self::XdgToplevelSetAppId(message) => message.write(writer, state).await,
            Self::XdgToplevelShowWindowMenu(message) => message.write(writer, state).await,
            Self::XdgToplevelMove(message) => message.write(writer, state).await,
            Self::XdgToplevelResize(message) => message.write(writer, state).await,
            Self::XdgToplevelSetMaxSize(message) => message.write(writer, state).await,
            Self::XdgToplevelSetMinSize(message) => message.write(writer, state).await,
            Self::XdgToplevelSetMaximized(message) => message.write(writer, state).await,
            Self::XdgToplevelUnsetMaximized(message) => message.write(writer, state).await,
            Self::XdgToplevelSetFullscreen(message) => message.write(writer, state).await,
            Self::XdgToplevelUnsetFullscreen(message) => message.write(writer, state).await,
            Self::XdgToplevelSetMinimized(message) => message.write(writer, state).await,
            Self::XdgPopupDestroy(message) => message.write(writer, state).await,
            Self::XdgPopupGrab(message) => message.write(writer, state).await,
            Self::XdgPopupReposition(message) => message.write(writer, state).await,
        }
    }
}
