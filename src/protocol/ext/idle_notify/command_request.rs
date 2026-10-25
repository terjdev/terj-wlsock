use super::{
    ExtIdleNotificationV1DestroyRequest, ExtIdleNotifierV1DestroyRequest,
    ExtIdleNotifierV1GetIdleNotificationRequest, ExtIdleNotifierV1GetInputIdleNotificationRequest,
};
use crate::protocol::ext::idle_notify::ObjectInteface as IdleNotifyObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Idle notify requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ExtIdleNotifierV1DestroyRequest].
    ExtIdleNotifierV1Destroy(ExtIdleNotifierV1DestroyRequest),
    /// Message [ExtIdleNotifierV1GetIdleNotificationRequest].
    ExtIdleNotifierV1GetIdleNotification(ExtIdleNotifierV1GetIdleNotificationRequest),
    /// Message [ExtIdleNotifierV1GetInputIdleNotificationRequest].
    ExtIdleNotifierV1GetInputIdleNotification(ExtIdleNotifierV1GetInputIdleNotificationRequest),
    /// Message [ExtIdleNotificationV1DestroyRequest].
    ExtIdleNotificationV1Destroy(ExtIdleNotificationV1DestroyRequest),
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
            ObjectInteface::IdleNotify(IdleNotifyObjectInteface::ExtIdleNotifierV1 { .. }) => {
                match opcode {
                    ExtIdleNotifierV1DestroyRequest::OPCODE => Ok(Self::ExtIdleNotifierV1Destroy(
                        ExtIdleNotifierV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ExtIdleNotifierV1GetIdleNotificationRequest::OPCODE => {
                        Ok(Self::ExtIdleNotifierV1GetIdleNotification(
                            ExtIdleNotifierV1GetIdleNotificationRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ExtIdleNotifierV1GetInputIdleNotificationRequest::OPCODE => {
                        Ok(Self::ExtIdleNotifierV1GetInputIdleNotification(
                            ExtIdleNotifierV1GetInputIdleNotificationRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::IdleNotify(IdleNotifyObjectInteface::ExtIdleNotificationV1 {
                ..
            }) => match opcode {
                ExtIdleNotificationV1DestroyRequest::OPCODE => {
                    Ok(Self::ExtIdleNotificationV1Destroy(
                        ExtIdleNotificationV1DestroyRequest::read_with_interface(
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
            Self::ExtIdleNotifierV1Destroy(message) => message.object_id(),
            Self::ExtIdleNotifierV1GetIdleNotification(message) => message.object_id(),
            Self::ExtIdleNotifierV1GetInputIdleNotification(message) => message.object_id(),
            Self::ExtIdleNotificationV1Destroy(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ExtIdleNotifierV1Destroy(message) => message.opcode(),
            Self::ExtIdleNotifierV1GetIdleNotification(message) => message.opcode(),
            Self::ExtIdleNotifierV1GetInputIdleNotification(message) => message.opcode(),
            Self::ExtIdleNotificationV1Destroy(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ExtIdleNotifierV1Destroy(message) => message.payload_size(),
            Self::ExtIdleNotifierV1GetIdleNotification(message) => message.payload_size(),
            Self::ExtIdleNotifierV1GetInputIdleNotification(message) => message.payload_size(),
            Self::ExtIdleNotificationV1Destroy(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ExtIdleNotifierV1Destroy(message) => message.write(writer, state).await,
            Self::ExtIdleNotifierV1GetIdleNotification(message) => {
                message.write(writer, state).await
            }
            Self::ExtIdleNotifierV1GetInputIdleNotification(message) => {
                message.write(writer, state).await
            }
            Self::ExtIdleNotificationV1Destroy(message) => message.write(writer, state).await,
        }
    }
}
