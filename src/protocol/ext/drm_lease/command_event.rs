use super::{
    WpDrmLeaseConnectorV1ConnectorIdEvent, WpDrmLeaseConnectorV1DescriptionEvent,
    WpDrmLeaseConnectorV1DoneEvent, WpDrmLeaseConnectorV1NameEvent,
    WpDrmLeaseConnectorV1WithdrawnEvent, WpDrmLeaseDeviceV1ConnectorEvent,
    WpDrmLeaseDeviceV1DoneEvent, WpDrmLeaseDeviceV1DrmFdEvent, WpDrmLeaseDeviceV1ReleasedEvent,
    WpDrmLeaseV1FinishedEvent, WpDrmLeaseV1LeaseFdEvent,
};
use crate::protocol::ext::drm_lease::ObjectInteface as DrmLeaseObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches DRM lease events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [WpDrmLeaseDeviceV1DrmFdEvent].
    WpDrmLeaseDeviceV1DrmFd(WpDrmLeaseDeviceV1DrmFdEvent),
    /// Message [WpDrmLeaseDeviceV1ConnectorEvent].
    WpDrmLeaseDeviceV1Connector(WpDrmLeaseDeviceV1ConnectorEvent),
    /// Message [WpDrmLeaseDeviceV1DoneEvent].
    WpDrmLeaseDeviceV1Done(WpDrmLeaseDeviceV1DoneEvent),
    /// Message [WpDrmLeaseDeviceV1ReleasedEvent].
    WpDrmLeaseDeviceV1Released(WpDrmLeaseDeviceV1ReleasedEvent),
    /// Message [WpDrmLeaseConnectorV1NameEvent].
    WpDrmLeaseConnectorV1Name(WpDrmLeaseConnectorV1NameEvent),
    /// Message [WpDrmLeaseConnectorV1DescriptionEvent].
    WpDrmLeaseConnectorV1Description(WpDrmLeaseConnectorV1DescriptionEvent),
    /// Message [WpDrmLeaseConnectorV1ConnectorIdEvent].
    WpDrmLeaseConnectorV1ConnectorId(WpDrmLeaseConnectorV1ConnectorIdEvent),
    /// Message [WpDrmLeaseConnectorV1DoneEvent].
    WpDrmLeaseConnectorV1Done(WpDrmLeaseConnectorV1DoneEvent),
    /// Message [WpDrmLeaseConnectorV1WithdrawnEvent].
    WpDrmLeaseConnectorV1Withdrawn(WpDrmLeaseConnectorV1WithdrawnEvent),
    /// Message [WpDrmLeaseV1LeaseFdEvent].
    WpDrmLeaseV1LeaseFd(WpDrmLeaseV1LeaseFdEvent),
    /// Message [WpDrmLeaseV1FinishedEvent].
    WpDrmLeaseV1Finished(WpDrmLeaseV1FinishedEvent),
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
            ObjectInteface::DrmLease(DrmLeaseObjectInteface::WpDrmLeaseDeviceV1 { .. }) => {
                match opcode {
                    WpDrmLeaseDeviceV1DrmFdEvent::OPCODE => Ok(Self::WpDrmLeaseDeviceV1DrmFd(
                        WpDrmLeaseDeviceV1DrmFdEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    WpDrmLeaseDeviceV1ConnectorEvent::OPCODE => {
                        Ok(Self::WpDrmLeaseDeviceV1Connector(
                            WpDrmLeaseDeviceV1ConnectorEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    WpDrmLeaseDeviceV1DoneEvent::OPCODE => Ok(Self::WpDrmLeaseDeviceV1Done(
                        WpDrmLeaseDeviceV1DoneEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    WpDrmLeaseDeviceV1ReleasedEvent::OPCODE => {
                        Ok(Self::WpDrmLeaseDeviceV1Released(
                            WpDrmLeaseDeviceV1ReleasedEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::DrmLease(DrmLeaseObjectInteface::WpDrmLeaseConnectorV1 { .. }) => {
                match opcode {
                    WpDrmLeaseConnectorV1NameEvent::OPCODE => Ok(Self::WpDrmLeaseConnectorV1Name(
                        WpDrmLeaseConnectorV1NameEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    WpDrmLeaseConnectorV1DescriptionEvent::OPCODE => {
                        Ok(Self::WpDrmLeaseConnectorV1Description(
                            WpDrmLeaseConnectorV1DescriptionEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    WpDrmLeaseConnectorV1ConnectorIdEvent::OPCODE => {
                        Ok(Self::WpDrmLeaseConnectorV1ConnectorId(
                            WpDrmLeaseConnectorV1ConnectorIdEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    WpDrmLeaseConnectorV1DoneEvent::OPCODE => Ok(Self::WpDrmLeaseConnectorV1Done(
                        WpDrmLeaseConnectorV1DoneEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    WpDrmLeaseConnectorV1WithdrawnEvent::OPCODE => {
                        Ok(Self::WpDrmLeaseConnectorV1Withdrawn(
                            WpDrmLeaseConnectorV1WithdrawnEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::DrmLease(DrmLeaseObjectInteface::WpDrmLeaseV1 { .. }) => match opcode {
                WpDrmLeaseV1LeaseFdEvent::OPCODE => Ok(Self::WpDrmLeaseV1LeaseFd(
                    WpDrmLeaseV1LeaseFdEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WpDrmLeaseV1FinishedEvent::OPCODE => Ok(Self::WpDrmLeaseV1Finished(
                    WpDrmLeaseV1FinishedEvent::read_with_interface(
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
impl CommandWriter for CommandEvent {
    fn object_id(&self) -> u32 {
        match self {
            Self::WpDrmLeaseDeviceV1DrmFd(message) => message.object_id(),
            Self::WpDrmLeaseDeviceV1Connector(message) => message.object_id(),
            Self::WpDrmLeaseDeviceV1Done(message) => message.object_id(),
            Self::WpDrmLeaseDeviceV1Released(message) => message.object_id(),
            Self::WpDrmLeaseConnectorV1Name(message) => message.object_id(),
            Self::WpDrmLeaseConnectorV1Description(message) => message.object_id(),
            Self::WpDrmLeaseConnectorV1ConnectorId(message) => message.object_id(),
            Self::WpDrmLeaseConnectorV1Done(message) => message.object_id(),
            Self::WpDrmLeaseConnectorV1Withdrawn(message) => message.object_id(),
            Self::WpDrmLeaseV1LeaseFd(message) => message.object_id(),
            Self::WpDrmLeaseV1Finished(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WpDrmLeaseDeviceV1DrmFd(message) => message.opcode(),
            Self::WpDrmLeaseDeviceV1Connector(message) => message.opcode(),
            Self::WpDrmLeaseDeviceV1Done(message) => message.opcode(),
            Self::WpDrmLeaseDeviceV1Released(message) => message.opcode(),
            Self::WpDrmLeaseConnectorV1Name(message) => message.opcode(),
            Self::WpDrmLeaseConnectorV1Description(message) => message.opcode(),
            Self::WpDrmLeaseConnectorV1ConnectorId(message) => message.opcode(),
            Self::WpDrmLeaseConnectorV1Done(message) => message.opcode(),
            Self::WpDrmLeaseConnectorV1Withdrawn(message) => message.opcode(),
            Self::WpDrmLeaseV1LeaseFd(message) => message.opcode(),
            Self::WpDrmLeaseV1Finished(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WpDrmLeaseDeviceV1DrmFd(message) => message.payload_size(),
            Self::WpDrmLeaseDeviceV1Connector(message) => message.payload_size(),
            Self::WpDrmLeaseDeviceV1Done(message) => message.payload_size(),
            Self::WpDrmLeaseDeviceV1Released(message) => message.payload_size(),
            Self::WpDrmLeaseConnectorV1Name(message) => message.payload_size(),
            Self::WpDrmLeaseConnectorV1Description(message) => message.payload_size(),
            Self::WpDrmLeaseConnectorV1ConnectorId(message) => message.payload_size(),
            Self::WpDrmLeaseConnectorV1Done(message) => message.payload_size(),
            Self::WpDrmLeaseConnectorV1Withdrawn(message) => message.payload_size(),
            Self::WpDrmLeaseV1LeaseFd(message) => message.payload_size(),
            Self::WpDrmLeaseV1Finished(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::WpDrmLeaseDeviceV1DrmFd(message) => message.write(writer, state).await,
            Self::WpDrmLeaseDeviceV1Connector(message) => message.write(writer, state).await,
            Self::WpDrmLeaseDeviceV1Done(message) => message.write(writer, state).await,
            Self::WpDrmLeaseDeviceV1Released(message) => message.write(writer, state).await,
            Self::WpDrmLeaseConnectorV1Name(message) => message.write(writer, state).await,
            Self::WpDrmLeaseConnectorV1Description(message) => message.write(writer, state).await,
            Self::WpDrmLeaseConnectorV1ConnectorId(message) => message.write(writer, state).await,
            Self::WpDrmLeaseConnectorV1Done(message) => message.write(writer, state).await,
            Self::WpDrmLeaseConnectorV1Withdrawn(message) => message.write(writer, state).await,
            Self::WpDrmLeaseV1LeaseFd(message) => message.write(writer, state).await,
            Self::WpDrmLeaseV1Finished(message) => message.write(writer, state).await,
        }
    }
}
