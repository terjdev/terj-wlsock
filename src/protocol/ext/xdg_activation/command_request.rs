use super::{
    XdgActivationTokenV1CommitRequest, XdgActivationTokenV1DestroyRequest,
    XdgActivationTokenV1SetAppIdRequest, XdgActivationTokenV1SetSerialRequest,
    XdgActivationTokenV1SetSurfaceRequest, XdgActivationV1ActivateRequest,
    XdgActivationV1DestroyRequest, XdgActivationV1GetActivationTokenRequest,
};
use crate::protocol::ext::xdg_activation::ObjectInteface as XdgActivationObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Xdg activation requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [XdgActivationV1DestroyRequest].
    XdgActivationV1Destroy(XdgActivationV1DestroyRequest),
    /// Message [XdgActivationV1GetActivationTokenRequest].
    XdgActivationV1GetActivationToken(XdgActivationV1GetActivationTokenRequest),
    /// Message [XdgActivationV1ActivateRequest].
    XdgActivationV1Activate(XdgActivationV1ActivateRequest),
    /// Message [XdgActivationTokenV1SetSerialRequest].
    XdgActivationTokenV1SetSerial(XdgActivationTokenV1SetSerialRequest),
    /// Message [XdgActivationTokenV1SetAppIdRequest].
    XdgActivationTokenV1SetAppId(XdgActivationTokenV1SetAppIdRequest),
    /// Message [XdgActivationTokenV1SetSurfaceRequest].
    XdgActivationTokenV1SetSurface(XdgActivationTokenV1SetSurfaceRequest),
    /// Message [XdgActivationTokenV1CommitRequest].
    XdgActivationTokenV1Commit(XdgActivationTokenV1CommitRequest),
    /// Message [XdgActivationTokenV1DestroyRequest].
    XdgActivationTokenV1Destroy(XdgActivationTokenV1DestroyRequest),
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
            ObjectInteface::XdgActivation(XdgActivationObjectInteface::XdgActivationV1 {
                ..
            }) => match opcode {
                XdgActivationV1DestroyRequest::OPCODE => Ok(Self::XdgActivationV1Destroy(
                    XdgActivationV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgActivationV1GetActivationTokenRequest::OPCODE => {
                    Ok(Self::XdgActivationV1GetActivationToken(
                        XdgActivationV1GetActivationTokenRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XdgActivationV1ActivateRequest::OPCODE => Ok(Self::XdgActivationV1Activate(
                    XdgActivationV1ActivateRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XdgActivation(XdgActivationObjectInteface::XdgActivationTokenV1 {
                ..
            }) => match opcode {
                XdgActivationTokenV1SetSerialRequest::OPCODE => {
                    Ok(Self::XdgActivationTokenV1SetSerial(
                        XdgActivationTokenV1SetSerialRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XdgActivationTokenV1SetAppIdRequest::OPCODE => {
                    Ok(Self::XdgActivationTokenV1SetAppId(
                        XdgActivationTokenV1SetAppIdRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XdgActivationTokenV1SetSurfaceRequest::OPCODE => {
                    Ok(Self::XdgActivationTokenV1SetSurface(
                        XdgActivationTokenV1SetSurfaceRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XdgActivationTokenV1CommitRequest::OPCODE => Ok(Self::XdgActivationTokenV1Commit(
                    XdgActivationTokenV1CommitRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgActivationTokenV1DestroyRequest::OPCODE => {
                    Ok(Self::XdgActivationTokenV1Destroy(
                        XdgActivationTokenV1DestroyRequest::read_with_interface(
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
            Self::XdgActivationV1Destroy(message) => message.object_id(),
            Self::XdgActivationV1GetActivationToken(message) => message.object_id(),
            Self::XdgActivationV1Activate(message) => message.object_id(),
            Self::XdgActivationTokenV1SetSerial(message) => message.object_id(),
            Self::XdgActivationTokenV1SetAppId(message) => message.object_id(),
            Self::XdgActivationTokenV1SetSurface(message) => message.object_id(),
            Self::XdgActivationTokenV1Commit(message) => message.object_id(),
            Self::XdgActivationTokenV1Destroy(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XdgActivationV1Destroy(message) => message.opcode(),
            Self::XdgActivationV1GetActivationToken(message) => message.opcode(),
            Self::XdgActivationV1Activate(message) => message.opcode(),
            Self::XdgActivationTokenV1SetSerial(message) => message.opcode(),
            Self::XdgActivationTokenV1SetAppId(message) => message.opcode(),
            Self::XdgActivationTokenV1SetSurface(message) => message.opcode(),
            Self::XdgActivationTokenV1Commit(message) => message.opcode(),
            Self::XdgActivationTokenV1Destroy(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XdgActivationV1Destroy(message) => message.payload_size(),
            Self::XdgActivationV1GetActivationToken(message) => message.payload_size(),
            Self::XdgActivationV1Activate(message) => message.payload_size(),
            Self::XdgActivationTokenV1SetSerial(message) => message.payload_size(),
            Self::XdgActivationTokenV1SetAppId(message) => message.payload_size(),
            Self::XdgActivationTokenV1SetSurface(message) => message.payload_size(),
            Self::XdgActivationTokenV1Commit(message) => message.payload_size(),
            Self::XdgActivationTokenV1Destroy(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XdgActivationV1Destroy(message) => message.write(writer, state).await,
            Self::XdgActivationV1GetActivationToken(message) => message.write(writer, state).await,
            Self::XdgActivationV1Activate(message) => message.write(writer, state).await,
            Self::XdgActivationTokenV1SetSerial(message) => message.write(writer, state).await,
            Self::XdgActivationTokenV1SetAppId(message) => message.write(writer, state).await,
            Self::XdgActivationTokenV1SetSurface(message) => message.write(writer, state).await,
            Self::XdgActivationTokenV1Commit(message) => message.write(writer, state).await,
            Self::XdgActivationTokenV1Destroy(message) => message.write(writer, state).await,
        }
    }
}
