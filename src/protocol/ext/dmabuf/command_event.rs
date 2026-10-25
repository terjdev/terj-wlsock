use super::{
    ZwpLinuxBufferParamsV1CreatedEvent, ZwpLinuxBufferParamsV1FailedEvent,
    ZwpLinuxDmabufFeedbackV1DoneEvent, ZwpLinuxDmabufFeedbackV1FormatTableEvent,
    ZwpLinuxDmabufFeedbackV1MainDeviceEvent, ZwpLinuxDmabufFeedbackV1TrancheDoneEvent,
    ZwpLinuxDmabufFeedbackV1TrancheFlagsEvent, ZwpLinuxDmabufFeedbackV1TrancheFormatsEvent,
    ZwpLinuxDmabufFeedbackV1TrancheTargetDeviceEvent, ZwpLinuxDmabufV1FormatEvent,
    ZwpLinuxDmabufV1ModifierEvent,
};
use crate::protocol::ext::dmabuf::ObjectInteface as DmabufObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Linux DMA-BUF events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZwpLinuxDmabufV1FormatEvent].
    ZwpLinuxDmabufV1Format(ZwpLinuxDmabufV1FormatEvent),
    /// Message [ZwpLinuxDmabufV1ModifierEvent].
    ZwpLinuxDmabufV1Modifier(ZwpLinuxDmabufV1ModifierEvent),
    /// Message [ZwpLinuxBufferParamsV1CreatedEvent].
    ZwpLinuxBufferParamsV1Created(ZwpLinuxBufferParamsV1CreatedEvent),
    /// Message [ZwpLinuxBufferParamsV1FailedEvent].
    ZwpLinuxBufferParamsV1Failed(ZwpLinuxBufferParamsV1FailedEvent),
    /// Message [ZwpLinuxDmabufFeedbackV1DoneEvent].
    ZwpLinuxDmabufFeedbackV1Done(ZwpLinuxDmabufFeedbackV1DoneEvent),
    /// Message [ZwpLinuxDmabufFeedbackV1FormatTableEvent].
    ZwpLinuxDmabufFeedbackV1FormatTable(ZwpLinuxDmabufFeedbackV1FormatTableEvent),
    /// Message [ZwpLinuxDmabufFeedbackV1MainDeviceEvent].
    ZwpLinuxDmabufFeedbackV1MainDevice(ZwpLinuxDmabufFeedbackV1MainDeviceEvent),
    /// Message [ZwpLinuxDmabufFeedbackV1TrancheDoneEvent].
    ZwpLinuxDmabufFeedbackV1TrancheDone(ZwpLinuxDmabufFeedbackV1TrancheDoneEvent),
    /// Message [ZwpLinuxDmabufFeedbackV1TrancheTargetDeviceEvent].
    ZwpLinuxDmabufFeedbackV1TrancheTargetDevice(ZwpLinuxDmabufFeedbackV1TrancheTargetDeviceEvent),
    /// Message [ZwpLinuxDmabufFeedbackV1TrancheFormatsEvent].
    ZwpLinuxDmabufFeedbackV1TrancheFormats(ZwpLinuxDmabufFeedbackV1TrancheFormatsEvent),
    /// Message [ZwpLinuxDmabufFeedbackV1TrancheFlagsEvent].
    ZwpLinuxDmabufFeedbackV1TrancheFlags(ZwpLinuxDmabufFeedbackV1TrancheFlagsEvent),
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
            ObjectInteface::Dmabuf(DmabufObjectInteface::ZwpLinuxDmabufV1 { .. }) => match opcode {
                ZwpLinuxDmabufV1FormatEvent::OPCODE => Ok(Self::ZwpLinuxDmabufV1Format(
                    ZwpLinuxDmabufV1FormatEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpLinuxDmabufV1ModifierEvent::OPCODE => Ok(Self::ZwpLinuxDmabufV1Modifier(
                    ZwpLinuxDmabufV1ModifierEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Dmabuf(DmabufObjectInteface::ZwpLinuxBufferParamsV1 { .. }) => {
                match opcode {
                    ZwpLinuxBufferParamsV1CreatedEvent::OPCODE => {
                        Ok(Self::ZwpLinuxBufferParamsV1Created(
                            ZwpLinuxBufferParamsV1CreatedEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpLinuxBufferParamsV1FailedEvent::OPCODE => {
                        Ok(Self::ZwpLinuxBufferParamsV1Failed(
                            ZwpLinuxBufferParamsV1FailedEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::Dmabuf(DmabufObjectInteface::ZwpLinuxDmabufFeedbackV1 { .. }) => {
                match opcode {
                    ZwpLinuxDmabufFeedbackV1DoneEvent::OPCODE => {
                        Ok(Self::ZwpLinuxDmabufFeedbackV1Done(
                            ZwpLinuxDmabufFeedbackV1DoneEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpLinuxDmabufFeedbackV1FormatTableEvent::OPCODE => {
                        Ok(Self::ZwpLinuxDmabufFeedbackV1FormatTable(
                            ZwpLinuxDmabufFeedbackV1FormatTableEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpLinuxDmabufFeedbackV1MainDeviceEvent::OPCODE => {
                        Ok(Self::ZwpLinuxDmabufFeedbackV1MainDevice(
                            ZwpLinuxDmabufFeedbackV1MainDeviceEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpLinuxDmabufFeedbackV1TrancheDoneEvent::OPCODE => {
                        Ok(Self::ZwpLinuxDmabufFeedbackV1TrancheDone(
                            ZwpLinuxDmabufFeedbackV1TrancheDoneEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpLinuxDmabufFeedbackV1TrancheTargetDeviceEvent::OPCODE => {
                        Ok(Self::ZwpLinuxDmabufFeedbackV1TrancheTargetDevice(
                            ZwpLinuxDmabufFeedbackV1TrancheTargetDeviceEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpLinuxDmabufFeedbackV1TrancheFormatsEvent::OPCODE => {
                        Ok(Self::ZwpLinuxDmabufFeedbackV1TrancheFormats(
                            ZwpLinuxDmabufFeedbackV1TrancheFormatsEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpLinuxDmabufFeedbackV1TrancheFlagsEvent::OPCODE => {
                        Ok(Self::ZwpLinuxDmabufFeedbackV1TrancheFlags(
                            ZwpLinuxDmabufFeedbackV1TrancheFlagsEvent::read_with_interface(
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
            Self::ZwpLinuxDmabufV1Format(message) => message.object_id(),
            Self::ZwpLinuxDmabufV1Modifier(message) => message.object_id(),
            Self::ZwpLinuxBufferParamsV1Created(message) => message.object_id(),
            Self::ZwpLinuxBufferParamsV1Failed(message) => message.object_id(),
            Self::ZwpLinuxDmabufFeedbackV1Done(message) => message.object_id(),
            Self::ZwpLinuxDmabufFeedbackV1FormatTable(message) => message.object_id(),
            Self::ZwpLinuxDmabufFeedbackV1MainDevice(message) => message.object_id(),
            Self::ZwpLinuxDmabufFeedbackV1TrancheDone(message) => message.object_id(),
            Self::ZwpLinuxDmabufFeedbackV1TrancheTargetDevice(message) => message.object_id(),
            Self::ZwpLinuxDmabufFeedbackV1TrancheFormats(message) => message.object_id(),
            Self::ZwpLinuxDmabufFeedbackV1TrancheFlags(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpLinuxDmabufV1Format(message) => message.opcode(),
            Self::ZwpLinuxDmabufV1Modifier(message) => message.opcode(),
            Self::ZwpLinuxBufferParamsV1Created(message) => message.opcode(),
            Self::ZwpLinuxBufferParamsV1Failed(message) => message.opcode(),
            Self::ZwpLinuxDmabufFeedbackV1Done(message) => message.opcode(),
            Self::ZwpLinuxDmabufFeedbackV1FormatTable(message) => message.opcode(),
            Self::ZwpLinuxDmabufFeedbackV1MainDevice(message) => message.opcode(),
            Self::ZwpLinuxDmabufFeedbackV1TrancheDone(message) => message.opcode(),
            Self::ZwpLinuxDmabufFeedbackV1TrancheTargetDevice(message) => message.opcode(),
            Self::ZwpLinuxDmabufFeedbackV1TrancheFormats(message) => message.opcode(),
            Self::ZwpLinuxDmabufFeedbackV1TrancheFlags(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpLinuxDmabufV1Format(message) => message.payload_size(),
            Self::ZwpLinuxDmabufV1Modifier(message) => message.payload_size(),
            Self::ZwpLinuxBufferParamsV1Created(message) => message.payload_size(),
            Self::ZwpLinuxBufferParamsV1Failed(message) => message.payload_size(),
            Self::ZwpLinuxDmabufFeedbackV1Done(message) => message.payload_size(),
            Self::ZwpLinuxDmabufFeedbackV1FormatTable(message) => message.payload_size(),
            Self::ZwpLinuxDmabufFeedbackV1MainDevice(message) => message.payload_size(),
            Self::ZwpLinuxDmabufFeedbackV1TrancheDone(message) => message.payload_size(),
            Self::ZwpLinuxDmabufFeedbackV1TrancheTargetDevice(message) => message.payload_size(),
            Self::ZwpLinuxDmabufFeedbackV1TrancheFormats(message) => message.payload_size(),
            Self::ZwpLinuxDmabufFeedbackV1TrancheFlags(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpLinuxDmabufV1Format(message) => message.write(writer, state).await,
            Self::ZwpLinuxDmabufV1Modifier(message) => message.write(writer, state).await,
            Self::ZwpLinuxBufferParamsV1Created(message) => message.write(writer, state).await,
            Self::ZwpLinuxBufferParamsV1Failed(message) => message.write(writer, state).await,
            Self::ZwpLinuxDmabufFeedbackV1Done(message) => message.write(writer, state).await,
            Self::ZwpLinuxDmabufFeedbackV1FormatTable(message) => {
                message.write(writer, state).await
            }
            Self::ZwpLinuxDmabufFeedbackV1MainDevice(message) => message.write(writer, state).await,
            Self::ZwpLinuxDmabufFeedbackV1TrancheDone(message) => {
                message.write(writer, state).await
            }
            Self::ZwpLinuxDmabufFeedbackV1TrancheTargetDevice(message) => {
                message.write(writer, state).await
            }
            Self::ZwpLinuxDmabufFeedbackV1TrancheFormats(message) => {
                message.write(writer, state).await
            }
            Self::ZwpLinuxDmabufFeedbackV1TrancheFlags(message) => {
                message.write(writer, state).await
            }
        }
    }
}
