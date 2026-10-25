use super::ObjectInteface as FullscreenShellObjectInteface;
use super::{
    ZwpFullscreenShellV1PresentSurfaceForModeRequest, ZwpFullscreenShellV1PresentSurfaceRequest,
    ZwpFullscreenShellV1ReleaseRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches fullscreen shell requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwpFullscreenShellV1ReleaseRequest].
    ZwpFullscreenShellV1Release(ZwpFullscreenShellV1ReleaseRequest),
    /// Message [ZwpFullscreenShellV1PresentSurfaceRequest].
    ZwpFullscreenShellV1PresentSurface(ZwpFullscreenShellV1PresentSurfaceRequest),
    /// Message [ZwpFullscreenShellV1PresentSurfaceForModeRequest].
    ZwpFullscreenShellV1PresentSurfaceForMode(ZwpFullscreenShellV1PresentSurfaceForModeRequest),
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
            ObjectInteface::FullscreenShell(
                FullscreenShellObjectInteface::ZwpFullscreenShellV1 { .. },
            ) => match opcode {
                ZwpFullscreenShellV1ReleaseRequest::OPCODE => {
                    Ok(Self::ZwpFullscreenShellV1Release(
                        ZwpFullscreenShellV1ReleaseRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpFullscreenShellV1PresentSurfaceRequest::OPCODE => {
                    Ok(Self::ZwpFullscreenShellV1PresentSurface(
                        ZwpFullscreenShellV1PresentSurfaceRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpFullscreenShellV1PresentSurfaceForModeRequest::OPCODE => {
                    Ok(Self::ZwpFullscreenShellV1PresentSurfaceForMode(
                        ZwpFullscreenShellV1PresentSurfaceForModeRequest::read_with_interface(
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
            Self::ZwpFullscreenShellV1Release(m) => m.object_id(),
            Self::ZwpFullscreenShellV1PresentSurface(m) => m.object_id(),
            Self::ZwpFullscreenShellV1PresentSurfaceForMode(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpFullscreenShellV1Release(m) => m.opcode(),
            Self::ZwpFullscreenShellV1PresentSurface(m) => m.opcode(),
            Self::ZwpFullscreenShellV1PresentSurfaceForMode(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpFullscreenShellV1Release(m) => m.payload_size(),
            Self::ZwpFullscreenShellV1PresentSurface(m) => m.payload_size(),
            Self::ZwpFullscreenShellV1PresentSurfaceForMode(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpFullscreenShellV1Release(m) => m.write(writer, state).await,
            Self::ZwpFullscreenShellV1PresentSurface(m) => m.write(writer, state).await,
            Self::ZwpFullscreenShellV1PresentSurfaceForMode(m) => m.write(writer, state).await,
        }
    }
}
