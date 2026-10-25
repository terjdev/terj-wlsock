use super::ObjectInteface as WlrForeignToplevelManagementObjectInteface;
use super::{
    ZwlrForeignToplevelHandleV1ActivateRequest, ZwlrForeignToplevelHandleV1CloseRequest,
    ZwlrForeignToplevelHandleV1DestroyRequest, ZwlrForeignToplevelHandleV1SetFullscreenRequest,
    ZwlrForeignToplevelHandleV1SetMaximizedRequest, ZwlrForeignToplevelHandleV1SetMinimizedRequest,
    ZwlrForeignToplevelHandleV1SetRectangleRequest,
    ZwlrForeignToplevelHandleV1UnsetFullscreenRequest,
    ZwlrForeignToplevelHandleV1UnsetMaximizedRequest,
    ZwlrForeignToplevelHandleV1UnsetMinimizedRequest, ZwlrForeignToplevelManagerV1StopRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches foreign toplevel management requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwlrForeignToplevelManagerV1StopRequest].
    ZwlrForeignToplevelManagerV1Stop(ZwlrForeignToplevelManagerV1StopRequest),
    /// Message [ZwlrForeignToplevelHandleV1SetMaximizedRequest].
    ZwlrForeignToplevelHandleV1SetMaximized(ZwlrForeignToplevelHandleV1SetMaximizedRequest),
    /// Message [ZwlrForeignToplevelHandleV1UnsetMaximizedRequest].
    ZwlrForeignToplevelHandleV1UnsetMaximized(ZwlrForeignToplevelHandleV1UnsetMaximizedRequest),
    /// Message [ZwlrForeignToplevelHandleV1SetMinimizedRequest].
    ZwlrForeignToplevelHandleV1SetMinimized(ZwlrForeignToplevelHandleV1SetMinimizedRequest),
    /// Message [ZwlrForeignToplevelHandleV1UnsetMinimizedRequest].
    ZwlrForeignToplevelHandleV1UnsetMinimized(ZwlrForeignToplevelHandleV1UnsetMinimizedRequest),
    /// Message [ZwlrForeignToplevelHandleV1ActivateRequest].
    ZwlrForeignToplevelHandleV1Activate(ZwlrForeignToplevelHandleV1ActivateRequest),
    /// Message [ZwlrForeignToplevelHandleV1CloseRequest].
    ZwlrForeignToplevelHandleV1Close(ZwlrForeignToplevelHandleV1CloseRequest),
    /// Message [ZwlrForeignToplevelHandleV1SetRectangleRequest].
    ZwlrForeignToplevelHandleV1SetRectangle(ZwlrForeignToplevelHandleV1SetRectangleRequest),
    /// Message [ZwlrForeignToplevelHandleV1DestroyRequest].
    ZwlrForeignToplevelHandleV1Destroy(ZwlrForeignToplevelHandleV1DestroyRequest),
    /// Message [ZwlrForeignToplevelHandleV1SetFullscreenRequest].
    ZwlrForeignToplevelHandleV1SetFullscreen(ZwlrForeignToplevelHandleV1SetFullscreenRequest),
    /// Message [ZwlrForeignToplevelHandleV1UnsetFullscreenRequest].
    ZwlrForeignToplevelHandleV1UnsetFullscreen(ZwlrForeignToplevelHandleV1UnsetFullscreenRequest),
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
            ObjectInteface::WlrForeignToplevelManagement(
                WlrForeignToplevelManagementObjectInteface::ZwlrForeignToplevelManagerV1 { .. },
            ) => match opcode {
                ZwlrForeignToplevelManagerV1StopRequest::OPCODE => {
                    Ok(Self::ZwlrForeignToplevelManagerV1Stop(
                        ZwlrForeignToplevelManagerV1StopRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::WlrForeignToplevelManagement(
                WlrForeignToplevelManagementObjectInteface::ZwlrForeignToplevelHandleV1 { .. },
            ) => match opcode {
                ZwlrForeignToplevelHandleV1SetMaximizedRequest::OPCODE => {
                    Ok(Self::ZwlrForeignToplevelHandleV1SetMaximized(
                        ZwlrForeignToplevelHandleV1SetMaximizedRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrForeignToplevelHandleV1UnsetMaximizedRequest::OPCODE => {
                    Ok(Self::ZwlrForeignToplevelHandleV1UnsetMaximized(
                        ZwlrForeignToplevelHandleV1UnsetMaximizedRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrForeignToplevelHandleV1SetMinimizedRequest::OPCODE => {
                    Ok(Self::ZwlrForeignToplevelHandleV1SetMinimized(
                        ZwlrForeignToplevelHandleV1SetMinimizedRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrForeignToplevelHandleV1UnsetMinimizedRequest::OPCODE => {
                    Ok(Self::ZwlrForeignToplevelHandleV1UnsetMinimized(
                        ZwlrForeignToplevelHandleV1UnsetMinimizedRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrForeignToplevelHandleV1ActivateRequest::OPCODE => {
                    Ok(Self::ZwlrForeignToplevelHandleV1Activate(
                        ZwlrForeignToplevelHandleV1ActivateRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrForeignToplevelHandleV1CloseRequest::OPCODE => {
                    Ok(Self::ZwlrForeignToplevelHandleV1Close(
                        ZwlrForeignToplevelHandleV1CloseRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrForeignToplevelHandleV1SetRectangleRequest::OPCODE => {
                    Ok(Self::ZwlrForeignToplevelHandleV1SetRectangle(
                        ZwlrForeignToplevelHandleV1SetRectangleRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrForeignToplevelHandleV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwlrForeignToplevelHandleV1Destroy(
                        ZwlrForeignToplevelHandleV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrForeignToplevelHandleV1SetFullscreenRequest::OPCODE => {
                    Ok(Self::ZwlrForeignToplevelHandleV1SetFullscreen(
                        ZwlrForeignToplevelHandleV1SetFullscreenRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrForeignToplevelHandleV1UnsetFullscreenRequest::OPCODE => {
                    Ok(Self::ZwlrForeignToplevelHandleV1UnsetFullscreen(
                        ZwlrForeignToplevelHandleV1UnsetFullscreenRequest::read_with_interface(
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
            Self::ZwlrForeignToplevelManagerV1Stop(m) => m.object_id(),
            Self::ZwlrForeignToplevelHandleV1SetMaximized(m) => m.object_id(),
            Self::ZwlrForeignToplevelHandleV1UnsetMaximized(m) => m.object_id(),
            Self::ZwlrForeignToplevelHandleV1SetMinimized(m) => m.object_id(),
            Self::ZwlrForeignToplevelHandleV1UnsetMinimized(m) => m.object_id(),
            Self::ZwlrForeignToplevelHandleV1Activate(m) => m.object_id(),
            Self::ZwlrForeignToplevelHandleV1Close(m) => m.object_id(),
            Self::ZwlrForeignToplevelHandleV1SetRectangle(m) => m.object_id(),
            Self::ZwlrForeignToplevelHandleV1Destroy(m) => m.object_id(),
            Self::ZwlrForeignToplevelHandleV1SetFullscreen(m) => m.object_id(),
            Self::ZwlrForeignToplevelHandleV1UnsetFullscreen(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwlrForeignToplevelManagerV1Stop(m) => m.opcode(),
            Self::ZwlrForeignToplevelHandleV1SetMaximized(m) => m.opcode(),
            Self::ZwlrForeignToplevelHandleV1UnsetMaximized(m) => m.opcode(),
            Self::ZwlrForeignToplevelHandleV1SetMinimized(m) => m.opcode(),
            Self::ZwlrForeignToplevelHandleV1UnsetMinimized(m) => m.opcode(),
            Self::ZwlrForeignToplevelHandleV1Activate(m) => m.opcode(),
            Self::ZwlrForeignToplevelHandleV1Close(m) => m.opcode(),
            Self::ZwlrForeignToplevelHandleV1SetRectangle(m) => m.opcode(),
            Self::ZwlrForeignToplevelHandleV1Destroy(m) => m.opcode(),
            Self::ZwlrForeignToplevelHandleV1SetFullscreen(m) => m.opcode(),
            Self::ZwlrForeignToplevelHandleV1UnsetFullscreen(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwlrForeignToplevelManagerV1Stop(m) => m.payload_size(),
            Self::ZwlrForeignToplevelHandleV1SetMaximized(m) => m.payload_size(),
            Self::ZwlrForeignToplevelHandleV1UnsetMaximized(m) => m.payload_size(),
            Self::ZwlrForeignToplevelHandleV1SetMinimized(m) => m.payload_size(),
            Self::ZwlrForeignToplevelHandleV1UnsetMinimized(m) => m.payload_size(),
            Self::ZwlrForeignToplevelHandleV1Activate(m) => m.payload_size(),
            Self::ZwlrForeignToplevelHandleV1Close(m) => m.payload_size(),
            Self::ZwlrForeignToplevelHandleV1SetRectangle(m) => m.payload_size(),
            Self::ZwlrForeignToplevelHandleV1Destroy(m) => m.payload_size(),
            Self::ZwlrForeignToplevelHandleV1SetFullscreen(m) => m.payload_size(),
            Self::ZwlrForeignToplevelHandleV1UnsetFullscreen(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwlrForeignToplevelManagerV1Stop(m) => m.write(writer, state).await,
            Self::ZwlrForeignToplevelHandleV1SetMaximized(m) => m.write(writer, state).await,
            Self::ZwlrForeignToplevelHandleV1UnsetMaximized(m) => m.write(writer, state).await,
            Self::ZwlrForeignToplevelHandleV1SetMinimized(m) => m.write(writer, state).await,
            Self::ZwlrForeignToplevelHandleV1UnsetMinimized(m) => m.write(writer, state).await,
            Self::ZwlrForeignToplevelHandleV1Activate(m) => m.write(writer, state).await,
            Self::ZwlrForeignToplevelHandleV1Close(m) => m.write(writer, state).await,
            Self::ZwlrForeignToplevelHandleV1SetRectangle(m) => m.write(writer, state).await,
            Self::ZwlrForeignToplevelHandleV1Destroy(m) => m.write(writer, state).await,
            Self::ZwlrForeignToplevelHandleV1SetFullscreen(m) => m.write(writer, state).await,
            Self::ZwlrForeignToplevelHandleV1UnsetFullscreen(m) => m.write(writer, state).await,
        }
    }
}
