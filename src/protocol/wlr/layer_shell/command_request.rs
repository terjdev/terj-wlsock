use super::{
    ZwlrLayerShellV1DestroyRequest, ZwlrLayerShellV1GetLayerSurfaceRequest,
    ZwlrLayerSurfaceV1AckConfigureRequest, ZwlrLayerSurfaceV1DestroyRequest,
    ZwlrLayerSurfaceV1GetPopupRequest, ZwlrLayerSurfaceV1SetAnchorRequest,
    ZwlrLayerSurfaceV1SetExclusiveEdgeRequest, ZwlrLayerSurfaceV1SetExclusiveZoneRequest,
    ZwlrLayerSurfaceV1SetKeyboardInteractivityRequest, ZwlrLayerSurfaceV1SetLayerRequest,
    ZwlrLayerSurfaceV1SetMarginRequest, ZwlrLayerSurfaceV1SetSizeRequest,
};
use crate::protocol::wlr::layer_shell::ObjectInteface as WlrLayerShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches WLR layer shell requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwlrLayerShellV1GetLayerSurfaceRequest].
    ZwlrLayerShellV1GetLayerSurface(ZwlrLayerShellV1GetLayerSurfaceRequest),
    /// Message [ZwlrLayerShellV1DestroyRequest].
    ZwlrLayerShellV1Destroy(ZwlrLayerShellV1DestroyRequest),
    /// Message [ZwlrLayerSurfaceV1SetSizeRequest].
    ZwlrLayerSurfaceV1SetSize(ZwlrLayerSurfaceV1SetSizeRequest),
    /// Message [ZwlrLayerSurfaceV1SetAnchorRequest].
    ZwlrLayerSurfaceV1SetAnchor(ZwlrLayerSurfaceV1SetAnchorRequest),
    /// Message [ZwlrLayerSurfaceV1SetExclusiveZoneRequest].
    ZwlrLayerSurfaceV1SetExclusiveZone(ZwlrLayerSurfaceV1SetExclusiveZoneRequest),
    /// Message [ZwlrLayerSurfaceV1SetMarginRequest].
    ZwlrLayerSurfaceV1SetMargin(ZwlrLayerSurfaceV1SetMarginRequest),
    /// Message [ZwlrLayerSurfaceV1SetKeyboardInteractivityRequest].
    ZwlrLayerSurfaceV1SetKeyboardInteractivity(ZwlrLayerSurfaceV1SetKeyboardInteractivityRequest),
    /// Message [ZwlrLayerSurfaceV1GetPopupRequest].
    ZwlrLayerSurfaceV1GetPopup(ZwlrLayerSurfaceV1GetPopupRequest),
    /// Message [ZwlrLayerSurfaceV1AckConfigureRequest].
    ZwlrLayerSurfaceV1AckConfigure(ZwlrLayerSurfaceV1AckConfigureRequest),
    /// Message [ZwlrLayerSurfaceV1DestroyRequest].
    ZwlrLayerSurfaceV1Destroy(ZwlrLayerSurfaceV1DestroyRequest),
    /// Message [ZwlrLayerSurfaceV1SetLayerRequest].
    ZwlrLayerSurfaceV1SetLayer(ZwlrLayerSurfaceV1SetLayerRequest),
    /// Message [ZwlrLayerSurfaceV1SetExclusiveEdgeRequest].
    ZwlrLayerSurfaceV1SetExclusiveEdge(ZwlrLayerSurfaceV1SetExclusiveEdgeRequest),
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
            ObjectInteface::WlrLayerShell(WlrLayerShellObjectInteface::ZwlrLayerShellV1 {
                ..
            }) => match opcode {
                ZwlrLayerShellV1GetLayerSurfaceRequest::OPCODE => {
                    Ok(Self::ZwlrLayerShellV1GetLayerSurface(
                        ZwlrLayerShellV1GetLayerSurfaceRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrLayerShellV1DestroyRequest::OPCODE => Ok(Self::ZwlrLayerShellV1Destroy(
                    ZwlrLayerShellV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::WlrLayerShell(WlrLayerShellObjectInteface::ZwlrLayerSurfaceV1 {
                ..
            }) => match opcode {
                ZwlrLayerSurfaceV1SetSizeRequest::OPCODE => Ok(Self::ZwlrLayerSurfaceV1SetSize(
                    ZwlrLayerSurfaceV1SetSizeRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrLayerSurfaceV1SetAnchorRequest::OPCODE => {
                    Ok(Self::ZwlrLayerSurfaceV1SetAnchor(
                        ZwlrLayerSurfaceV1SetAnchorRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrLayerSurfaceV1SetExclusiveZoneRequest::OPCODE => {
                    Ok(Self::ZwlrLayerSurfaceV1SetExclusiveZone(
                        ZwlrLayerSurfaceV1SetExclusiveZoneRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrLayerSurfaceV1SetMarginRequest::OPCODE => {
                    Ok(Self::ZwlrLayerSurfaceV1SetMargin(
                        ZwlrLayerSurfaceV1SetMarginRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrLayerSurfaceV1SetKeyboardInteractivityRequest::OPCODE => {
                    Ok(Self::ZwlrLayerSurfaceV1SetKeyboardInteractivity(
                        ZwlrLayerSurfaceV1SetKeyboardInteractivityRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrLayerSurfaceV1GetPopupRequest::OPCODE => Ok(Self::ZwlrLayerSurfaceV1GetPopup(
                    ZwlrLayerSurfaceV1GetPopupRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrLayerSurfaceV1AckConfigureRequest::OPCODE => {
                    Ok(Self::ZwlrLayerSurfaceV1AckConfigure(
                        ZwlrLayerSurfaceV1AckConfigureRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrLayerSurfaceV1DestroyRequest::OPCODE => Ok(Self::ZwlrLayerSurfaceV1Destroy(
                    ZwlrLayerSurfaceV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrLayerSurfaceV1SetLayerRequest::OPCODE => Ok(Self::ZwlrLayerSurfaceV1SetLayer(
                    ZwlrLayerSurfaceV1SetLayerRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrLayerSurfaceV1SetExclusiveEdgeRequest::OPCODE => {
                    Ok(Self::ZwlrLayerSurfaceV1SetExclusiveEdge(
                        ZwlrLayerSurfaceV1SetExclusiveEdgeRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            _ => Err(FdReaderError::InvalidProtocol),
        }
    }
}
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::ZwlrLayerShellV1GetLayerSurface(message) => message.object_id(),
            Self::ZwlrLayerShellV1Destroy(message) => message.object_id(),
            Self::ZwlrLayerSurfaceV1SetSize(message) => message.object_id(),
            Self::ZwlrLayerSurfaceV1SetAnchor(message) => message.object_id(),
            Self::ZwlrLayerSurfaceV1SetExclusiveZone(message) => message.object_id(),
            Self::ZwlrLayerSurfaceV1SetMargin(message) => message.object_id(),
            Self::ZwlrLayerSurfaceV1SetKeyboardInteractivity(message) => message.object_id(),
            Self::ZwlrLayerSurfaceV1GetPopup(message) => message.object_id(),
            Self::ZwlrLayerSurfaceV1AckConfigure(message) => message.object_id(),
            Self::ZwlrLayerSurfaceV1Destroy(message) => message.object_id(),
            Self::ZwlrLayerSurfaceV1SetLayer(message) => message.object_id(),
            Self::ZwlrLayerSurfaceV1SetExclusiveEdge(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwlrLayerShellV1GetLayerSurface(message) => message.opcode(),
            Self::ZwlrLayerShellV1Destroy(message) => message.opcode(),
            Self::ZwlrLayerSurfaceV1SetSize(message) => message.opcode(),
            Self::ZwlrLayerSurfaceV1SetAnchor(message) => message.opcode(),
            Self::ZwlrLayerSurfaceV1SetExclusiveZone(message) => message.opcode(),
            Self::ZwlrLayerSurfaceV1SetMargin(message) => message.opcode(),
            Self::ZwlrLayerSurfaceV1SetKeyboardInteractivity(message) => message.opcode(),
            Self::ZwlrLayerSurfaceV1GetPopup(message) => message.opcode(),
            Self::ZwlrLayerSurfaceV1AckConfigure(message) => message.opcode(),
            Self::ZwlrLayerSurfaceV1Destroy(message) => message.opcode(),
            Self::ZwlrLayerSurfaceV1SetLayer(message) => message.opcode(),
            Self::ZwlrLayerSurfaceV1SetExclusiveEdge(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwlrLayerShellV1GetLayerSurface(message) => message.payload_size(),
            Self::ZwlrLayerShellV1Destroy(message) => message.payload_size(),
            Self::ZwlrLayerSurfaceV1SetSize(message) => message.payload_size(),
            Self::ZwlrLayerSurfaceV1SetAnchor(message) => message.payload_size(),
            Self::ZwlrLayerSurfaceV1SetExclusiveZone(message) => message.payload_size(),
            Self::ZwlrLayerSurfaceV1SetMargin(message) => message.payload_size(),
            Self::ZwlrLayerSurfaceV1SetKeyboardInteractivity(message) => message.payload_size(),
            Self::ZwlrLayerSurfaceV1GetPopup(message) => message.payload_size(),
            Self::ZwlrLayerSurfaceV1AckConfigure(message) => message.payload_size(),
            Self::ZwlrLayerSurfaceV1Destroy(message) => message.payload_size(),
            Self::ZwlrLayerSurfaceV1SetLayer(message) => message.payload_size(),
            Self::ZwlrLayerSurfaceV1SetExclusiveEdge(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwlrLayerShellV1GetLayerSurface(message) => message.write(writer, state).await,
            Self::ZwlrLayerShellV1Destroy(message) => message.write(writer, state).await,
            Self::ZwlrLayerSurfaceV1SetSize(message) => message.write(writer, state).await,
            Self::ZwlrLayerSurfaceV1SetAnchor(message) => message.write(writer, state).await,
            Self::ZwlrLayerSurfaceV1SetExclusiveZone(message) => message.write(writer, state).await,
            Self::ZwlrLayerSurfaceV1SetMargin(message) => message.write(writer, state).await,
            Self::ZwlrLayerSurfaceV1SetKeyboardInteractivity(message) => {
                message.write(writer, state).await
            }
            Self::ZwlrLayerSurfaceV1GetPopup(message) => message.write(writer, state).await,
            Self::ZwlrLayerSurfaceV1AckConfigure(message) => message.write(writer, state).await,
            Self::ZwlrLayerSurfaceV1Destroy(message) => message.write(writer, state).await,
            Self::ZwlrLayerSurfaceV1SetLayer(message) => message.write(writer, state).await,
            Self::ZwlrLayerSurfaceV1SetExclusiveEdge(message) => message.write(writer, state).await,
        }
    }
}
