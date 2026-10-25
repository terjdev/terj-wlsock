use super::ObjectInteface as PointerConstraintsObjectInteface;
use super::{
    ZwpConfinedPointerV1DestroyRequest, ZwpConfinedPointerV1SetRegionRequest,
    ZwpLockedPointerV1DestroyRequest, ZwpLockedPointerV1SetCursorPositionHintRequest,
    ZwpLockedPointerV1SetRegionRequest, ZwpPointerConstraintsV1ConfinePointerRequest,
    ZwpPointerConstraintsV1DestroyRequest, ZwpPointerConstraintsV1LockPointerRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches pointer constraints requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwpPointerConstraintsV1DestroyRequest].
    ZwpPointerConstraintsV1Destroy(ZwpPointerConstraintsV1DestroyRequest),
    /// Message [ZwpPointerConstraintsV1LockPointerRequest].
    ZwpPointerConstraintsV1LockPointer(ZwpPointerConstraintsV1LockPointerRequest),
    /// Message [ZwpPointerConstraintsV1ConfinePointerRequest].
    ZwpPointerConstraintsV1ConfinePointer(ZwpPointerConstraintsV1ConfinePointerRequest),
    /// Message [ZwpLockedPointerV1DestroyRequest].
    ZwpLockedPointerV1Destroy(ZwpLockedPointerV1DestroyRequest),
    /// Message [ZwpLockedPointerV1SetCursorPositionHintRequest].
    ZwpLockedPointerV1SetCursorPositionHint(ZwpLockedPointerV1SetCursorPositionHintRequest),
    /// Message [ZwpLockedPointerV1SetRegionRequest].
    ZwpLockedPointerV1SetRegion(ZwpLockedPointerV1SetRegionRequest),
    /// Message [ZwpConfinedPointerV1DestroyRequest].
    ZwpConfinedPointerV1Destroy(ZwpConfinedPointerV1DestroyRequest),
    /// Message [ZwpConfinedPointerV1SetRegionRequest].
    ZwpConfinedPointerV1SetRegion(ZwpConfinedPointerV1SetRegionRequest),
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
            ObjectInteface::PointerConstraints(
                PointerConstraintsObjectInteface::ZwpPointerConstraintsV1 { .. },
            ) => match opcode {
                ZwpPointerConstraintsV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwpPointerConstraintsV1Destroy(
                        ZwpPointerConstraintsV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpPointerConstraintsV1LockPointerRequest::OPCODE => {
                    Ok(Self::ZwpPointerConstraintsV1LockPointer(
                        ZwpPointerConstraintsV1LockPointerRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpPointerConstraintsV1ConfinePointerRequest::OPCODE => {
                    Ok(Self::ZwpPointerConstraintsV1ConfinePointer(
                        ZwpPointerConstraintsV1ConfinePointerRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::PointerConstraints(
                PointerConstraintsObjectInteface::ZwpLockedPointerV1 { .. },
            ) => match opcode {
                ZwpLockedPointerV1DestroyRequest::OPCODE => Ok(Self::ZwpLockedPointerV1Destroy(
                    ZwpLockedPointerV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpLockedPointerV1SetCursorPositionHintRequest::OPCODE => {
                    Ok(Self::ZwpLockedPointerV1SetCursorPositionHint(
                        ZwpLockedPointerV1SetCursorPositionHintRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpLockedPointerV1SetRegionRequest::OPCODE => {
                    Ok(Self::ZwpLockedPointerV1SetRegion(
                        ZwpLockedPointerV1SetRegionRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::PointerConstraints(
                PointerConstraintsObjectInteface::ZwpConfinedPointerV1 { .. },
            ) => match opcode {
                ZwpConfinedPointerV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwpConfinedPointerV1Destroy(
                        ZwpConfinedPointerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpConfinedPointerV1SetRegionRequest::OPCODE => {
                    Ok(Self::ZwpConfinedPointerV1SetRegion(
                        ZwpConfinedPointerV1SetRegionRequest::read_with_interface(
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
            Self::ZwpPointerConstraintsV1Destroy(m) => m.object_id(),
            Self::ZwpPointerConstraintsV1LockPointer(m) => m.object_id(),
            Self::ZwpPointerConstraintsV1ConfinePointer(m) => m.object_id(),
            Self::ZwpLockedPointerV1Destroy(m) => m.object_id(),
            Self::ZwpLockedPointerV1SetCursorPositionHint(m) => m.object_id(),
            Self::ZwpLockedPointerV1SetRegion(m) => m.object_id(),
            Self::ZwpConfinedPointerV1Destroy(m) => m.object_id(),
            Self::ZwpConfinedPointerV1SetRegion(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpPointerConstraintsV1Destroy(m) => m.opcode(),
            Self::ZwpPointerConstraintsV1LockPointer(m) => m.opcode(),
            Self::ZwpPointerConstraintsV1ConfinePointer(m) => m.opcode(),
            Self::ZwpLockedPointerV1Destroy(m) => m.opcode(),
            Self::ZwpLockedPointerV1SetCursorPositionHint(m) => m.opcode(),
            Self::ZwpLockedPointerV1SetRegion(m) => m.opcode(),
            Self::ZwpConfinedPointerV1Destroy(m) => m.opcode(),
            Self::ZwpConfinedPointerV1SetRegion(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpPointerConstraintsV1Destroy(m) => m.payload_size(),
            Self::ZwpPointerConstraintsV1LockPointer(m) => m.payload_size(),
            Self::ZwpPointerConstraintsV1ConfinePointer(m) => m.payload_size(),
            Self::ZwpLockedPointerV1Destroy(m) => m.payload_size(),
            Self::ZwpLockedPointerV1SetCursorPositionHint(m) => m.payload_size(),
            Self::ZwpLockedPointerV1SetRegion(m) => m.payload_size(),
            Self::ZwpConfinedPointerV1Destroy(m) => m.payload_size(),
            Self::ZwpConfinedPointerV1SetRegion(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpPointerConstraintsV1Destroy(m) => m.write(writer, state).await,
            Self::ZwpPointerConstraintsV1LockPointer(m) => m.write(writer, state).await,
            Self::ZwpPointerConstraintsV1ConfinePointer(m) => m.write(writer, state).await,
            Self::ZwpLockedPointerV1Destroy(m) => m.write(writer, state).await,
            Self::ZwpLockedPointerV1SetCursorPositionHint(m) => m.write(writer, state).await,
            Self::ZwpLockedPointerV1SetRegion(m) => m.write(writer, state).await,
            Self::ZwpConfinedPointerV1Destroy(m) => m.write(writer, state).await,
            Self::ZwpConfinedPointerV1SetRegion(m) => m.write(writer, state).await,
        }
    }
}
