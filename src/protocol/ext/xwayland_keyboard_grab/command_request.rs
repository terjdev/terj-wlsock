use super::ObjectInteface as XwaylandKeyboardGrabObjectInteface;
use super::{
    ZwpXwaylandKeyboardGrabManagerV1DestroyRequest,
    ZwpXwaylandKeyboardGrabManagerV1GrabKeyboardRequest, ZwpXwaylandKeyboardGrabV1DestroyRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches xwayland keyboard grab requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwpXwaylandKeyboardGrabManagerV1DestroyRequest].
    ZwpXwaylandKeyboardGrabManagerV1Destroy(ZwpXwaylandKeyboardGrabManagerV1DestroyRequest),
    /// Message [ZwpXwaylandKeyboardGrabManagerV1GrabKeyboardRequest].
    ZwpXwaylandKeyboardGrabManagerV1GrabKeyboard(
        ZwpXwaylandKeyboardGrabManagerV1GrabKeyboardRequest,
    ),
    /// Message [ZwpXwaylandKeyboardGrabV1DestroyRequest].
    ZwpXwaylandKeyboardGrabV1Destroy(ZwpXwaylandKeyboardGrabV1DestroyRequest),
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
            ObjectInteface::XwaylandKeyboardGrab(
                XwaylandKeyboardGrabObjectInteface::ZwpXwaylandKeyboardGrabManagerV1 { .. },
            ) => match opcode {
                ZwpXwaylandKeyboardGrabManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwpXwaylandKeyboardGrabManagerV1Destroy(
                        ZwpXwaylandKeyboardGrabManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpXwaylandKeyboardGrabManagerV1GrabKeyboardRequest::OPCODE => {
                    Ok(Self::ZwpXwaylandKeyboardGrabManagerV1GrabKeyboard(
                        ZwpXwaylandKeyboardGrabManagerV1GrabKeyboardRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XwaylandKeyboardGrab(
                XwaylandKeyboardGrabObjectInteface::ZwpXwaylandKeyboardGrabV1 { .. },
            ) => match opcode {
                ZwpXwaylandKeyboardGrabV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwpXwaylandKeyboardGrabV1Destroy(
                        ZwpXwaylandKeyboardGrabV1DestroyRequest::read_with_interface(
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
            Self::ZwpXwaylandKeyboardGrabManagerV1Destroy(m) => m.object_id(),
            Self::ZwpXwaylandKeyboardGrabManagerV1GrabKeyboard(m) => m.object_id(),
            Self::ZwpXwaylandKeyboardGrabV1Destroy(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpXwaylandKeyboardGrabManagerV1Destroy(m) => m.opcode(),
            Self::ZwpXwaylandKeyboardGrabManagerV1GrabKeyboard(m) => m.opcode(),
            Self::ZwpXwaylandKeyboardGrabV1Destroy(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpXwaylandKeyboardGrabManagerV1Destroy(m) => m.payload_size(),
            Self::ZwpXwaylandKeyboardGrabManagerV1GrabKeyboard(m) => m.payload_size(),
            Self::ZwpXwaylandKeyboardGrabV1Destroy(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpXwaylandKeyboardGrabManagerV1Destroy(m) => m.write(writer, state).await,
            Self::ZwpXwaylandKeyboardGrabManagerV1GrabKeyboard(m) => m.write(writer, state).await,
            Self::ZwpXwaylandKeyboardGrabV1Destroy(m) => m.write(writer, state).await,
        }
    }
}
