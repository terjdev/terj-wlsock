use super::ObjectInteface as WlrOutputManagementObjectInteface;
use super::{
    ZwlrOutputConfigurationV1CancelledEvent, ZwlrOutputConfigurationV1FailedEvent,
    ZwlrOutputConfigurationV1SucceededEvent, ZwlrOutputHeadV1AdaptiveSyncEvent,
    ZwlrOutputHeadV1CurrentModeEvent, ZwlrOutputHeadV1DescriptionEvent,
    ZwlrOutputHeadV1EnabledEvent, ZwlrOutputHeadV1FinishedEvent, ZwlrOutputHeadV1MakeEvent,
    ZwlrOutputHeadV1ModeEvent, ZwlrOutputHeadV1ModelEvent, ZwlrOutputHeadV1NameEvent,
    ZwlrOutputHeadV1PhysicalSizeEvent, ZwlrOutputHeadV1PositionEvent, ZwlrOutputHeadV1ScaleEvent,
    ZwlrOutputHeadV1SerialNumberEvent, ZwlrOutputHeadV1TransformEvent,
    ZwlrOutputManagerV1DoneEvent, ZwlrOutputManagerV1FinishedEvent, ZwlrOutputManagerV1HeadEvent,
    ZwlrOutputModeV1FinishedEvent, ZwlrOutputModeV1PreferredEvent, ZwlrOutputModeV1RefreshEvent,
    ZwlrOutputModeV1SizeEvent,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches output management events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZwlrOutputManagerV1HeadEvent].
    ZwlrOutputManagerV1Head(ZwlrOutputManagerV1HeadEvent),
    /// Message [ZwlrOutputManagerV1DoneEvent].
    ZwlrOutputManagerV1Done(ZwlrOutputManagerV1DoneEvent),
    /// Message [ZwlrOutputManagerV1FinishedEvent].
    ZwlrOutputManagerV1Finished(ZwlrOutputManagerV1FinishedEvent),
    /// Message [ZwlrOutputHeadV1NameEvent].
    ZwlrOutputHeadV1Name(ZwlrOutputHeadV1NameEvent),
    /// Message [ZwlrOutputHeadV1DescriptionEvent].
    ZwlrOutputHeadV1Description(ZwlrOutputHeadV1DescriptionEvent),
    /// Message [ZwlrOutputHeadV1PhysicalSizeEvent].
    ZwlrOutputHeadV1PhysicalSize(ZwlrOutputHeadV1PhysicalSizeEvent),
    /// Message [ZwlrOutputHeadV1ModeEvent].
    ZwlrOutputHeadV1Mode(ZwlrOutputHeadV1ModeEvent),
    /// Message [ZwlrOutputHeadV1EnabledEvent].
    ZwlrOutputHeadV1Enabled(ZwlrOutputHeadV1EnabledEvent),
    /// Message [ZwlrOutputHeadV1CurrentModeEvent].
    ZwlrOutputHeadV1CurrentMode(ZwlrOutputHeadV1CurrentModeEvent),
    /// Message [ZwlrOutputHeadV1PositionEvent].
    ZwlrOutputHeadV1Position(ZwlrOutputHeadV1PositionEvent),
    /// Message [ZwlrOutputHeadV1TransformEvent].
    ZwlrOutputHeadV1Transform(ZwlrOutputHeadV1TransformEvent),
    /// Message [ZwlrOutputHeadV1ScaleEvent].
    ZwlrOutputHeadV1Scale(ZwlrOutputHeadV1ScaleEvent),
    /// Message [ZwlrOutputHeadV1FinishedEvent].
    ZwlrOutputHeadV1Finished(ZwlrOutputHeadV1FinishedEvent),
    /// Message [ZwlrOutputHeadV1MakeEvent].
    ZwlrOutputHeadV1Make(ZwlrOutputHeadV1MakeEvent),
    /// Message [ZwlrOutputHeadV1ModelEvent].
    ZwlrOutputHeadV1Model(ZwlrOutputHeadV1ModelEvent),
    /// Message [ZwlrOutputHeadV1SerialNumberEvent].
    ZwlrOutputHeadV1SerialNumber(ZwlrOutputHeadV1SerialNumberEvent),
    /// Message [ZwlrOutputHeadV1AdaptiveSyncEvent].
    ZwlrOutputHeadV1AdaptiveSync(ZwlrOutputHeadV1AdaptiveSyncEvent),
    /// Message [ZwlrOutputModeV1SizeEvent].
    ZwlrOutputModeV1Size(ZwlrOutputModeV1SizeEvent),
    /// Message [ZwlrOutputModeV1RefreshEvent].
    ZwlrOutputModeV1Refresh(ZwlrOutputModeV1RefreshEvent),
    /// Message [ZwlrOutputModeV1PreferredEvent].
    ZwlrOutputModeV1Preferred(ZwlrOutputModeV1PreferredEvent),
    /// Message [ZwlrOutputModeV1FinishedEvent].
    ZwlrOutputModeV1Finished(ZwlrOutputModeV1FinishedEvent),
    /// Message [ZwlrOutputConfigurationV1SucceededEvent].
    ZwlrOutputConfigurationV1Succeeded(ZwlrOutputConfigurationV1SucceededEvent),
    /// Message [ZwlrOutputConfigurationV1FailedEvent].
    ZwlrOutputConfigurationV1Failed(ZwlrOutputConfigurationV1FailedEvent),
    /// Message [ZwlrOutputConfigurationV1CancelledEvent].
    ZwlrOutputConfigurationV1Cancelled(ZwlrOutputConfigurationV1CancelledEvent),
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
            ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputManagerV1 { .. },
            ) => match opcode {
                ZwlrOutputManagerV1HeadEvent::OPCODE => Ok(Self::ZwlrOutputManagerV1Head(
                    ZwlrOutputManagerV1HeadEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrOutputManagerV1DoneEvent::OPCODE => Ok(Self::ZwlrOutputManagerV1Done(
                    ZwlrOutputManagerV1DoneEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrOutputManagerV1FinishedEvent::OPCODE => Ok(Self::ZwlrOutputManagerV1Finished(
                    ZwlrOutputManagerV1FinishedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputHeadV1 { .. },
            ) => match opcode {
                ZwlrOutputHeadV1NameEvent::OPCODE => Ok(Self::ZwlrOutputHeadV1Name(
                    ZwlrOutputHeadV1NameEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrOutputHeadV1DescriptionEvent::OPCODE => Ok(Self::ZwlrOutputHeadV1Description(
                    ZwlrOutputHeadV1DescriptionEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrOutputHeadV1PhysicalSizeEvent::OPCODE => {
                    Ok(Self::ZwlrOutputHeadV1PhysicalSize(
                        ZwlrOutputHeadV1PhysicalSizeEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrOutputHeadV1ModeEvent::OPCODE => Ok(Self::ZwlrOutputHeadV1Mode(
                    ZwlrOutputHeadV1ModeEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrOutputHeadV1EnabledEvent::OPCODE => Ok(Self::ZwlrOutputHeadV1Enabled(
                    ZwlrOutputHeadV1EnabledEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrOutputHeadV1CurrentModeEvent::OPCODE => Ok(Self::ZwlrOutputHeadV1CurrentMode(
                    ZwlrOutputHeadV1CurrentModeEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrOutputHeadV1PositionEvent::OPCODE => Ok(Self::ZwlrOutputHeadV1Position(
                    ZwlrOutputHeadV1PositionEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrOutputHeadV1TransformEvent::OPCODE => Ok(Self::ZwlrOutputHeadV1Transform(
                    ZwlrOutputHeadV1TransformEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrOutputHeadV1ScaleEvent::OPCODE => Ok(Self::ZwlrOutputHeadV1Scale(
                    ZwlrOutputHeadV1ScaleEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrOutputHeadV1FinishedEvent::OPCODE => Ok(Self::ZwlrOutputHeadV1Finished(
                    ZwlrOutputHeadV1FinishedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrOutputHeadV1MakeEvent::OPCODE => Ok(Self::ZwlrOutputHeadV1Make(
                    ZwlrOutputHeadV1MakeEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrOutputHeadV1ModelEvent::OPCODE => Ok(Self::ZwlrOutputHeadV1Model(
                    ZwlrOutputHeadV1ModelEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrOutputHeadV1SerialNumberEvent::OPCODE => {
                    Ok(Self::ZwlrOutputHeadV1SerialNumber(
                        ZwlrOutputHeadV1SerialNumberEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrOutputHeadV1AdaptiveSyncEvent::OPCODE => {
                    Ok(Self::ZwlrOutputHeadV1AdaptiveSync(
                        ZwlrOutputHeadV1AdaptiveSyncEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputModeV1 { .. },
            ) => match opcode {
                ZwlrOutputModeV1SizeEvent::OPCODE => Ok(Self::ZwlrOutputModeV1Size(
                    ZwlrOutputModeV1SizeEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrOutputModeV1RefreshEvent::OPCODE => Ok(Self::ZwlrOutputModeV1Refresh(
                    ZwlrOutputModeV1RefreshEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrOutputModeV1PreferredEvent::OPCODE => Ok(Self::ZwlrOutputModeV1Preferred(
                    ZwlrOutputModeV1PreferredEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrOutputModeV1FinishedEvent::OPCODE => Ok(Self::ZwlrOutputModeV1Finished(
                    ZwlrOutputModeV1FinishedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputConfigurationV1 { .. },
            ) => match opcode {
                ZwlrOutputConfigurationV1SucceededEvent::OPCODE => {
                    Ok(Self::ZwlrOutputConfigurationV1Succeeded(
                        ZwlrOutputConfigurationV1SucceededEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrOutputConfigurationV1FailedEvent::OPCODE => {
                    Ok(Self::ZwlrOutputConfigurationV1Failed(
                        ZwlrOutputConfigurationV1FailedEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrOutputConfigurationV1CancelledEvent::OPCODE => {
                    Ok(Self::ZwlrOutputConfigurationV1Cancelled(
                        ZwlrOutputConfigurationV1CancelledEvent::read_with_interface(
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
impl CommandWriter for CommandEvent {
    fn object_id(&self) -> u32 {
        match self {
            Self::ZwlrOutputManagerV1Head(m) => m.object_id(),
            Self::ZwlrOutputManagerV1Done(m) => m.object_id(),
            Self::ZwlrOutputManagerV1Finished(m) => m.object_id(),
            Self::ZwlrOutputHeadV1Name(m) => m.object_id(),
            Self::ZwlrOutputHeadV1Description(m) => m.object_id(),
            Self::ZwlrOutputHeadV1PhysicalSize(m) => m.object_id(),
            Self::ZwlrOutputHeadV1Mode(m) => m.object_id(),
            Self::ZwlrOutputHeadV1Enabled(m) => m.object_id(),
            Self::ZwlrOutputHeadV1CurrentMode(m) => m.object_id(),
            Self::ZwlrOutputHeadV1Position(m) => m.object_id(),
            Self::ZwlrOutputHeadV1Transform(m) => m.object_id(),
            Self::ZwlrOutputHeadV1Scale(m) => m.object_id(),
            Self::ZwlrOutputHeadV1Finished(m) => m.object_id(),
            Self::ZwlrOutputHeadV1Make(m) => m.object_id(),
            Self::ZwlrOutputHeadV1Model(m) => m.object_id(),
            Self::ZwlrOutputHeadV1SerialNumber(m) => m.object_id(),
            Self::ZwlrOutputHeadV1AdaptiveSync(m) => m.object_id(),
            Self::ZwlrOutputModeV1Size(m) => m.object_id(),
            Self::ZwlrOutputModeV1Refresh(m) => m.object_id(),
            Self::ZwlrOutputModeV1Preferred(m) => m.object_id(),
            Self::ZwlrOutputModeV1Finished(m) => m.object_id(),
            Self::ZwlrOutputConfigurationV1Succeeded(m) => m.object_id(),
            Self::ZwlrOutputConfigurationV1Failed(m) => m.object_id(),
            Self::ZwlrOutputConfigurationV1Cancelled(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwlrOutputManagerV1Head(m) => m.opcode(),
            Self::ZwlrOutputManagerV1Done(m) => m.opcode(),
            Self::ZwlrOutputManagerV1Finished(m) => m.opcode(),
            Self::ZwlrOutputHeadV1Name(m) => m.opcode(),
            Self::ZwlrOutputHeadV1Description(m) => m.opcode(),
            Self::ZwlrOutputHeadV1PhysicalSize(m) => m.opcode(),
            Self::ZwlrOutputHeadV1Mode(m) => m.opcode(),
            Self::ZwlrOutputHeadV1Enabled(m) => m.opcode(),
            Self::ZwlrOutputHeadV1CurrentMode(m) => m.opcode(),
            Self::ZwlrOutputHeadV1Position(m) => m.opcode(),
            Self::ZwlrOutputHeadV1Transform(m) => m.opcode(),
            Self::ZwlrOutputHeadV1Scale(m) => m.opcode(),
            Self::ZwlrOutputHeadV1Finished(m) => m.opcode(),
            Self::ZwlrOutputHeadV1Make(m) => m.opcode(),
            Self::ZwlrOutputHeadV1Model(m) => m.opcode(),
            Self::ZwlrOutputHeadV1SerialNumber(m) => m.opcode(),
            Self::ZwlrOutputHeadV1AdaptiveSync(m) => m.opcode(),
            Self::ZwlrOutputModeV1Size(m) => m.opcode(),
            Self::ZwlrOutputModeV1Refresh(m) => m.opcode(),
            Self::ZwlrOutputModeV1Preferred(m) => m.opcode(),
            Self::ZwlrOutputModeV1Finished(m) => m.opcode(),
            Self::ZwlrOutputConfigurationV1Succeeded(m) => m.opcode(),
            Self::ZwlrOutputConfigurationV1Failed(m) => m.opcode(),
            Self::ZwlrOutputConfigurationV1Cancelled(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwlrOutputManagerV1Head(m) => m.payload_size(),
            Self::ZwlrOutputManagerV1Done(m) => m.payload_size(),
            Self::ZwlrOutputManagerV1Finished(m) => m.payload_size(),
            Self::ZwlrOutputHeadV1Name(m) => m.payload_size(),
            Self::ZwlrOutputHeadV1Description(m) => m.payload_size(),
            Self::ZwlrOutputHeadV1PhysicalSize(m) => m.payload_size(),
            Self::ZwlrOutputHeadV1Mode(m) => m.payload_size(),
            Self::ZwlrOutputHeadV1Enabled(m) => m.payload_size(),
            Self::ZwlrOutputHeadV1CurrentMode(m) => m.payload_size(),
            Self::ZwlrOutputHeadV1Position(m) => m.payload_size(),
            Self::ZwlrOutputHeadV1Transform(m) => m.payload_size(),
            Self::ZwlrOutputHeadV1Scale(m) => m.payload_size(),
            Self::ZwlrOutputHeadV1Finished(m) => m.payload_size(),
            Self::ZwlrOutputHeadV1Make(m) => m.payload_size(),
            Self::ZwlrOutputHeadV1Model(m) => m.payload_size(),
            Self::ZwlrOutputHeadV1SerialNumber(m) => m.payload_size(),
            Self::ZwlrOutputHeadV1AdaptiveSync(m) => m.payload_size(),
            Self::ZwlrOutputModeV1Size(m) => m.payload_size(),
            Self::ZwlrOutputModeV1Refresh(m) => m.payload_size(),
            Self::ZwlrOutputModeV1Preferred(m) => m.payload_size(),
            Self::ZwlrOutputModeV1Finished(m) => m.payload_size(),
            Self::ZwlrOutputConfigurationV1Succeeded(m) => m.payload_size(),
            Self::ZwlrOutputConfigurationV1Failed(m) => m.payload_size(),
            Self::ZwlrOutputConfigurationV1Cancelled(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwlrOutputManagerV1Head(m) => m.write(writer, state).await,
            Self::ZwlrOutputManagerV1Done(m) => m.write(writer, state).await,
            Self::ZwlrOutputManagerV1Finished(m) => m.write(writer, state).await,
            Self::ZwlrOutputHeadV1Name(m) => m.write(writer, state).await,
            Self::ZwlrOutputHeadV1Description(m) => m.write(writer, state).await,
            Self::ZwlrOutputHeadV1PhysicalSize(m) => m.write(writer, state).await,
            Self::ZwlrOutputHeadV1Mode(m) => m.write(writer, state).await,
            Self::ZwlrOutputHeadV1Enabled(m) => m.write(writer, state).await,
            Self::ZwlrOutputHeadV1CurrentMode(m) => m.write(writer, state).await,
            Self::ZwlrOutputHeadV1Position(m) => m.write(writer, state).await,
            Self::ZwlrOutputHeadV1Transform(m) => m.write(writer, state).await,
            Self::ZwlrOutputHeadV1Scale(m) => m.write(writer, state).await,
            Self::ZwlrOutputHeadV1Finished(m) => m.write(writer, state).await,
            Self::ZwlrOutputHeadV1Make(m) => m.write(writer, state).await,
            Self::ZwlrOutputHeadV1Model(m) => m.write(writer, state).await,
            Self::ZwlrOutputHeadV1SerialNumber(m) => m.write(writer, state).await,
            Self::ZwlrOutputHeadV1AdaptiveSync(m) => m.write(writer, state).await,
            Self::ZwlrOutputModeV1Size(m) => m.write(writer, state).await,
            Self::ZwlrOutputModeV1Refresh(m) => m.write(writer, state).await,
            Self::ZwlrOutputModeV1Preferred(m) => m.write(writer, state).await,
            Self::ZwlrOutputModeV1Finished(m) => m.write(writer, state).await,
            Self::ZwlrOutputConfigurationV1Succeeded(m) => m.write(writer, state).await,
            Self::ZwlrOutputConfigurationV1Failed(m) => m.write(writer, state).await,
            Self::ZwlrOutputConfigurationV1Cancelled(m) => m.write(writer, state).await,
        }
    }
}
