use super::{
    WpSecurityContextManagerV1CreateListenerRequest, WpSecurityContextManagerV1DestroyRequest,
    WpSecurityContextV1CommitRequest, WpSecurityContextV1DestroyRequest,
    WpSecurityContextV1SetAppIdRequest, WpSecurityContextV1SetInstanceIdRequest,
    WpSecurityContextV1SetSandboxEngineRequest,
};
use crate::protocol::ext::security_context::ObjectInteface as SecurityContextObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Security context requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [WpSecurityContextManagerV1DestroyRequest].
    WpSecurityContextManagerV1Destroy(WpSecurityContextManagerV1DestroyRequest),
    /// Message [WpSecurityContextManagerV1CreateListenerRequest].
    WpSecurityContextManagerV1CreateListener(WpSecurityContextManagerV1CreateListenerRequest),
    /// Message [WpSecurityContextV1DestroyRequest].
    WpSecurityContextV1Destroy(WpSecurityContextV1DestroyRequest),
    /// Message [WpSecurityContextV1SetSandboxEngineRequest].
    WpSecurityContextV1SetSandboxEngine(WpSecurityContextV1SetSandboxEngineRequest),
    /// Message [WpSecurityContextV1SetAppIdRequest].
    WpSecurityContextV1SetAppId(WpSecurityContextV1SetAppIdRequest),
    /// Message [WpSecurityContextV1SetInstanceIdRequest].
    WpSecurityContextV1SetInstanceId(WpSecurityContextV1SetInstanceIdRequest),
    /// Message [WpSecurityContextV1CommitRequest].
    WpSecurityContextV1Commit(WpSecurityContextV1CommitRequest),
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
            ObjectInteface::SecurityContext(
                SecurityContextObjectInteface::WpSecurityContextManagerV1 { .. },
            ) => match opcode {
                WpSecurityContextManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::WpSecurityContextManagerV1Destroy(
                        WpSecurityContextManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WpSecurityContextManagerV1CreateListenerRequest::OPCODE => {
                    Ok(Self::WpSecurityContextManagerV1CreateListener(
                        WpSecurityContextManagerV1CreateListenerRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::SecurityContext(
                SecurityContextObjectInteface::WpSecurityContextV1 { .. },
            ) => match opcode {
                WpSecurityContextV1DestroyRequest::OPCODE => Ok(Self::WpSecurityContextV1Destroy(
                    WpSecurityContextV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WpSecurityContextV1SetSandboxEngineRequest::OPCODE => {
                    Ok(Self::WpSecurityContextV1SetSandboxEngine(
                        WpSecurityContextV1SetSandboxEngineRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WpSecurityContextV1SetAppIdRequest::OPCODE => {
                    Ok(Self::WpSecurityContextV1SetAppId(
                        WpSecurityContextV1SetAppIdRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WpSecurityContextV1SetInstanceIdRequest::OPCODE => {
                    Ok(Self::WpSecurityContextV1SetInstanceId(
                        WpSecurityContextV1SetInstanceIdRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WpSecurityContextV1CommitRequest::OPCODE => Ok(Self::WpSecurityContextV1Commit(
                    WpSecurityContextV1CommitRequest::read_with_interface(
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
            Self::WpSecurityContextManagerV1Destroy(message) => message.object_id(),
            Self::WpSecurityContextManagerV1CreateListener(message) => message.object_id(),
            Self::WpSecurityContextV1Destroy(message) => message.object_id(),
            Self::WpSecurityContextV1SetSandboxEngine(message) => message.object_id(),
            Self::WpSecurityContextV1SetAppId(message) => message.object_id(),
            Self::WpSecurityContextV1SetInstanceId(message) => message.object_id(),
            Self::WpSecurityContextV1Commit(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WpSecurityContextManagerV1Destroy(message) => message.opcode(),
            Self::WpSecurityContextManagerV1CreateListener(message) => message.opcode(),
            Self::WpSecurityContextV1Destroy(message) => message.opcode(),
            Self::WpSecurityContextV1SetSandboxEngine(message) => message.opcode(),
            Self::WpSecurityContextV1SetAppId(message) => message.opcode(),
            Self::WpSecurityContextV1SetInstanceId(message) => message.opcode(),
            Self::WpSecurityContextV1Commit(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WpSecurityContextManagerV1Destroy(message) => message.payload_size(),
            Self::WpSecurityContextManagerV1CreateListener(message) => message.payload_size(),
            Self::WpSecurityContextV1Destroy(message) => message.payload_size(),
            Self::WpSecurityContextV1SetSandboxEngine(message) => message.payload_size(),
            Self::WpSecurityContextV1SetAppId(message) => message.payload_size(),
            Self::WpSecurityContextV1SetInstanceId(message) => message.payload_size(),
            Self::WpSecurityContextV1Commit(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::WpSecurityContextManagerV1Destroy(message) => message.write(writer, state).await,
            Self::WpSecurityContextManagerV1CreateListener(message) => {
                message.write(writer, state).await
            }
            Self::WpSecurityContextV1Destroy(message) => message.write(writer, state).await,
            Self::WpSecurityContextV1SetSandboxEngine(message) => {
                message.write(writer, state).await
            }
            Self::WpSecurityContextV1SetAppId(message) => message.write(writer, state).await,
            Self::WpSecurityContextV1SetInstanceId(message) => message.write(writer, state).await,
            Self::WpSecurityContextV1Commit(message) => message.write(writer, state).await,
        }
    }
}
