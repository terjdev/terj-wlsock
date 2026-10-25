use super::{
    ZwpLinuxBufferParamsV1AddRequest, ZwpLinuxBufferParamsV1CreateImmedRequest,
    ZwpLinuxBufferParamsV1CreateRequest, ZwpLinuxBufferParamsV1DestroyRequest,
    ZwpLinuxBufferParamsV1SetSamplingDeviceRequest, ZwpLinuxDmabufFeedbackV1DestroyRequest,
    ZwpLinuxDmabufV1CreateParamsRequest, ZwpLinuxDmabufV1DestroyRequest,
    ZwpLinuxDmabufV1GetDefaultFeedbackRequest, ZwpLinuxDmabufV1GetSurfaceFeedbackRequest,
};
use crate::protocol::ext::dmabuf::ObjectInteface as DmabufObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Linux DMA-BUF requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwpLinuxDmabufV1DestroyRequest].
    ZwpLinuxDmabufV1Destroy(ZwpLinuxDmabufV1DestroyRequest),
    /// Message [ZwpLinuxDmabufV1CreateParamsRequest].
    ZwpLinuxDmabufV1CreateParams(ZwpLinuxDmabufV1CreateParamsRequest),
    /// Message [ZwpLinuxDmabufV1GetDefaultFeedbackRequest].
    ZwpLinuxDmabufV1GetDefaultFeedback(ZwpLinuxDmabufV1GetDefaultFeedbackRequest),
    /// Message [ZwpLinuxDmabufV1GetSurfaceFeedbackRequest].
    ZwpLinuxDmabufV1GetSurfaceFeedback(ZwpLinuxDmabufV1GetSurfaceFeedbackRequest),
    /// Message [ZwpLinuxBufferParamsV1DestroyRequest].
    ZwpLinuxBufferParamsV1Destroy(ZwpLinuxBufferParamsV1DestroyRequest),
    /// Message [ZwpLinuxBufferParamsV1AddRequest].
    ZwpLinuxBufferParamsV1Add(ZwpLinuxBufferParamsV1AddRequest),
    /// Message [ZwpLinuxBufferParamsV1CreateRequest].
    ZwpLinuxBufferParamsV1Create(ZwpLinuxBufferParamsV1CreateRequest),
    /// Message [ZwpLinuxBufferParamsV1CreateImmedRequest].
    ZwpLinuxBufferParamsV1CreateImmed(ZwpLinuxBufferParamsV1CreateImmedRequest),
    /// Message [ZwpLinuxBufferParamsV1SetSamplingDeviceRequest].
    ZwpLinuxBufferParamsV1SetSamplingDevice(ZwpLinuxBufferParamsV1SetSamplingDeviceRequest),
    /// Message [ZwpLinuxDmabufFeedbackV1DestroyRequest].
    ZwpLinuxDmabufFeedbackV1Destroy(ZwpLinuxDmabufFeedbackV1DestroyRequest),
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
            ObjectInteface::Dmabuf(DmabufObjectInteface::ZwpLinuxDmabufV1 { .. }) => match opcode {
                ZwpLinuxDmabufV1DestroyRequest::OPCODE => Ok(Self::ZwpLinuxDmabufV1Destroy(
                    ZwpLinuxDmabufV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpLinuxDmabufV1CreateParamsRequest::OPCODE => {
                    Ok(Self::ZwpLinuxDmabufV1CreateParams(
                        ZwpLinuxDmabufV1CreateParamsRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpLinuxDmabufV1GetDefaultFeedbackRequest::OPCODE => {
                    Ok(Self::ZwpLinuxDmabufV1GetDefaultFeedback(
                        ZwpLinuxDmabufV1GetDefaultFeedbackRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpLinuxDmabufV1GetSurfaceFeedbackRequest::OPCODE => {
                    Ok(Self::ZwpLinuxDmabufV1GetSurfaceFeedback(
                        ZwpLinuxDmabufV1GetSurfaceFeedbackRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Dmabuf(DmabufObjectInteface::ZwpLinuxBufferParamsV1 { .. }) => {
                match opcode {
                    ZwpLinuxBufferParamsV1DestroyRequest::OPCODE => {
                        Ok(Self::ZwpLinuxBufferParamsV1Destroy(
                            ZwpLinuxBufferParamsV1DestroyRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpLinuxBufferParamsV1AddRequest::OPCODE => {
                        Ok(Self::ZwpLinuxBufferParamsV1Add(
                            ZwpLinuxBufferParamsV1AddRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpLinuxBufferParamsV1CreateRequest::OPCODE => {
                        Ok(Self::ZwpLinuxBufferParamsV1Create(
                            ZwpLinuxBufferParamsV1CreateRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpLinuxBufferParamsV1CreateImmedRequest::OPCODE => {
                        Ok(Self::ZwpLinuxBufferParamsV1CreateImmed(
                            ZwpLinuxBufferParamsV1CreateImmedRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpLinuxBufferParamsV1SetSamplingDeviceRequest::OPCODE => {
                        Ok(Self::ZwpLinuxBufferParamsV1SetSamplingDevice(
                            ZwpLinuxBufferParamsV1SetSamplingDeviceRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::Dmabuf(DmabufObjectInteface::ZwpLinuxDmabufFeedbackV1 { .. }) => {
                match opcode {
                    ZwpLinuxDmabufFeedbackV1DestroyRequest::OPCODE => {
                        Ok(Self::ZwpLinuxDmabufFeedbackV1Destroy(
                            ZwpLinuxDmabufFeedbackV1DestroyRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            _ => Err(FdReaderError::InvalidProtocol),
        }
    }
}
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::ZwpLinuxDmabufV1Destroy(message) => message.object_id(),
            Self::ZwpLinuxDmabufV1CreateParams(message) => message.object_id(),
            Self::ZwpLinuxDmabufV1GetDefaultFeedback(message) => message.object_id(),
            Self::ZwpLinuxDmabufV1GetSurfaceFeedback(message) => message.object_id(),
            Self::ZwpLinuxBufferParamsV1Destroy(message) => message.object_id(),
            Self::ZwpLinuxBufferParamsV1Add(message) => message.object_id(),
            Self::ZwpLinuxBufferParamsV1Create(message) => message.object_id(),
            Self::ZwpLinuxBufferParamsV1CreateImmed(message) => message.object_id(),
            Self::ZwpLinuxBufferParamsV1SetSamplingDevice(message) => message.object_id(),
            Self::ZwpLinuxDmabufFeedbackV1Destroy(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpLinuxDmabufV1Destroy(message) => message.opcode(),
            Self::ZwpLinuxDmabufV1CreateParams(message) => message.opcode(),
            Self::ZwpLinuxDmabufV1GetDefaultFeedback(message) => message.opcode(),
            Self::ZwpLinuxDmabufV1GetSurfaceFeedback(message) => message.opcode(),
            Self::ZwpLinuxBufferParamsV1Destroy(message) => message.opcode(),
            Self::ZwpLinuxBufferParamsV1Add(message) => message.opcode(),
            Self::ZwpLinuxBufferParamsV1Create(message) => message.opcode(),
            Self::ZwpLinuxBufferParamsV1CreateImmed(message) => message.opcode(),
            Self::ZwpLinuxBufferParamsV1SetSamplingDevice(message) => message.opcode(),
            Self::ZwpLinuxDmabufFeedbackV1Destroy(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpLinuxDmabufV1Destroy(message) => message.payload_size(),
            Self::ZwpLinuxDmabufV1CreateParams(message) => message.payload_size(),
            Self::ZwpLinuxDmabufV1GetDefaultFeedback(message) => message.payload_size(),
            Self::ZwpLinuxDmabufV1GetSurfaceFeedback(message) => message.payload_size(),
            Self::ZwpLinuxBufferParamsV1Destroy(message) => message.payload_size(),
            Self::ZwpLinuxBufferParamsV1Add(message) => message.payload_size(),
            Self::ZwpLinuxBufferParamsV1Create(message) => message.payload_size(),
            Self::ZwpLinuxBufferParamsV1CreateImmed(message) => message.payload_size(),
            Self::ZwpLinuxBufferParamsV1SetSamplingDevice(message) => message.payload_size(),
            Self::ZwpLinuxDmabufFeedbackV1Destroy(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpLinuxDmabufV1Destroy(message) => message.write(writer, state).await,
            Self::ZwpLinuxDmabufV1CreateParams(message) => message.write(writer, state).await,
            Self::ZwpLinuxDmabufV1GetDefaultFeedback(message) => message.write(writer, state).await,
            Self::ZwpLinuxDmabufV1GetSurfaceFeedback(message) => message.write(writer, state).await,
            Self::ZwpLinuxBufferParamsV1Destroy(message) => message.write(writer, state).await,
            Self::ZwpLinuxBufferParamsV1Add(message) => message.write(writer, state).await,
            Self::ZwpLinuxBufferParamsV1Create(message) => message.write(writer, state).await,
            Self::ZwpLinuxBufferParamsV1CreateImmed(message) => message.write(writer, state).await,
            Self::ZwpLinuxBufferParamsV1SetSamplingDevice(message) => {
                message.write(writer, state).await
            }
            Self::ZwpLinuxDmabufFeedbackV1Destroy(message) => message.write(writer, state).await,
        }
    }
}
