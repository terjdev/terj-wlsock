use super::{
    ExtWorkspaceGroupHandleV1CapabilitiesEvent, ExtWorkspaceGroupHandleV1OutputEnterEvent,
    ExtWorkspaceGroupHandleV1OutputLeaveEvent, ExtWorkspaceGroupHandleV1RemovedEvent,
    ExtWorkspaceGroupHandleV1WorkspaceEnterEvent, ExtWorkspaceGroupHandleV1WorkspaceLeaveEvent,
    ExtWorkspaceHandleV1CapabilitiesEvent, ExtWorkspaceHandleV1CoordinatesEvent,
    ExtWorkspaceHandleV1IdEvent, ExtWorkspaceHandleV1NameEvent, ExtWorkspaceHandleV1RemovedEvent,
    ExtWorkspaceHandleV1StateEvent, ExtWorkspaceManagerV1DoneEvent,
    ExtWorkspaceManagerV1FinishedEvent, ExtWorkspaceManagerV1WorkspaceEvent,
    ExtWorkspaceManagerV1WorkspaceGroupEvent,
};
use crate::protocol::ext::workspace::ObjectInteface as WorkspaceObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Workspace events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ExtWorkspaceManagerV1WorkspaceGroupEvent].
    ExtWorkspaceManagerV1WorkspaceGroup(ExtWorkspaceManagerV1WorkspaceGroupEvent),
    /// Message [ExtWorkspaceManagerV1WorkspaceEvent].
    ExtWorkspaceManagerV1Workspace(ExtWorkspaceManagerV1WorkspaceEvent),
    /// Message [ExtWorkspaceManagerV1DoneEvent].
    ExtWorkspaceManagerV1Done(ExtWorkspaceManagerV1DoneEvent),
    /// Message [ExtWorkspaceManagerV1FinishedEvent].
    ExtWorkspaceManagerV1Finished(ExtWorkspaceManagerV1FinishedEvent),
    /// Message [ExtWorkspaceGroupHandleV1CapabilitiesEvent].
    ExtWorkspaceGroupHandleV1Capabilities(ExtWorkspaceGroupHandleV1CapabilitiesEvent),
    /// Message [ExtWorkspaceGroupHandleV1OutputEnterEvent].
    ExtWorkspaceGroupHandleV1OutputEnter(ExtWorkspaceGroupHandleV1OutputEnterEvent),
    /// Message [ExtWorkspaceGroupHandleV1OutputLeaveEvent].
    ExtWorkspaceGroupHandleV1OutputLeave(ExtWorkspaceGroupHandleV1OutputLeaveEvent),
    /// Message [ExtWorkspaceGroupHandleV1WorkspaceEnterEvent].
    ExtWorkspaceGroupHandleV1WorkspaceEnter(ExtWorkspaceGroupHandleV1WorkspaceEnterEvent),
    /// Message [ExtWorkspaceGroupHandleV1WorkspaceLeaveEvent].
    ExtWorkspaceGroupHandleV1WorkspaceLeave(ExtWorkspaceGroupHandleV1WorkspaceLeaveEvent),
    /// Message [ExtWorkspaceGroupHandleV1RemovedEvent].
    ExtWorkspaceGroupHandleV1Removed(ExtWorkspaceGroupHandleV1RemovedEvent),
    /// Message [ExtWorkspaceHandleV1IdEvent].
    ExtWorkspaceHandleV1Id(ExtWorkspaceHandleV1IdEvent),
    /// Message [ExtWorkspaceHandleV1NameEvent].
    ExtWorkspaceHandleV1Name(ExtWorkspaceHandleV1NameEvent),
    /// Message [ExtWorkspaceHandleV1CoordinatesEvent].
    ExtWorkspaceHandleV1Coordinates(ExtWorkspaceHandleV1CoordinatesEvent),
    /// Message [ExtWorkspaceHandleV1StateEvent].
    ExtWorkspaceHandleV1State(ExtWorkspaceHandleV1StateEvent),
    /// Message [ExtWorkspaceHandleV1CapabilitiesEvent].
    ExtWorkspaceHandleV1Capabilities(ExtWorkspaceHandleV1CapabilitiesEvent),
    /// Message [ExtWorkspaceHandleV1RemovedEvent].
    ExtWorkspaceHandleV1Removed(ExtWorkspaceHandleV1RemovedEvent),
}
impl CommandReader<Self> for CommandEvent {
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
impl InterfaceReader for CommandEvent {
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
                ExtWorkspaceManagerV1WorkspaceGroupEvent::OPCODE => {
                    Ok(Self::ExtWorkspaceManagerV1WorkspaceGroup(
                        ExtWorkspaceManagerV1WorkspaceGroupEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtWorkspaceManagerV1WorkspaceEvent::OPCODE => {
                    Ok(Self::ExtWorkspaceManagerV1Workspace(
                        ExtWorkspaceManagerV1WorkspaceEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtWorkspaceManagerV1DoneEvent::OPCODE => Ok(Self::ExtWorkspaceManagerV1Done(
                    ExtWorkspaceManagerV1DoneEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ExtWorkspaceManagerV1FinishedEvent::OPCODE => {
                    Ok(Self::ExtWorkspaceManagerV1Finished(
                        ExtWorkspaceManagerV1FinishedEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Workspace(WorkspaceObjectInteface::ExtWorkspaceGroupHandleV1 {
                ..
            }) => match opcode {
                ExtWorkspaceGroupHandleV1CapabilitiesEvent::OPCODE => {
                    Ok(Self::ExtWorkspaceGroupHandleV1Capabilities(
                        ExtWorkspaceGroupHandleV1CapabilitiesEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtWorkspaceGroupHandleV1OutputEnterEvent::OPCODE => {
                    Ok(Self::ExtWorkspaceGroupHandleV1OutputEnter(
                        ExtWorkspaceGroupHandleV1OutputEnterEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtWorkspaceGroupHandleV1OutputLeaveEvent::OPCODE => {
                    Ok(Self::ExtWorkspaceGroupHandleV1OutputLeave(
                        ExtWorkspaceGroupHandleV1OutputLeaveEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtWorkspaceGroupHandleV1WorkspaceEnterEvent::OPCODE => {
                    Ok(Self::ExtWorkspaceGroupHandleV1WorkspaceEnter(
                        ExtWorkspaceGroupHandleV1WorkspaceEnterEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtWorkspaceGroupHandleV1WorkspaceLeaveEvent::OPCODE => {
                    Ok(Self::ExtWorkspaceGroupHandleV1WorkspaceLeave(
                        ExtWorkspaceGroupHandleV1WorkspaceLeaveEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtWorkspaceGroupHandleV1RemovedEvent::OPCODE => {
                    Ok(Self::ExtWorkspaceGroupHandleV1Removed(
                        ExtWorkspaceGroupHandleV1RemovedEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Workspace(WorkspaceObjectInteface::ExtWorkspaceHandleV1 { .. }) => {
                match opcode {
                    ExtWorkspaceHandleV1IdEvent::OPCODE => Ok(Self::ExtWorkspaceHandleV1Id(
                        ExtWorkspaceHandleV1IdEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ExtWorkspaceHandleV1NameEvent::OPCODE => Ok(Self::ExtWorkspaceHandleV1Name(
                        ExtWorkspaceHandleV1NameEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ExtWorkspaceHandleV1CoordinatesEvent::OPCODE => {
                        Ok(Self::ExtWorkspaceHandleV1Coordinates(
                            ExtWorkspaceHandleV1CoordinatesEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ExtWorkspaceHandleV1StateEvent::OPCODE => Ok(Self::ExtWorkspaceHandleV1State(
                        ExtWorkspaceHandleV1StateEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ExtWorkspaceHandleV1CapabilitiesEvent::OPCODE => {
                        Ok(Self::ExtWorkspaceHandleV1Capabilities(
                            ExtWorkspaceHandleV1CapabilitiesEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ExtWorkspaceHandleV1RemovedEvent::OPCODE => {
                        Ok(Self::ExtWorkspaceHandleV1Removed(
                            ExtWorkspaceHandleV1RemovedEvent::read_with_interface(
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
impl CommandWriter for CommandEvent {
    fn object_id(&self) -> u32 {
        match self {
            Self::ExtWorkspaceManagerV1WorkspaceGroup(message) => message.object_id(),
            Self::ExtWorkspaceManagerV1Workspace(message) => message.object_id(),
            Self::ExtWorkspaceManagerV1Done(message) => message.object_id(),
            Self::ExtWorkspaceManagerV1Finished(message) => message.object_id(),
            Self::ExtWorkspaceGroupHandleV1Capabilities(message) => message.object_id(),
            Self::ExtWorkspaceGroupHandleV1OutputEnter(message) => message.object_id(),
            Self::ExtWorkspaceGroupHandleV1OutputLeave(message) => message.object_id(),
            Self::ExtWorkspaceGroupHandleV1WorkspaceEnter(message) => message.object_id(),
            Self::ExtWorkspaceGroupHandleV1WorkspaceLeave(message) => message.object_id(),
            Self::ExtWorkspaceGroupHandleV1Removed(message) => message.object_id(),
            Self::ExtWorkspaceHandleV1Id(message) => message.object_id(),
            Self::ExtWorkspaceHandleV1Name(message) => message.object_id(),
            Self::ExtWorkspaceHandleV1Coordinates(message) => message.object_id(),
            Self::ExtWorkspaceHandleV1State(message) => message.object_id(),
            Self::ExtWorkspaceHandleV1Capabilities(message) => message.object_id(),
            Self::ExtWorkspaceHandleV1Removed(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ExtWorkspaceManagerV1WorkspaceGroup(message) => message.opcode(),
            Self::ExtWorkspaceManagerV1Workspace(message) => message.opcode(),
            Self::ExtWorkspaceManagerV1Done(message) => message.opcode(),
            Self::ExtWorkspaceManagerV1Finished(message) => message.opcode(),
            Self::ExtWorkspaceGroupHandleV1Capabilities(message) => message.opcode(),
            Self::ExtWorkspaceGroupHandleV1OutputEnter(message) => message.opcode(),
            Self::ExtWorkspaceGroupHandleV1OutputLeave(message) => message.opcode(),
            Self::ExtWorkspaceGroupHandleV1WorkspaceEnter(message) => message.opcode(),
            Self::ExtWorkspaceGroupHandleV1WorkspaceLeave(message) => message.opcode(),
            Self::ExtWorkspaceGroupHandleV1Removed(message) => message.opcode(),
            Self::ExtWorkspaceHandleV1Id(message) => message.opcode(),
            Self::ExtWorkspaceHandleV1Name(message) => message.opcode(),
            Self::ExtWorkspaceHandleV1Coordinates(message) => message.opcode(),
            Self::ExtWorkspaceHandleV1State(message) => message.opcode(),
            Self::ExtWorkspaceHandleV1Capabilities(message) => message.opcode(),
            Self::ExtWorkspaceHandleV1Removed(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ExtWorkspaceManagerV1WorkspaceGroup(message) => message.payload_size(),
            Self::ExtWorkspaceManagerV1Workspace(message) => message.payload_size(),
            Self::ExtWorkspaceManagerV1Done(message) => message.payload_size(),
            Self::ExtWorkspaceManagerV1Finished(message) => message.payload_size(),
            Self::ExtWorkspaceGroupHandleV1Capabilities(message) => message.payload_size(),
            Self::ExtWorkspaceGroupHandleV1OutputEnter(message) => message.payload_size(),
            Self::ExtWorkspaceGroupHandleV1OutputLeave(message) => message.payload_size(),
            Self::ExtWorkspaceGroupHandleV1WorkspaceEnter(message) => message.payload_size(),
            Self::ExtWorkspaceGroupHandleV1WorkspaceLeave(message) => message.payload_size(),
            Self::ExtWorkspaceGroupHandleV1Removed(message) => message.payload_size(),
            Self::ExtWorkspaceHandleV1Id(message) => message.payload_size(),
            Self::ExtWorkspaceHandleV1Name(message) => message.payload_size(),
            Self::ExtWorkspaceHandleV1Coordinates(message) => message.payload_size(),
            Self::ExtWorkspaceHandleV1State(message) => message.payload_size(),
            Self::ExtWorkspaceHandleV1Capabilities(message) => message.payload_size(),
            Self::ExtWorkspaceHandleV1Removed(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ExtWorkspaceManagerV1WorkspaceGroup(message) => {
                message.write(writer, state).await
            }
            Self::ExtWorkspaceManagerV1Workspace(message) => message.write(writer, state).await,
            Self::ExtWorkspaceManagerV1Done(message) => message.write(writer, state).await,
            Self::ExtWorkspaceManagerV1Finished(message) => message.write(writer, state).await,
            Self::ExtWorkspaceGroupHandleV1Capabilities(message) => {
                message.write(writer, state).await
            }
            Self::ExtWorkspaceGroupHandleV1OutputEnter(message) => {
                message.write(writer, state).await
            }
            Self::ExtWorkspaceGroupHandleV1OutputLeave(message) => {
                message.write(writer, state).await
            }
            Self::ExtWorkspaceGroupHandleV1WorkspaceEnter(message) => {
                message.write(writer, state).await
            }
            Self::ExtWorkspaceGroupHandleV1WorkspaceLeave(message) => {
                message.write(writer, state).await
            }
            Self::ExtWorkspaceGroupHandleV1Removed(message) => message.write(writer, state).await,
            Self::ExtWorkspaceHandleV1Id(message) => message.write(writer, state).await,
            Self::ExtWorkspaceHandleV1Name(message) => message.write(writer, state).await,
            Self::ExtWorkspaceHandleV1Coordinates(message) => message.write(writer, state).await,
            Self::ExtWorkspaceHandleV1State(message) => message.write(writer, state).await,
            Self::ExtWorkspaceHandleV1Capabilities(message) => message.write(writer, state).await,
            Self::ExtWorkspaceHandleV1Removed(message) => message.write(writer, state).await,
        }
    }
}
