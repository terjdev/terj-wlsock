use super::{
    WpTearingControlManagerV1DestroyRequest, WpTearingControlManagerV1GetTearingControlRequest,
    WpTearingControlV1DestroyRequest, WpTearingControlV1SetPresentationHintRequest,
};
use crate::protocol::ext::tearing_control::ObjectInteface as TearingControlObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Tearing control requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [WpTearingControlManagerV1DestroyRequest].
    WpTearingControlManagerV1Destroy(WpTearingControlManagerV1DestroyRequest),
    /// Message [WpTearingControlManagerV1GetTearingControlRequest].
    WpTearingControlManagerV1GetTearingControl(WpTearingControlManagerV1GetTearingControlRequest),
    /// Message [WpTearingControlV1SetPresentationHintRequest].
    WpTearingControlV1SetPresentationHint(WpTearingControlV1SetPresentationHintRequest),
    /// Message [WpTearingControlV1DestroyRequest].
    WpTearingControlV1Destroy(WpTearingControlV1DestroyRequest),
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
            ObjectInteface::TearingControl(
                TearingControlObjectInteface::WpTearingControlManagerV1 { .. },
            ) => match opcode {
                WpTearingControlManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::WpTearingControlManagerV1Destroy(
                        WpTearingControlManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WpTearingControlManagerV1GetTearingControlRequest::OPCODE => {
                    Ok(Self::WpTearingControlManagerV1GetTearingControl(
                        WpTearingControlManagerV1GetTearingControlRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::TearingControl(TearingControlObjectInteface::WpTearingControlV1 {
                ..
            }) => match opcode {
                WpTearingControlV1SetPresentationHintRequest::OPCODE => {
                    Ok(Self::WpTearingControlV1SetPresentationHint(
                        WpTearingControlV1SetPresentationHintRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WpTearingControlV1DestroyRequest::OPCODE => Ok(Self::WpTearingControlV1Destroy(
                    WpTearingControlV1DestroyRequest::read_with_interface(
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
            Self::WpTearingControlManagerV1Destroy(message) => message.object_id(),
            Self::WpTearingControlManagerV1GetTearingControl(message) => message.object_id(),
            Self::WpTearingControlV1SetPresentationHint(message) => message.object_id(),
            Self::WpTearingControlV1Destroy(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WpTearingControlManagerV1Destroy(message) => message.opcode(),
            Self::WpTearingControlManagerV1GetTearingControl(message) => message.opcode(),
            Self::WpTearingControlV1SetPresentationHint(message) => message.opcode(),
            Self::WpTearingControlV1Destroy(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WpTearingControlManagerV1Destroy(message) => message.payload_size(),
            Self::WpTearingControlManagerV1GetTearingControl(message) => message.payload_size(),
            Self::WpTearingControlV1SetPresentationHint(message) => message.payload_size(),
            Self::WpTearingControlV1Destroy(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::WpTearingControlManagerV1Destroy(message) => message.write(writer, state).await,
            Self::WpTearingControlManagerV1GetTearingControl(message) => {
                message.write(writer, state).await
            }
            Self::WpTearingControlV1SetPresentationHint(message) => {
                message.write(writer, state).await
            }
            Self::WpTearingControlV1Destroy(message) => message.write(writer, state).await,
        }
    }
}
