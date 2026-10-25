use super::ObjectInteface as PointerGesturesObjectInteface;
use super::{
    ZwpPointerGestureHoldV1BeginEvent, ZwpPointerGestureHoldV1EndEvent,
    ZwpPointerGesturePinchV1BeginEvent, ZwpPointerGesturePinchV1EndEvent,
    ZwpPointerGesturePinchV1UpdateEvent, ZwpPointerGestureSwipeV1BeginEvent,
    ZwpPointerGestureSwipeV1EndEvent, ZwpPointerGestureSwipeV1UpdateEvent,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches pointer gestures events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZwpPointerGestureSwipeV1BeginEvent].
    ZwpPointerGestureSwipeV1Begin(ZwpPointerGestureSwipeV1BeginEvent),
    /// Message [ZwpPointerGestureSwipeV1UpdateEvent].
    ZwpPointerGestureSwipeV1Update(ZwpPointerGestureSwipeV1UpdateEvent),
    /// Message [ZwpPointerGestureSwipeV1EndEvent].
    ZwpPointerGestureSwipeV1End(ZwpPointerGestureSwipeV1EndEvent),
    /// Message [ZwpPointerGesturePinchV1BeginEvent].
    ZwpPointerGesturePinchV1Begin(ZwpPointerGesturePinchV1BeginEvent),
    /// Message [ZwpPointerGesturePinchV1UpdateEvent].
    ZwpPointerGesturePinchV1Update(ZwpPointerGesturePinchV1UpdateEvent),
    /// Message [ZwpPointerGesturePinchV1EndEvent].
    ZwpPointerGesturePinchV1End(ZwpPointerGesturePinchV1EndEvent),
    /// Message [ZwpPointerGestureHoldV1BeginEvent].
    ZwpPointerGestureHoldV1Begin(ZwpPointerGestureHoldV1BeginEvent),
    /// Message [ZwpPointerGestureHoldV1EndEvent].
    ZwpPointerGestureHoldV1End(ZwpPointerGestureHoldV1EndEvent),
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
            ObjectInteface::PointerGestures(
                PointerGesturesObjectInteface::ZwpPointerGestureSwipeV1 { .. },
            ) => match opcode {
                ZwpPointerGestureSwipeV1BeginEvent::OPCODE => {
                    Ok(Self::ZwpPointerGestureSwipeV1Begin(
                        ZwpPointerGestureSwipeV1BeginEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpPointerGestureSwipeV1UpdateEvent::OPCODE => {
                    Ok(Self::ZwpPointerGestureSwipeV1Update(
                        ZwpPointerGestureSwipeV1UpdateEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpPointerGestureSwipeV1EndEvent::OPCODE => Ok(Self::ZwpPointerGestureSwipeV1End(
                    ZwpPointerGestureSwipeV1EndEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::PointerGestures(
                PointerGesturesObjectInteface::ZwpPointerGesturePinchV1 { .. },
            ) => match opcode {
                ZwpPointerGesturePinchV1BeginEvent::OPCODE => {
                    Ok(Self::ZwpPointerGesturePinchV1Begin(
                        ZwpPointerGesturePinchV1BeginEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpPointerGesturePinchV1UpdateEvent::OPCODE => {
                    Ok(Self::ZwpPointerGesturePinchV1Update(
                        ZwpPointerGesturePinchV1UpdateEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpPointerGesturePinchV1EndEvent::OPCODE => Ok(Self::ZwpPointerGesturePinchV1End(
                    ZwpPointerGesturePinchV1EndEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::PointerGestures(
                PointerGesturesObjectInteface::ZwpPointerGestureHoldV1 { .. },
            ) => match opcode {
                ZwpPointerGestureHoldV1BeginEvent::OPCODE => {
                    Ok(Self::ZwpPointerGestureHoldV1Begin(
                        ZwpPointerGestureHoldV1BeginEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpPointerGestureHoldV1EndEvent::OPCODE => Ok(Self::ZwpPointerGestureHoldV1End(
                    ZwpPointerGestureHoldV1EndEvent::read_with_interface(
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
            Self::ZwpPointerGestureSwipeV1Begin(m) => m.object_id(),
            Self::ZwpPointerGestureSwipeV1Update(m) => m.object_id(),
            Self::ZwpPointerGestureSwipeV1End(m) => m.object_id(),
            Self::ZwpPointerGesturePinchV1Begin(m) => m.object_id(),
            Self::ZwpPointerGesturePinchV1Update(m) => m.object_id(),
            Self::ZwpPointerGesturePinchV1End(m) => m.object_id(),
            Self::ZwpPointerGestureHoldV1Begin(m) => m.object_id(),
            Self::ZwpPointerGestureHoldV1End(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpPointerGestureSwipeV1Begin(m) => m.opcode(),
            Self::ZwpPointerGestureSwipeV1Update(m) => m.opcode(),
            Self::ZwpPointerGestureSwipeV1End(m) => m.opcode(),
            Self::ZwpPointerGesturePinchV1Begin(m) => m.opcode(),
            Self::ZwpPointerGesturePinchV1Update(m) => m.opcode(),
            Self::ZwpPointerGesturePinchV1End(m) => m.opcode(),
            Self::ZwpPointerGestureHoldV1Begin(m) => m.opcode(),
            Self::ZwpPointerGestureHoldV1End(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpPointerGestureSwipeV1Begin(m) => m.payload_size(),
            Self::ZwpPointerGestureSwipeV1Update(m) => m.payload_size(),
            Self::ZwpPointerGestureSwipeV1End(m) => m.payload_size(),
            Self::ZwpPointerGesturePinchV1Begin(m) => m.payload_size(),
            Self::ZwpPointerGesturePinchV1Update(m) => m.payload_size(),
            Self::ZwpPointerGesturePinchV1End(m) => m.payload_size(),
            Self::ZwpPointerGestureHoldV1Begin(m) => m.payload_size(),
            Self::ZwpPointerGestureHoldV1End(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpPointerGestureSwipeV1Begin(m) => m.write(writer, state).await,
            Self::ZwpPointerGestureSwipeV1Update(m) => m.write(writer, state).await,
            Self::ZwpPointerGestureSwipeV1End(m) => m.write(writer, state).await,
            Self::ZwpPointerGesturePinchV1Begin(m) => m.write(writer, state).await,
            Self::ZwpPointerGesturePinchV1Update(m) => m.write(writer, state).await,
            Self::ZwpPointerGesturePinchV1End(m) => m.write(writer, state).await,
            Self::ZwpPointerGestureHoldV1Begin(m) => m.write(writer, state).await,
            Self::ZwpPointerGestureHoldV1End(m) => m.write(writer, state).await,
        }
    }
}
