use super::{
    ExtWorkspaceGroupHandleV1CreateWorkspaceRequest, ExtWorkspaceGroupHandleV1DestroyRequest,
    ExtWorkspaceHandleV1ActivateRequest, ExtWorkspaceHandleV1AssignRequest,
    ExtWorkspaceHandleV1DeactivateRequest, ExtWorkspaceHandleV1DestroyRequest,
    ExtWorkspaceHandleV1RemoveRequest, ExtWorkspaceManagerV1CommitRequest,
    ExtWorkspaceManagerV1StopRequest,
};
use crate::protocol::ext::workspace::ObjectInteface as WorkspaceObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Workspace requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ExtWorkspaceManagerV1CommitRequest].
    ExtWorkspaceManagerV1Commit(ExtWorkspaceManagerV1CommitRequest),
    /// Message [ExtWorkspaceManagerV1StopRequest].
    ExtWorkspaceManagerV1Stop(ExtWorkspaceManagerV1StopRequest),
    /// Message [ExtWorkspaceGroupHandleV1CreateWorkspaceRequest].
    ExtWorkspaceGroupHandleV1CreateWorkspace(ExtWorkspaceGroupHandleV1CreateWorkspaceRequest),
    /// Message [ExtWorkspaceGroupHandleV1DestroyRequest].
    ExtWorkspaceGroupHandleV1Destroy(ExtWorkspaceGroupHandleV1DestroyRequest),
    /// Message [ExtWorkspaceHandleV1DestroyRequest].
    ExtWorkspaceHandleV1Destroy(ExtWorkspaceHandleV1DestroyRequest),
    /// Message [ExtWorkspaceHandleV1ActivateRequest].
    ExtWorkspaceHandleV1Activate(ExtWorkspaceHandleV1ActivateRequest),
    /// Message [ExtWorkspaceHandleV1DeactivateRequest].
    ExtWorkspaceHandleV1Deactivate(ExtWorkspaceHandleV1DeactivateRequest),
    /// Message [ExtWorkspaceHandleV1AssignRequest].
    ExtWorkspaceHandleV1Assign(ExtWorkspaceHandleV1AssignRequest),
    /// Message [ExtWorkspaceHandleV1RemoveRequest].
    ExtWorkspaceHandleV1Remove(ExtWorkspaceHandleV1RemoveRequest),
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
            ObjectInteface::Workspace(WorkspaceObjectInteface::ExtWorkspaceManagerV1 {
                ..
            }) => match opcode {
                ExtWorkspaceManagerV1CommitRequest::OPCODE => {
                    Ok(Self::ExtWorkspaceManagerV1Commit(
                        ExtWorkspaceManagerV1CommitRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtWorkspaceManagerV1StopRequest::OPCODE => Ok(Self::ExtWorkspaceManagerV1Stop(
                    ExtWorkspaceManagerV1StopRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Workspace(WorkspaceObjectInteface::ExtWorkspaceGroupHandleV1 {
                ..
            }) => match opcode {
                ExtWorkspaceGroupHandleV1CreateWorkspaceRequest::OPCODE => {
                    Ok(Self::ExtWorkspaceGroupHandleV1CreateWorkspace(
                        ExtWorkspaceGroupHandleV1CreateWorkspaceRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtWorkspaceGroupHandleV1DestroyRequest::OPCODE => {
                    Ok(Self::ExtWorkspaceGroupHandleV1Destroy(
                        ExtWorkspaceGroupHandleV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Workspace(WorkspaceObjectInteface::ExtWorkspaceHandleV1 { .. }) => {
                match opcode {
                    ExtWorkspaceHandleV1DestroyRequest::OPCODE => {
                        Ok(Self::ExtWorkspaceHandleV1Destroy(
                            ExtWorkspaceHandleV1DestroyRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ExtWorkspaceHandleV1ActivateRequest::OPCODE => {
                        Ok(Self::ExtWorkspaceHandleV1Activate(
                            ExtWorkspaceHandleV1ActivateRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ExtWorkspaceHandleV1DeactivateRequest::OPCODE => {
                        Ok(Self::ExtWorkspaceHandleV1Deactivate(
                            ExtWorkspaceHandleV1DeactivateRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ExtWorkspaceHandleV1AssignRequest::OPCODE => {
                        Ok(Self::ExtWorkspaceHandleV1Assign(
                            ExtWorkspaceHandleV1AssignRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ExtWorkspaceHandleV1RemoveRequest::OPCODE => {
                        Ok(Self::ExtWorkspaceHandleV1Remove(
                            ExtWorkspaceHandleV1RemoveRequest::read_with_interface(
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
            Self::ExtWorkspaceManagerV1Commit(message) => message.object_id(),
            Self::ExtWorkspaceManagerV1Stop(message) => message.object_id(),
            Self::ExtWorkspaceGroupHandleV1CreateWorkspace(message) => message.object_id(),
            Self::ExtWorkspaceGroupHandleV1Destroy(message) => message.object_id(),
            Self::ExtWorkspaceHandleV1Destroy(message) => message.object_id(),
            Self::ExtWorkspaceHandleV1Activate(message) => message.object_id(),
            Self::ExtWorkspaceHandleV1Deactivate(message) => message.object_id(),
            Self::ExtWorkspaceHandleV1Assign(message) => message.object_id(),
            Self::ExtWorkspaceHandleV1Remove(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ExtWorkspaceManagerV1Commit(message) => message.opcode(),
            Self::ExtWorkspaceManagerV1Stop(message) => message.opcode(),
            Self::ExtWorkspaceGroupHandleV1CreateWorkspace(message) => message.opcode(),
            Self::ExtWorkspaceGroupHandleV1Destroy(message) => message.opcode(),
            Self::ExtWorkspaceHandleV1Destroy(message) => message.opcode(),
            Self::ExtWorkspaceHandleV1Activate(message) => message.opcode(),
            Self::ExtWorkspaceHandleV1Deactivate(message) => message.opcode(),
            Self::ExtWorkspaceHandleV1Assign(message) => message.opcode(),
            Self::ExtWorkspaceHandleV1Remove(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ExtWorkspaceManagerV1Commit(message) => message.payload_size(),
            Self::ExtWorkspaceManagerV1Stop(message) => message.payload_size(),
            Self::ExtWorkspaceGroupHandleV1CreateWorkspace(message) => message.payload_size(),
            Self::ExtWorkspaceGroupHandleV1Destroy(message) => message.payload_size(),
            Self::ExtWorkspaceHandleV1Destroy(message) => message.payload_size(),
            Self::ExtWorkspaceHandleV1Activate(message) => message.payload_size(),
            Self::ExtWorkspaceHandleV1Deactivate(message) => message.payload_size(),
            Self::ExtWorkspaceHandleV1Assign(message) => message.payload_size(),
            Self::ExtWorkspaceHandleV1Remove(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ExtWorkspaceManagerV1Commit(message) => message.write(writer, state).await,
            Self::ExtWorkspaceManagerV1Stop(message) => message.write(writer, state).await,
            Self::ExtWorkspaceGroupHandleV1CreateWorkspace(message) => {
                message.write(writer, state).await
            }
            Self::ExtWorkspaceGroupHandleV1Destroy(message) => message.write(writer, state).await,
            Self::ExtWorkspaceHandleV1Destroy(message) => message.write(writer, state).await,
            Self::ExtWorkspaceHandleV1Activate(message) => message.write(writer, state).await,
            Self::ExtWorkspaceHandleV1Deactivate(message) => message.write(writer, state).await,
            Self::ExtWorkspaceHandleV1Assign(message) => message.write(writer, state).await,
            Self::ExtWorkspaceHandleV1Remove(message) => message.write(writer, state).await,
        }
    }
}
