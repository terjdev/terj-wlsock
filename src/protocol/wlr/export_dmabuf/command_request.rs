use super::ObjectInteface as WlrExportDmabufObjectInteface;
use super::{
    ZwlrExportDmabufFrameV1DestroyRequest, ZwlrExportDmabufManagerV1CaptureOutputRequest,
    ZwlrExportDmabufManagerV1DestroyRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches export dmabuf requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwlrExportDmabufManagerV1CaptureOutputRequest].
    ZwlrExportDmabufManagerV1CaptureOutput(ZwlrExportDmabufManagerV1CaptureOutputRequest),
    /// Message [ZwlrExportDmabufManagerV1DestroyRequest].
    ZwlrExportDmabufManagerV1Destroy(ZwlrExportDmabufManagerV1DestroyRequest),
    /// Message [ZwlrExportDmabufFrameV1DestroyRequest].
    ZwlrExportDmabufFrameV1Destroy(ZwlrExportDmabufFrameV1DestroyRequest),
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
            ObjectInteface::WlrExportDmabuf(
                WlrExportDmabufObjectInteface::ZwlrExportDmabufManagerV1 { .. },
            ) => match opcode {
                ZwlrExportDmabufManagerV1CaptureOutputRequest::OPCODE => {
                    Ok(Self::ZwlrExportDmabufManagerV1CaptureOutput(
                        ZwlrExportDmabufManagerV1CaptureOutputRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrExportDmabufManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwlrExportDmabufManagerV1Destroy(
                        ZwlrExportDmabufManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::WlrExportDmabuf(
                WlrExportDmabufObjectInteface::ZwlrExportDmabufFrameV1 { .. },
            ) => match opcode {
                ZwlrExportDmabufFrameV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwlrExportDmabufFrameV1Destroy(
                        ZwlrExportDmabufFrameV1DestroyRequest::read_with_interface(
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
            Self::ZwlrExportDmabufManagerV1CaptureOutput(m) => m.object_id(),
            Self::ZwlrExportDmabufManagerV1Destroy(m) => m.object_id(),
            Self::ZwlrExportDmabufFrameV1Destroy(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwlrExportDmabufManagerV1CaptureOutput(m) => m.opcode(),
            Self::ZwlrExportDmabufManagerV1Destroy(m) => m.opcode(),
            Self::ZwlrExportDmabufFrameV1Destroy(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwlrExportDmabufManagerV1CaptureOutput(m) => m.payload_size(),
            Self::ZwlrExportDmabufManagerV1Destroy(m) => m.payload_size(),
            Self::ZwlrExportDmabufFrameV1Destroy(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwlrExportDmabufManagerV1CaptureOutput(m) => m.write(writer, state).await,
            Self::ZwlrExportDmabufManagerV1Destroy(m) => m.write(writer, state).await,
            Self::ZwlrExportDmabufFrameV1Destroy(m) => m.write(writer, state).await,
        }
    }
}
