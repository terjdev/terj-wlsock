use super::ObjectInteface as XxCutoutsObjectInteface;
use super::{
    XxCutoutsManagerV1DestroyRequest, XxCutoutsManagerV1GetCutoutsRequest,
    XxCutoutsV1DestroyRequest, XxCutoutsV1SetUnhandledRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches cutouts requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [XxCutoutsManagerV1DestroyRequest].
    XxCutoutsManagerV1Destroy(XxCutoutsManagerV1DestroyRequest),
    /// Message [XxCutoutsManagerV1GetCutoutsRequest].
    XxCutoutsManagerV1GetCutouts(XxCutoutsManagerV1GetCutoutsRequest),
    /// Message [XxCutoutsV1DestroyRequest].
    XxCutoutsV1Destroy(XxCutoutsV1DestroyRequest),
    /// Message [XxCutoutsV1SetUnhandledRequest].
    XxCutoutsV1SetUnhandled(XxCutoutsV1SetUnhandledRequest),
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
            ObjectInteface::XxCutouts(XxCutoutsObjectInteface::XxCutoutsManagerV1 { .. }) => {
                match opcode {
                    XxCutoutsManagerV1DestroyRequest::OPCODE => {
                        Ok(Self::XxCutoutsManagerV1Destroy(
                            XxCutoutsManagerV1DestroyRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    XxCutoutsManagerV1GetCutoutsRequest::OPCODE => {
                        Ok(Self::XxCutoutsManagerV1GetCutouts(
                            XxCutoutsManagerV1GetCutoutsRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::XxCutouts(XxCutoutsObjectInteface::XxCutoutsV1 { .. }) => {
                match opcode {
                    XxCutoutsV1DestroyRequest::OPCODE => Ok(Self::XxCutoutsV1Destroy(
                        XxCutoutsV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XxCutoutsV1SetUnhandledRequest::OPCODE => Ok(Self::XxCutoutsV1SetUnhandled(
                        XxCutoutsV1SetUnhandledRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
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
            Self::XxCutoutsManagerV1Destroy(m) => m.object_id(),
            Self::XxCutoutsManagerV1GetCutouts(m) => m.object_id(),
            Self::XxCutoutsV1Destroy(m) => m.object_id(),
            Self::XxCutoutsV1SetUnhandled(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XxCutoutsManagerV1Destroy(m) => m.opcode(),
            Self::XxCutoutsManagerV1GetCutouts(m) => m.opcode(),
            Self::XxCutoutsV1Destroy(m) => m.opcode(),
            Self::XxCutoutsV1SetUnhandled(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XxCutoutsManagerV1Destroy(m) => m.payload_size(),
            Self::XxCutoutsManagerV1GetCutouts(m) => m.payload_size(),
            Self::XxCutoutsV1Destroy(m) => m.payload_size(),
            Self::XxCutoutsV1SetUnhandled(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XxCutoutsManagerV1Destroy(m) => m.write(writer, state).await,
            Self::XxCutoutsManagerV1GetCutouts(m) => m.write(writer, state).await,
            Self::XxCutoutsV1Destroy(m) => m.write(writer, state).await,
            Self::XxCutoutsV1SetUnhandled(m) => m.write(writer, state).await,
        }
    }
}
