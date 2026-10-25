use super::ObjectInteface as XxZonesObjectInteface;
use super::{
    XxZoneItemV1ClosedEvent, XxZoneItemV1FrameExtentsEvent, XxZoneItemV1PositionEvent,
    XxZoneItemV1PositionFailedEvent, XxZoneV1DoneEvent, XxZoneV1HandleEvent,
    XxZoneV1ItemBlockedEvent, XxZoneV1ItemEnteredEvent, XxZoneV1ItemLeftEvent, XxZoneV1SizeEvent,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches zones events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [XxZoneItemV1FrameExtentsEvent].
    XxZoneItemV1FrameExtents(XxZoneItemV1FrameExtentsEvent),
    /// Message [XxZoneItemV1PositionEvent].
    XxZoneItemV1Position(XxZoneItemV1PositionEvent),
    /// Message [XxZoneItemV1PositionFailedEvent].
    XxZoneItemV1PositionFailed(XxZoneItemV1PositionFailedEvent),
    /// Message [XxZoneItemV1ClosedEvent].
    XxZoneItemV1Closed(XxZoneItemV1ClosedEvent),
    /// Message [XxZoneV1SizeEvent].
    XxZoneV1Size(XxZoneV1SizeEvent),
    /// Message [XxZoneV1HandleEvent].
    XxZoneV1Handle(XxZoneV1HandleEvent),
    /// Message [XxZoneV1DoneEvent].
    XxZoneV1Done(XxZoneV1DoneEvent),
    /// Message [XxZoneV1ItemBlockedEvent].
    XxZoneV1ItemBlocked(XxZoneV1ItemBlockedEvent),
    /// Message [XxZoneV1ItemEnteredEvent].
    XxZoneV1ItemEntered(XxZoneV1ItemEnteredEvent),
    /// Message [XxZoneV1ItemLeftEvent].
    XxZoneV1ItemLeft(XxZoneV1ItemLeftEvent),
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
            ObjectInteface::XxZones(XxZonesObjectInteface::XxZoneItemV1 { .. }) => match opcode {
                XxZoneItemV1FrameExtentsEvent::OPCODE => Ok(Self::XxZoneItemV1FrameExtents(
                    XxZoneItemV1FrameExtentsEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxZoneItemV1PositionEvent::OPCODE => Ok(Self::XxZoneItemV1Position(
                    XxZoneItemV1PositionEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxZoneItemV1PositionFailedEvent::OPCODE => Ok(Self::XxZoneItemV1PositionFailed(
                    XxZoneItemV1PositionFailedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxZoneItemV1ClosedEvent::OPCODE => Ok(Self::XxZoneItemV1Closed(
                    XxZoneItemV1ClosedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XxZones(XxZonesObjectInteface::XxZoneV1 { .. }) => match opcode {
                XxZoneV1SizeEvent::OPCODE => Ok(Self::XxZoneV1Size(
                    XxZoneV1SizeEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxZoneV1HandleEvent::OPCODE => Ok(Self::XxZoneV1Handle(
                    XxZoneV1HandleEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxZoneV1DoneEvent::OPCODE => Ok(Self::XxZoneV1Done(
                    XxZoneV1DoneEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxZoneV1ItemBlockedEvent::OPCODE => Ok(Self::XxZoneV1ItemBlocked(
                    XxZoneV1ItemBlockedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxZoneV1ItemEnteredEvent::OPCODE => Ok(Self::XxZoneV1ItemEntered(
                    XxZoneV1ItemEnteredEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxZoneV1ItemLeftEvent::OPCODE => Ok(Self::XxZoneV1ItemLeft(
                    XxZoneV1ItemLeftEvent::read_with_interface(
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
            Self::XxZoneItemV1FrameExtents(m) => m.object_id(),
            Self::XxZoneItemV1Position(m) => m.object_id(),
            Self::XxZoneItemV1PositionFailed(m) => m.object_id(),
            Self::XxZoneItemV1Closed(m) => m.object_id(),
            Self::XxZoneV1Size(m) => m.object_id(),
            Self::XxZoneV1Handle(m) => m.object_id(),
            Self::XxZoneV1Done(m) => m.object_id(),
            Self::XxZoneV1ItemBlocked(m) => m.object_id(),
            Self::XxZoneV1ItemEntered(m) => m.object_id(),
            Self::XxZoneV1ItemLeft(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XxZoneItemV1FrameExtents(m) => m.opcode(),
            Self::XxZoneItemV1Position(m) => m.opcode(),
            Self::XxZoneItemV1PositionFailed(m) => m.opcode(),
            Self::XxZoneItemV1Closed(m) => m.opcode(),
            Self::XxZoneV1Size(m) => m.opcode(),
            Self::XxZoneV1Handle(m) => m.opcode(),
            Self::XxZoneV1Done(m) => m.opcode(),
            Self::XxZoneV1ItemBlocked(m) => m.opcode(),
            Self::XxZoneV1ItemEntered(m) => m.opcode(),
            Self::XxZoneV1ItemLeft(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XxZoneItemV1FrameExtents(m) => m.payload_size(),
            Self::XxZoneItemV1Position(m) => m.payload_size(),
            Self::XxZoneItemV1PositionFailed(m) => m.payload_size(),
            Self::XxZoneItemV1Closed(m) => m.payload_size(),
            Self::XxZoneV1Size(m) => m.payload_size(),
            Self::XxZoneV1Handle(m) => m.payload_size(),
            Self::XxZoneV1Done(m) => m.payload_size(),
            Self::XxZoneV1ItemBlocked(m) => m.payload_size(),
            Self::XxZoneV1ItemEntered(m) => m.payload_size(),
            Self::XxZoneV1ItemLeft(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XxZoneItemV1FrameExtents(m) => m.write(writer, state).await,
            Self::XxZoneItemV1Position(m) => m.write(writer, state).await,
            Self::XxZoneItemV1PositionFailed(m) => m.write(writer, state).await,
            Self::XxZoneItemV1Closed(m) => m.write(writer, state).await,
            Self::XxZoneV1Size(m) => m.write(writer, state).await,
            Self::XxZoneV1Handle(m) => m.write(writer, state).await,
            Self::XxZoneV1Done(m) => m.write(writer, state).await,
            Self::XxZoneV1ItemBlocked(m) => m.write(writer, state).await,
            Self::XxZoneV1ItemEntered(m) => m.write(writer, state).await,
            Self::XxZoneV1ItemLeft(m) => m.write(writer, state).await,
        }
    }
}
