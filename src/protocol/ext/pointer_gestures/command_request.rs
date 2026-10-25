use super::ObjectInteface as PointerGesturesObjectInteface;
use super::{
    ZwpPointerGestureHoldV1DestroyRequest, ZwpPointerGesturePinchV1DestroyRequest,
    ZwpPointerGestureSwipeV1DestroyRequest, ZwpPointerGesturesV1GetHoldGestureRequest,
    ZwpPointerGesturesV1GetPinchGestureRequest, ZwpPointerGesturesV1GetSwipeGestureRequest,
    ZwpPointerGesturesV1ReleaseRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches pointer gestures requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwpPointerGesturesV1GetSwipeGestureRequest].
    ZwpPointerGesturesV1GetSwipeGesture(ZwpPointerGesturesV1GetSwipeGestureRequest),
    /// Message [ZwpPointerGesturesV1GetPinchGestureRequest].
    ZwpPointerGesturesV1GetPinchGesture(ZwpPointerGesturesV1GetPinchGestureRequest),
    /// Message [ZwpPointerGesturesV1ReleaseRequest].
    ZwpPointerGesturesV1Release(ZwpPointerGesturesV1ReleaseRequest),
    /// Message [ZwpPointerGesturesV1GetHoldGestureRequest].
    ZwpPointerGesturesV1GetHoldGesture(ZwpPointerGesturesV1GetHoldGestureRequest),
    /// Message [ZwpPointerGestureSwipeV1DestroyRequest].
    ZwpPointerGestureSwipeV1Destroy(ZwpPointerGestureSwipeV1DestroyRequest),
    /// Message [ZwpPointerGesturePinchV1DestroyRequest].
    ZwpPointerGesturePinchV1Destroy(ZwpPointerGesturePinchV1DestroyRequest),
    /// Message [ZwpPointerGestureHoldV1DestroyRequest].
    ZwpPointerGestureHoldV1Destroy(ZwpPointerGestureHoldV1DestroyRequest),
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
            ObjectInteface::PointerGestures(
                PointerGesturesObjectInteface::ZwpPointerGesturesV1 { .. },
            ) => match opcode {
                ZwpPointerGesturesV1GetSwipeGestureRequest::OPCODE => {
                    Ok(Self::ZwpPointerGesturesV1GetSwipeGesture(
                        ZwpPointerGesturesV1GetSwipeGestureRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpPointerGesturesV1GetPinchGestureRequest::OPCODE => {
                    Ok(Self::ZwpPointerGesturesV1GetPinchGesture(
                        ZwpPointerGesturesV1GetPinchGestureRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpPointerGesturesV1ReleaseRequest::OPCODE => {
                    Ok(Self::ZwpPointerGesturesV1Release(
                        ZwpPointerGesturesV1ReleaseRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpPointerGesturesV1GetHoldGestureRequest::OPCODE => {
                    Ok(Self::ZwpPointerGesturesV1GetHoldGesture(
                        ZwpPointerGesturesV1GetHoldGestureRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::PointerGestures(
                PointerGesturesObjectInteface::ZwpPointerGestureSwipeV1 { .. },
            ) => match opcode {
                ZwpPointerGestureSwipeV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwpPointerGestureSwipeV1Destroy(
                        ZwpPointerGestureSwipeV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::PointerGestures(
                PointerGesturesObjectInteface::ZwpPointerGesturePinchV1 { .. },
            ) => match opcode {
                ZwpPointerGesturePinchV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwpPointerGesturePinchV1Destroy(
                        ZwpPointerGesturePinchV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::PointerGestures(
                PointerGesturesObjectInteface::ZwpPointerGestureHoldV1 { .. },
            ) => match opcode {
                ZwpPointerGestureHoldV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwpPointerGestureHoldV1Destroy(
                        ZwpPointerGestureHoldV1DestroyRequest::read_with_interface(
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
            Self::ZwpPointerGesturesV1GetSwipeGesture(m) => m.object_id(),
            Self::ZwpPointerGesturesV1GetPinchGesture(m) => m.object_id(),
            Self::ZwpPointerGesturesV1Release(m) => m.object_id(),
            Self::ZwpPointerGesturesV1GetHoldGesture(m) => m.object_id(),
            Self::ZwpPointerGestureSwipeV1Destroy(m) => m.object_id(),
            Self::ZwpPointerGesturePinchV1Destroy(m) => m.object_id(),
            Self::ZwpPointerGestureHoldV1Destroy(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpPointerGesturesV1GetSwipeGesture(m) => m.opcode(),
            Self::ZwpPointerGesturesV1GetPinchGesture(m) => m.opcode(),
            Self::ZwpPointerGesturesV1Release(m) => m.opcode(),
            Self::ZwpPointerGesturesV1GetHoldGesture(m) => m.opcode(),
            Self::ZwpPointerGestureSwipeV1Destroy(m) => m.opcode(),
            Self::ZwpPointerGesturePinchV1Destroy(m) => m.opcode(),
            Self::ZwpPointerGestureHoldV1Destroy(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpPointerGesturesV1GetSwipeGesture(m) => m.payload_size(),
            Self::ZwpPointerGesturesV1GetPinchGesture(m) => m.payload_size(),
            Self::ZwpPointerGesturesV1Release(m) => m.payload_size(),
            Self::ZwpPointerGesturesV1GetHoldGesture(m) => m.payload_size(),
            Self::ZwpPointerGestureSwipeV1Destroy(m) => m.payload_size(),
            Self::ZwpPointerGesturePinchV1Destroy(m) => m.payload_size(),
            Self::ZwpPointerGestureHoldV1Destroy(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpPointerGesturesV1GetSwipeGesture(m) => m.write(writer, state).await,
            Self::ZwpPointerGesturesV1GetPinchGesture(m) => m.write(writer, state).await,
            Self::ZwpPointerGesturesV1Release(m) => m.write(writer, state).await,
            Self::ZwpPointerGesturesV1GetHoldGesture(m) => m.write(writer, state).await,
            Self::ZwpPointerGestureSwipeV1Destroy(m) => m.write(writer, state).await,
            Self::ZwpPointerGesturePinchV1Destroy(m) => m.write(writer, state).await,
            Self::ZwpPointerGestureHoldV1Destroy(m) => m.write(writer, state).await,
        }
    }
}
