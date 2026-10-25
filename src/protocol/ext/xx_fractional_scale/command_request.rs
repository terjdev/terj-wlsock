use super::ObjectInteface as XxFractionalScaleObjectInteface;
use super::{
    XxFractionalScaleManagerV2DestroyRequest, XxFractionalScaleManagerV2GetFractionalScaleRequest,
    XxFractionalScaleV2DestroyRequest, XxFractionalScaleV2SetScaleFactorRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches xx fractional scale requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [XxFractionalScaleManagerV2DestroyRequest].
    XxFractionalScaleManagerV2Destroy(XxFractionalScaleManagerV2DestroyRequest),
    /// Message [XxFractionalScaleManagerV2GetFractionalScaleRequest].
    XxFractionalScaleManagerV2GetFractionalScale(
        XxFractionalScaleManagerV2GetFractionalScaleRequest,
    ),
    /// Message [XxFractionalScaleV2SetScaleFactorRequest].
    XxFractionalScaleV2SetScaleFactor(XxFractionalScaleV2SetScaleFactorRequest),
    /// Message [XxFractionalScaleV2DestroyRequest].
    XxFractionalScaleV2Destroy(XxFractionalScaleV2DestroyRequest),
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
            ObjectInteface::XxFractionalScale(
                XxFractionalScaleObjectInteface::XxFractionalScaleManagerV2 { .. },
            ) => match opcode {
                XxFractionalScaleManagerV2DestroyRequest::OPCODE => {
                    Ok(Self::XxFractionalScaleManagerV2Destroy(
                        XxFractionalScaleManagerV2DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XxFractionalScaleManagerV2GetFractionalScaleRequest::OPCODE => {
                    Ok(Self::XxFractionalScaleManagerV2GetFractionalScale(
                        XxFractionalScaleManagerV2GetFractionalScaleRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XxFractionalScale(
                XxFractionalScaleObjectInteface::XxFractionalScaleV2 { .. },
            ) => match opcode {
                XxFractionalScaleV2SetScaleFactorRequest::OPCODE => {
                    Ok(Self::XxFractionalScaleV2SetScaleFactor(
                        XxFractionalScaleV2SetScaleFactorRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XxFractionalScaleV2DestroyRequest::OPCODE => Ok(Self::XxFractionalScaleV2Destroy(
                    XxFractionalScaleV2DestroyRequest::read_with_interface(
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
            Self::XxFractionalScaleManagerV2Destroy(m) => m.object_id(),
            Self::XxFractionalScaleManagerV2GetFractionalScale(m) => m.object_id(),
            Self::XxFractionalScaleV2SetScaleFactor(m) => m.object_id(),
            Self::XxFractionalScaleV2Destroy(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XxFractionalScaleManagerV2Destroy(m) => m.opcode(),
            Self::XxFractionalScaleManagerV2GetFractionalScale(m) => m.opcode(),
            Self::XxFractionalScaleV2SetScaleFactor(m) => m.opcode(),
            Self::XxFractionalScaleV2Destroy(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XxFractionalScaleManagerV2Destroy(m) => m.payload_size(),
            Self::XxFractionalScaleManagerV2GetFractionalScale(m) => m.payload_size(),
            Self::XxFractionalScaleV2SetScaleFactor(m) => m.payload_size(),
            Self::XxFractionalScaleV2Destroy(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XxFractionalScaleManagerV2Destroy(m) => m.write(writer, state).await,
            Self::XxFractionalScaleManagerV2GetFractionalScale(m) => m.write(writer, state).await,
            Self::XxFractionalScaleV2SetScaleFactor(m) => m.write(writer, state).await,
            Self::XxFractionalScaleV2Destroy(m) => m.write(writer, state).await,
        }
    }
}
