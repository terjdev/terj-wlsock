use super::{
    ZwlrScreencopyFrameV1CopyRequest, ZwlrScreencopyFrameV1CopyWithDamageRequest,
    ZwlrScreencopyFrameV1DestroyRequest, ZwlrScreencopyManagerV1CaptureOutputRegionRequest,
    ZwlrScreencopyManagerV1CaptureOutputRequest, ZwlrScreencopyManagerV1DestroyRequest,
};
use crate::protocol::wlr::screencopy::ObjectInteface as WlrScreencopyObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches WLR screencopy requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwlrScreencopyManagerV1CaptureOutputRequest].
    ZwlrScreencopyManagerV1CaptureOutput(ZwlrScreencopyManagerV1CaptureOutputRequest),
    /// Message [ZwlrScreencopyManagerV1CaptureOutputRegionRequest].
    ZwlrScreencopyManagerV1CaptureOutputRegion(ZwlrScreencopyManagerV1CaptureOutputRegionRequest),
    /// Message [ZwlrScreencopyManagerV1DestroyRequest].
    ZwlrScreencopyManagerV1Destroy(ZwlrScreencopyManagerV1DestroyRequest),
    /// Message [ZwlrScreencopyFrameV1CopyRequest].
    ZwlrScreencopyFrameV1Copy(ZwlrScreencopyFrameV1CopyRequest),
    /// Message [ZwlrScreencopyFrameV1DestroyRequest].
    ZwlrScreencopyFrameV1Destroy(ZwlrScreencopyFrameV1DestroyRequest),
    /// Message [ZwlrScreencopyFrameV1CopyWithDamageRequest].
    ZwlrScreencopyFrameV1CopyWithDamage(ZwlrScreencopyFrameV1CopyWithDamageRequest),
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
            ObjectInteface::WlrScreencopy(
                WlrScreencopyObjectInteface::ZwlrScreencopyManagerV1 { .. },
            ) => match opcode {
                ZwlrScreencopyManagerV1CaptureOutputRequest::OPCODE => {
                    Ok(Self::ZwlrScreencopyManagerV1CaptureOutput(
                        ZwlrScreencopyManagerV1CaptureOutputRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrScreencopyManagerV1CaptureOutputRegionRequest::OPCODE => {
                    Ok(Self::ZwlrScreencopyManagerV1CaptureOutputRegion(
                        ZwlrScreencopyManagerV1CaptureOutputRegionRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrScreencopyManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwlrScreencopyManagerV1Destroy(
                        ZwlrScreencopyManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::WlrScreencopy(WlrScreencopyObjectInteface::ZwlrScreencopyFrameV1 {
                ..
            }) => match opcode {
                ZwlrScreencopyFrameV1CopyRequest::OPCODE => Ok(Self::ZwlrScreencopyFrameV1Copy(
                    ZwlrScreencopyFrameV1CopyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrScreencopyFrameV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwlrScreencopyFrameV1Destroy(
                        ZwlrScreencopyFrameV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrScreencopyFrameV1CopyWithDamageRequest::OPCODE => {
                    Ok(Self::ZwlrScreencopyFrameV1CopyWithDamage(
                        ZwlrScreencopyFrameV1CopyWithDamageRequest::read_with_interface(
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
            Self::ZwlrScreencopyManagerV1CaptureOutput(message) => message.object_id(),
            Self::ZwlrScreencopyManagerV1CaptureOutputRegion(message) => message.object_id(),
            Self::ZwlrScreencopyManagerV1Destroy(message) => message.object_id(),
            Self::ZwlrScreencopyFrameV1Copy(message) => message.object_id(),
            Self::ZwlrScreencopyFrameV1Destroy(message) => message.object_id(),
            Self::ZwlrScreencopyFrameV1CopyWithDamage(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwlrScreencopyManagerV1CaptureOutput(message) => message.opcode(),
            Self::ZwlrScreencopyManagerV1CaptureOutputRegion(message) => message.opcode(),
            Self::ZwlrScreencopyManagerV1Destroy(message) => message.opcode(),
            Self::ZwlrScreencopyFrameV1Copy(message) => message.opcode(),
            Self::ZwlrScreencopyFrameV1Destroy(message) => message.opcode(),
            Self::ZwlrScreencopyFrameV1CopyWithDamage(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwlrScreencopyManagerV1CaptureOutput(message) => message.payload_size(),
            Self::ZwlrScreencopyManagerV1CaptureOutputRegion(message) => message.payload_size(),
            Self::ZwlrScreencopyManagerV1Destroy(message) => message.payload_size(),
            Self::ZwlrScreencopyFrameV1Copy(message) => message.payload_size(),
            Self::ZwlrScreencopyFrameV1Destroy(message) => message.payload_size(),
            Self::ZwlrScreencopyFrameV1CopyWithDamage(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwlrScreencopyManagerV1CaptureOutput(message) => {
                message.write(writer, state).await
            }
            Self::ZwlrScreencopyManagerV1CaptureOutputRegion(message) => {
                message.write(writer, state).await
            }
            Self::ZwlrScreencopyManagerV1Destroy(message) => message.write(writer, state).await,
            Self::ZwlrScreencopyFrameV1Copy(message) => message.write(writer, state).await,
            Self::ZwlrScreencopyFrameV1Destroy(message) => message.write(writer, state).await,
            Self::ZwlrScreencopyFrameV1CopyWithDamage(message) => {
                message.write(writer, state).await
            }
        }
    }
}
