use super::ObjectInteface as WlrForeignToplevelManagementObjectInteface;
use super::{
    ZwlrForeignToplevelHandleV1AppIdEvent, ZwlrForeignToplevelHandleV1ClosedEvent,
    ZwlrForeignToplevelHandleV1DoneEvent, ZwlrForeignToplevelHandleV1OutputEnterEvent,
    ZwlrForeignToplevelHandleV1OutputLeaveEvent, ZwlrForeignToplevelHandleV1ParentEvent,
    ZwlrForeignToplevelHandleV1StateEvent, ZwlrForeignToplevelHandleV1TitleEvent,
    ZwlrForeignToplevelManagerV1FinishedEvent, ZwlrForeignToplevelManagerV1ToplevelEvent,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches foreign toplevel management events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZwlrForeignToplevelManagerV1ToplevelEvent].
    ZwlrForeignToplevelManagerV1Toplevel(ZwlrForeignToplevelManagerV1ToplevelEvent),
    /// Message [ZwlrForeignToplevelManagerV1FinishedEvent].
    ZwlrForeignToplevelManagerV1Finished(ZwlrForeignToplevelManagerV1FinishedEvent),
    /// Message [ZwlrForeignToplevelHandleV1TitleEvent].
    ZwlrForeignToplevelHandleV1Title(ZwlrForeignToplevelHandleV1TitleEvent),
    /// Message [ZwlrForeignToplevelHandleV1AppIdEvent].
    ZwlrForeignToplevelHandleV1AppId(ZwlrForeignToplevelHandleV1AppIdEvent),
    /// Message [ZwlrForeignToplevelHandleV1OutputEnterEvent].
    ZwlrForeignToplevelHandleV1OutputEnter(ZwlrForeignToplevelHandleV1OutputEnterEvent),
    /// Message [ZwlrForeignToplevelHandleV1OutputLeaveEvent].
    ZwlrForeignToplevelHandleV1OutputLeave(ZwlrForeignToplevelHandleV1OutputLeaveEvent),
    /// Message [ZwlrForeignToplevelHandleV1StateEvent].
    ZwlrForeignToplevelHandleV1State(ZwlrForeignToplevelHandleV1StateEvent),
    /// Message [ZwlrForeignToplevelHandleV1DoneEvent].
    ZwlrForeignToplevelHandleV1Done(ZwlrForeignToplevelHandleV1DoneEvent),
    /// Message [ZwlrForeignToplevelHandleV1ClosedEvent].
    ZwlrForeignToplevelHandleV1Closed(ZwlrForeignToplevelHandleV1ClosedEvent),
    /// Message [ZwlrForeignToplevelHandleV1ParentEvent].
    ZwlrForeignToplevelHandleV1Parent(ZwlrForeignToplevelHandleV1ParentEvent),
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
            ObjectInteface::WlrForeignToplevelManagement(
                WlrForeignToplevelManagementObjectInteface::ZwlrForeignToplevelManagerV1 { .. },
            ) => match opcode {
                ZwlrForeignToplevelManagerV1ToplevelEvent::OPCODE => {
                    Ok(Self::ZwlrForeignToplevelManagerV1Toplevel(
                        ZwlrForeignToplevelManagerV1ToplevelEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrForeignToplevelManagerV1FinishedEvent::OPCODE => {
                    Ok(Self::ZwlrForeignToplevelManagerV1Finished(
                        ZwlrForeignToplevelManagerV1FinishedEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::WlrForeignToplevelManagement(
                WlrForeignToplevelManagementObjectInteface::ZwlrForeignToplevelHandleV1 { .. },
            ) => match opcode {
                ZwlrForeignToplevelHandleV1TitleEvent::OPCODE => {
                    Ok(Self::ZwlrForeignToplevelHandleV1Title(
                        ZwlrForeignToplevelHandleV1TitleEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrForeignToplevelHandleV1AppIdEvent::OPCODE => {
                    Ok(Self::ZwlrForeignToplevelHandleV1AppId(
                        ZwlrForeignToplevelHandleV1AppIdEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrForeignToplevelHandleV1OutputEnterEvent::OPCODE => {
                    Ok(Self::ZwlrForeignToplevelHandleV1OutputEnter(
                        ZwlrForeignToplevelHandleV1OutputEnterEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrForeignToplevelHandleV1OutputLeaveEvent::OPCODE => {
                    Ok(Self::ZwlrForeignToplevelHandleV1OutputLeave(
                        ZwlrForeignToplevelHandleV1OutputLeaveEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrForeignToplevelHandleV1StateEvent::OPCODE => {
                    Ok(Self::ZwlrForeignToplevelHandleV1State(
                        ZwlrForeignToplevelHandleV1StateEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrForeignToplevelHandleV1DoneEvent::OPCODE => {
                    Ok(Self::ZwlrForeignToplevelHandleV1Done(
                        ZwlrForeignToplevelHandleV1DoneEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrForeignToplevelHandleV1ClosedEvent::OPCODE => {
                    Ok(Self::ZwlrForeignToplevelHandleV1Closed(
                        ZwlrForeignToplevelHandleV1ClosedEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrForeignToplevelHandleV1ParentEvent::OPCODE => {
                    Ok(Self::ZwlrForeignToplevelHandleV1Parent(
                        ZwlrForeignToplevelHandleV1ParentEvent::read_with_interface(
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
            Self::ZwlrForeignToplevelManagerV1Toplevel(m) => m.object_id(),
            Self::ZwlrForeignToplevelManagerV1Finished(m) => m.object_id(),
            Self::ZwlrForeignToplevelHandleV1Title(m) => m.object_id(),
            Self::ZwlrForeignToplevelHandleV1AppId(m) => m.object_id(),
            Self::ZwlrForeignToplevelHandleV1OutputEnter(m) => m.object_id(),
            Self::ZwlrForeignToplevelHandleV1OutputLeave(m) => m.object_id(),
            Self::ZwlrForeignToplevelHandleV1State(m) => m.object_id(),
            Self::ZwlrForeignToplevelHandleV1Done(m) => m.object_id(),
            Self::ZwlrForeignToplevelHandleV1Closed(m) => m.object_id(),
            Self::ZwlrForeignToplevelHandleV1Parent(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwlrForeignToplevelManagerV1Toplevel(m) => m.opcode(),
            Self::ZwlrForeignToplevelManagerV1Finished(m) => m.opcode(),
            Self::ZwlrForeignToplevelHandleV1Title(m) => m.opcode(),
            Self::ZwlrForeignToplevelHandleV1AppId(m) => m.opcode(),
            Self::ZwlrForeignToplevelHandleV1OutputEnter(m) => m.opcode(),
            Self::ZwlrForeignToplevelHandleV1OutputLeave(m) => m.opcode(),
            Self::ZwlrForeignToplevelHandleV1State(m) => m.opcode(),
            Self::ZwlrForeignToplevelHandleV1Done(m) => m.opcode(),
            Self::ZwlrForeignToplevelHandleV1Closed(m) => m.opcode(),
            Self::ZwlrForeignToplevelHandleV1Parent(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwlrForeignToplevelManagerV1Toplevel(m) => m.payload_size(),
            Self::ZwlrForeignToplevelManagerV1Finished(m) => m.payload_size(),
            Self::ZwlrForeignToplevelHandleV1Title(m) => m.payload_size(),
            Self::ZwlrForeignToplevelHandleV1AppId(m) => m.payload_size(),
            Self::ZwlrForeignToplevelHandleV1OutputEnter(m) => m.payload_size(),
            Self::ZwlrForeignToplevelHandleV1OutputLeave(m) => m.payload_size(),
            Self::ZwlrForeignToplevelHandleV1State(m) => m.payload_size(),
            Self::ZwlrForeignToplevelHandleV1Done(m) => m.payload_size(),
            Self::ZwlrForeignToplevelHandleV1Closed(m) => m.payload_size(),
            Self::ZwlrForeignToplevelHandleV1Parent(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwlrForeignToplevelManagerV1Toplevel(m) => m.write(writer, state).await,
            Self::ZwlrForeignToplevelManagerV1Finished(m) => m.write(writer, state).await,
            Self::ZwlrForeignToplevelHandleV1Title(m) => m.write(writer, state).await,
            Self::ZwlrForeignToplevelHandleV1AppId(m) => m.write(writer, state).await,
            Self::ZwlrForeignToplevelHandleV1OutputEnter(m) => m.write(writer, state).await,
            Self::ZwlrForeignToplevelHandleV1OutputLeave(m) => m.write(writer, state).await,
            Self::ZwlrForeignToplevelHandleV1State(m) => m.write(writer, state).await,
            Self::ZwlrForeignToplevelHandleV1Done(m) => m.write(writer, state).await,
            Self::ZwlrForeignToplevelHandleV1Closed(m) => m.write(writer, state).await,
            Self::ZwlrForeignToplevelHandleV1Parent(m) => m.write(writer, state).await,
        }
    }
}
