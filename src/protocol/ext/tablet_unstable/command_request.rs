use super::ObjectInteface as TabletUnstableObjectInteface;
use super::{
    ZwpTabletManagerV1DestroyRequest, ZwpTabletManagerV1GetTabletSeatRequest,
    ZwpTabletSeatV1DestroyRequest, ZwpTabletToolV1DestroyRequest, ZwpTabletToolV1SetCursorRequest,
    ZwpTabletV1DestroyRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches tablet unstable requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwpTabletManagerV1GetTabletSeatRequest].
    ZwpTabletManagerV1GetTabletSeat(ZwpTabletManagerV1GetTabletSeatRequest),
    /// Message [ZwpTabletManagerV1DestroyRequest].
    ZwpTabletManagerV1Destroy(ZwpTabletManagerV1DestroyRequest),
    /// Message [ZwpTabletSeatV1DestroyRequest].
    ZwpTabletSeatV1Destroy(ZwpTabletSeatV1DestroyRequest),
    /// Message [ZwpTabletToolV1SetCursorRequest].
    ZwpTabletToolV1SetCursor(ZwpTabletToolV1SetCursorRequest),
    /// Message [ZwpTabletToolV1DestroyRequest].
    ZwpTabletToolV1Destroy(ZwpTabletToolV1DestroyRequest),
    /// Message [ZwpTabletV1DestroyRequest].
    ZwpTabletV1Destroy(ZwpTabletV1DestroyRequest),
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
            ObjectInteface::TabletUnstable(TabletUnstableObjectInteface::ZwpTabletManagerV1 {
                ..
            }) => match opcode {
                ZwpTabletManagerV1GetTabletSeatRequest::OPCODE => {
                    Ok(Self::ZwpTabletManagerV1GetTabletSeat(
                        ZwpTabletManagerV1GetTabletSeatRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpTabletManagerV1DestroyRequest::OPCODE => Ok(Self::ZwpTabletManagerV1Destroy(
                    ZwpTabletManagerV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::TabletUnstable(TabletUnstableObjectInteface::ZwpTabletSeatV1 {
                ..
            }) => match opcode {
                ZwpTabletSeatV1DestroyRequest::OPCODE => Ok(Self::ZwpTabletSeatV1Destroy(
                    ZwpTabletSeatV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::TabletUnstable(TabletUnstableObjectInteface::ZwpTabletToolV1 {
                ..
            }) => match opcode {
                ZwpTabletToolV1SetCursorRequest::OPCODE => Ok(Self::ZwpTabletToolV1SetCursor(
                    ZwpTabletToolV1SetCursorRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV1DestroyRequest::OPCODE => Ok(Self::ZwpTabletToolV1Destroy(
                    ZwpTabletToolV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::TabletUnstable(TabletUnstableObjectInteface::ZwpTabletV1 {
                ..
            }) => match opcode {
                ZwpTabletV1DestroyRequest::OPCODE => Ok(Self::ZwpTabletV1Destroy(
                    ZwpTabletV1DestroyRequest::read_with_interface(
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
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::ZwpTabletManagerV1GetTabletSeat(m) => m.object_id(),
            Self::ZwpTabletManagerV1Destroy(m) => m.object_id(),
            Self::ZwpTabletSeatV1Destroy(m) => m.object_id(),
            Self::ZwpTabletToolV1SetCursor(m) => m.object_id(),
            Self::ZwpTabletToolV1Destroy(m) => m.object_id(),
            Self::ZwpTabletV1Destroy(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpTabletManagerV1GetTabletSeat(m) => m.opcode(),
            Self::ZwpTabletManagerV1Destroy(m) => m.opcode(),
            Self::ZwpTabletSeatV1Destroy(m) => m.opcode(),
            Self::ZwpTabletToolV1SetCursor(m) => m.opcode(),
            Self::ZwpTabletToolV1Destroy(m) => m.opcode(),
            Self::ZwpTabletV1Destroy(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpTabletManagerV1GetTabletSeat(m) => m.payload_size(),
            Self::ZwpTabletManagerV1Destroy(m) => m.payload_size(),
            Self::ZwpTabletSeatV1Destroy(m) => m.payload_size(),
            Self::ZwpTabletToolV1SetCursor(m) => m.payload_size(),
            Self::ZwpTabletToolV1Destroy(m) => m.payload_size(),
            Self::ZwpTabletV1Destroy(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpTabletManagerV1GetTabletSeat(m) => m.write(writer, state).await,
            Self::ZwpTabletManagerV1Destroy(m) => m.write(writer, state).await,
            Self::ZwpTabletSeatV1Destroy(m) => m.write(writer, state).await,
            Self::ZwpTabletToolV1SetCursor(m) => m.write(writer, state).await,
            Self::ZwpTabletToolV1Destroy(m) => m.write(writer, state).await,
            Self::ZwpTabletV1Destroy(m) => m.write(writer, state).await,
        }
    }
}
