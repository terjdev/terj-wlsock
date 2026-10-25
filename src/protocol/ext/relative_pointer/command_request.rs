use super::ObjectInteface as RelativePointerObjectInteface;
use super::{
    ZwpRelativePointerManagerV1DestroyRequest,
    ZwpRelativePointerManagerV1GetRelativePointerRequest, ZwpRelativePointerV1DestroyRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches relative pointer requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwpRelativePointerManagerV1DestroyRequest].
    ZwpRelativePointerManagerV1Destroy(ZwpRelativePointerManagerV1DestroyRequest),
    /// Message [ZwpRelativePointerManagerV1GetRelativePointerRequest].
    ZwpRelativePointerManagerV1GetRelativePointer(
        ZwpRelativePointerManagerV1GetRelativePointerRequest,
    ),
    /// Message [ZwpRelativePointerV1DestroyRequest].
    ZwpRelativePointerV1Destroy(ZwpRelativePointerV1DestroyRequest),
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
            ObjectInteface::RelativePointer(
                RelativePointerObjectInteface::ZwpRelativePointerManagerV1 { .. },
            ) => match opcode {
                ZwpRelativePointerManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwpRelativePointerManagerV1Destroy(
                        ZwpRelativePointerManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpRelativePointerManagerV1GetRelativePointerRequest::OPCODE => {
                    Ok(Self::ZwpRelativePointerManagerV1GetRelativePointer(
                        ZwpRelativePointerManagerV1GetRelativePointerRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::RelativePointer(
                RelativePointerObjectInteface::ZwpRelativePointerV1 { .. },
            ) => match opcode {
                ZwpRelativePointerV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwpRelativePointerV1Destroy(
                        ZwpRelativePointerV1DestroyRequest::read_with_interface(
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
            Self::ZwpRelativePointerManagerV1Destroy(m) => m.object_id(),
            Self::ZwpRelativePointerManagerV1GetRelativePointer(m) => m.object_id(),
            Self::ZwpRelativePointerV1Destroy(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpRelativePointerManagerV1Destroy(m) => m.opcode(),
            Self::ZwpRelativePointerManagerV1GetRelativePointer(m) => m.opcode(),
            Self::ZwpRelativePointerV1Destroy(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpRelativePointerManagerV1Destroy(m) => m.payload_size(),
            Self::ZwpRelativePointerManagerV1GetRelativePointer(m) => m.payload_size(),
            Self::ZwpRelativePointerV1Destroy(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpRelativePointerManagerV1Destroy(m) => m.write(writer, state).await,
            Self::ZwpRelativePointerManagerV1GetRelativePointer(m) => m.write(writer, state).await,
            Self::ZwpRelativePointerV1Destroy(m) => m.write(writer, state).await,
        }
    }
}
