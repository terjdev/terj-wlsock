use super::{
    WpFractionalScaleManagerV1DestroyRequest, WpFractionalScaleManagerV1GetFractionalScaleRequest,
    WpFractionalScaleV1DestroyRequest,
};
use crate::protocol::ext::fractional_scale::ObjectInteface as FractionalScaleObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Fractional scale requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [WpFractionalScaleManagerV1DestroyRequest].
    WpFractionalScaleManagerV1Destroy(WpFractionalScaleManagerV1DestroyRequest),
    /// Message [WpFractionalScaleManagerV1GetFractionalScaleRequest].
    WpFractionalScaleManagerV1GetFractionalScale(
        WpFractionalScaleManagerV1GetFractionalScaleRequest,
    ),
    /// Message [WpFractionalScaleV1DestroyRequest].
    WpFractionalScaleV1Destroy(WpFractionalScaleV1DestroyRequest),
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
            ObjectInteface::FractionalScale(
                FractionalScaleObjectInteface::WpFractionalScaleManagerV1 { .. },
            ) => match opcode {
                WpFractionalScaleManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::WpFractionalScaleManagerV1Destroy(
                        WpFractionalScaleManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WpFractionalScaleManagerV1GetFractionalScaleRequest::OPCODE => {
                    Ok(Self::WpFractionalScaleManagerV1GetFractionalScale(
                        WpFractionalScaleManagerV1GetFractionalScaleRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::FractionalScale(
                FractionalScaleObjectInteface::WpFractionalScaleV1 { .. },
            ) => match opcode {
                WpFractionalScaleV1DestroyRequest::OPCODE => Ok(Self::WpFractionalScaleV1Destroy(
                    WpFractionalScaleV1DestroyRequest::read_with_interface(
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
            Self::WpFractionalScaleManagerV1Destroy(message) => message.object_id(),
            Self::WpFractionalScaleManagerV1GetFractionalScale(message) => message.object_id(),
            Self::WpFractionalScaleV1Destroy(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WpFractionalScaleManagerV1Destroy(message) => message.opcode(),
            Self::WpFractionalScaleManagerV1GetFractionalScale(message) => message.opcode(),
            Self::WpFractionalScaleV1Destroy(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WpFractionalScaleManagerV1Destroy(message) => message.payload_size(),
            Self::WpFractionalScaleManagerV1GetFractionalScale(message) => message.payload_size(),
            Self::WpFractionalScaleV1Destroy(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::WpFractionalScaleManagerV1Destroy(message) => message.write(writer, state).await,
            Self::WpFractionalScaleManagerV1GetFractionalScale(message) => {
                message.write(writer, state).await
            }
            Self::WpFractionalScaleV1Destroy(message) => message.write(writer, state).await,
        }
    }
}
