use super::ObjectInteface as WlrGammaControlObjectInteface;
use super::{
    ZwlrGammaControlManagerV1DestroyRequest, ZwlrGammaControlManagerV1GetGammaControlRequest,
    ZwlrGammaControlV1DestroyRequest, ZwlrGammaControlV1SetGammaRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches gamma control requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwlrGammaControlManagerV1GetGammaControlRequest].
    ZwlrGammaControlManagerV1GetGammaControl(ZwlrGammaControlManagerV1GetGammaControlRequest),
    /// Message [ZwlrGammaControlManagerV1DestroyRequest].
    ZwlrGammaControlManagerV1Destroy(ZwlrGammaControlManagerV1DestroyRequest),
    /// Message [ZwlrGammaControlV1SetGammaRequest].
    ZwlrGammaControlV1SetGamma(ZwlrGammaControlV1SetGammaRequest),
    /// Message [ZwlrGammaControlV1DestroyRequest].
    ZwlrGammaControlV1Destroy(ZwlrGammaControlV1DestroyRequest),
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
            ObjectInteface::WlrGammaControl(
                WlrGammaControlObjectInteface::ZwlrGammaControlManagerV1 { .. },
            ) => match opcode {
                ZwlrGammaControlManagerV1GetGammaControlRequest::OPCODE => {
                    Ok(Self::ZwlrGammaControlManagerV1GetGammaControl(
                        ZwlrGammaControlManagerV1GetGammaControlRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrGammaControlManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwlrGammaControlManagerV1Destroy(
                        ZwlrGammaControlManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::WlrGammaControl(
                WlrGammaControlObjectInteface::ZwlrGammaControlV1 { .. },
            ) => match opcode {
                ZwlrGammaControlV1SetGammaRequest::OPCODE => Ok(Self::ZwlrGammaControlV1SetGamma(
                    ZwlrGammaControlV1SetGammaRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrGammaControlV1DestroyRequest::OPCODE => Ok(Self::ZwlrGammaControlV1Destroy(
                    ZwlrGammaControlV1DestroyRequest::read_with_interface(
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
            Self::ZwlrGammaControlManagerV1GetGammaControl(m) => m.object_id(),
            Self::ZwlrGammaControlManagerV1Destroy(m) => m.object_id(),
            Self::ZwlrGammaControlV1SetGamma(m) => m.object_id(),
            Self::ZwlrGammaControlV1Destroy(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwlrGammaControlManagerV1GetGammaControl(m) => m.opcode(),
            Self::ZwlrGammaControlManagerV1Destroy(m) => m.opcode(),
            Self::ZwlrGammaControlV1SetGamma(m) => m.opcode(),
            Self::ZwlrGammaControlV1Destroy(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwlrGammaControlManagerV1GetGammaControl(m) => m.payload_size(),
            Self::ZwlrGammaControlManagerV1Destroy(m) => m.payload_size(),
            Self::ZwlrGammaControlV1SetGamma(m) => m.payload_size(),
            Self::ZwlrGammaControlV1Destroy(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwlrGammaControlManagerV1GetGammaControl(m) => m.write(writer, state).await,
            Self::ZwlrGammaControlManagerV1Destroy(m) => m.write(writer, state).await,
            Self::ZwlrGammaControlV1SetGamma(m) => m.write(writer, state).await,
            Self::ZwlrGammaControlV1Destroy(m) => m.write(writer, state).await,
        }
    }
}
