use super::{
    ExtSessionLockManagerV1DestroyRequest, ExtSessionLockManagerV1LockRequest,
    ExtSessionLockSurfaceV1AckConfigureRequest, ExtSessionLockSurfaceV1DestroyRequest,
    ExtSessionLockV1DestroyRequest, ExtSessionLockV1GetLockSurfaceRequest,
    ExtSessionLockV1UnlockAndDestroyRequest,
};
use crate::protocol::ext::session_lock::ObjectInteface as SessionLockObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Session lock requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ExtSessionLockManagerV1DestroyRequest].
    ExtSessionLockManagerV1Destroy(ExtSessionLockManagerV1DestroyRequest),
    /// Message [ExtSessionLockManagerV1LockRequest].
    ExtSessionLockManagerV1Lock(ExtSessionLockManagerV1LockRequest),
    /// Message [ExtSessionLockV1DestroyRequest].
    ExtSessionLockV1Destroy(ExtSessionLockV1DestroyRequest),
    /// Message [ExtSessionLockV1GetLockSurfaceRequest].
    ExtSessionLockV1GetLockSurface(ExtSessionLockV1GetLockSurfaceRequest),
    /// Message [ExtSessionLockV1UnlockAndDestroyRequest].
    ExtSessionLockV1UnlockAndDestroy(ExtSessionLockV1UnlockAndDestroyRequest),
    /// Message [ExtSessionLockSurfaceV1DestroyRequest].
    ExtSessionLockSurfaceV1Destroy(ExtSessionLockSurfaceV1DestroyRequest),
    /// Message [ExtSessionLockSurfaceV1AckConfigureRequest].
    ExtSessionLockSurfaceV1AckConfigure(ExtSessionLockSurfaceV1AckConfigureRequest),
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
            ObjectInteface::SessionLock(SessionLockObjectInteface::ExtSessionLockManagerV1 {
                ..
            }) => match opcode {
                ExtSessionLockManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::ExtSessionLockManagerV1Destroy(
                        ExtSessionLockManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtSessionLockManagerV1LockRequest::OPCODE => {
                    Ok(Self::ExtSessionLockManagerV1Lock(
                        ExtSessionLockManagerV1LockRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::SessionLock(SessionLockObjectInteface::ExtSessionLockV1 { .. }) => {
                match opcode {
                    ExtSessionLockV1DestroyRequest::OPCODE => Ok(Self::ExtSessionLockV1Destroy(
                        ExtSessionLockV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ExtSessionLockV1GetLockSurfaceRequest::OPCODE => {
                        Ok(Self::ExtSessionLockV1GetLockSurface(
                            ExtSessionLockV1GetLockSurfaceRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ExtSessionLockV1UnlockAndDestroyRequest::OPCODE => {
                        Ok(Self::ExtSessionLockV1UnlockAndDestroy(
                            ExtSessionLockV1UnlockAndDestroyRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::SessionLock(SessionLockObjectInteface::ExtSessionLockSurfaceV1 {
                ..
            }) => match opcode {
                ExtSessionLockSurfaceV1DestroyRequest::OPCODE => {
                    Ok(Self::ExtSessionLockSurfaceV1Destroy(
                        ExtSessionLockSurfaceV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtSessionLockSurfaceV1AckConfigureRequest::OPCODE => {
                    Ok(Self::ExtSessionLockSurfaceV1AckConfigure(
                        ExtSessionLockSurfaceV1AckConfigureRequest::read_with_interface(
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
            Self::ExtSessionLockManagerV1Destroy(message) => message.object_id(),
            Self::ExtSessionLockManagerV1Lock(message) => message.object_id(),
            Self::ExtSessionLockV1Destroy(message) => message.object_id(),
            Self::ExtSessionLockV1GetLockSurface(message) => message.object_id(),
            Self::ExtSessionLockV1UnlockAndDestroy(message) => message.object_id(),
            Self::ExtSessionLockSurfaceV1Destroy(message) => message.object_id(),
            Self::ExtSessionLockSurfaceV1AckConfigure(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ExtSessionLockManagerV1Destroy(message) => message.opcode(),
            Self::ExtSessionLockManagerV1Lock(message) => message.opcode(),
            Self::ExtSessionLockV1Destroy(message) => message.opcode(),
            Self::ExtSessionLockV1GetLockSurface(message) => message.opcode(),
            Self::ExtSessionLockV1UnlockAndDestroy(message) => message.opcode(),
            Self::ExtSessionLockSurfaceV1Destroy(message) => message.opcode(),
            Self::ExtSessionLockSurfaceV1AckConfigure(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ExtSessionLockManagerV1Destroy(message) => message.payload_size(),
            Self::ExtSessionLockManagerV1Lock(message) => message.payload_size(),
            Self::ExtSessionLockV1Destroy(message) => message.payload_size(),
            Self::ExtSessionLockV1GetLockSurface(message) => message.payload_size(),
            Self::ExtSessionLockV1UnlockAndDestroy(message) => message.payload_size(),
            Self::ExtSessionLockSurfaceV1Destroy(message) => message.payload_size(),
            Self::ExtSessionLockSurfaceV1AckConfigure(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ExtSessionLockManagerV1Destroy(message) => message.write(writer, state).await,
            Self::ExtSessionLockManagerV1Lock(message) => message.write(writer, state).await,
            Self::ExtSessionLockV1Destroy(message) => message.write(writer, state).await,
            Self::ExtSessionLockV1GetLockSurface(message) => message.write(writer, state).await,
            Self::ExtSessionLockV1UnlockAndDestroy(message) => message.write(writer, state).await,
            Self::ExtSessionLockSurfaceV1Destroy(message) => message.write(writer, state).await,
            Self::ExtSessionLockSurfaceV1AckConfigure(message) => {
                message.write(writer, state).await
            }
        }
    }
}
