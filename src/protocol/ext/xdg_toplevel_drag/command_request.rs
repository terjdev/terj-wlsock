use super::{
    XdgToplevelDragManagerV1DestroyRequest, XdgToplevelDragManagerV1GetXdgToplevelDragRequest,
    XdgToplevelDragV1AttachRequest, XdgToplevelDragV1DestroyRequest,
};
use crate::protocol::ext::xdg_toplevel_drag::ObjectInteface as XdgToplevelDragObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Xdg toplevel drag requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [XdgToplevelDragManagerV1DestroyRequest].
    XdgToplevelDragManagerV1Destroy(XdgToplevelDragManagerV1DestroyRequest),
    /// Message [XdgToplevelDragManagerV1GetXdgToplevelDragRequest].
    XdgToplevelDragManagerV1GetXdgToplevelDrag(XdgToplevelDragManagerV1GetXdgToplevelDragRequest),
    /// Message [XdgToplevelDragV1DestroyRequest].
    XdgToplevelDragV1Destroy(XdgToplevelDragV1DestroyRequest),
    /// Message [XdgToplevelDragV1AttachRequest].
    XdgToplevelDragV1Attach(XdgToplevelDragV1AttachRequest),
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
            ObjectInteface::XdgToplevelDrag(
                XdgToplevelDragObjectInteface::XdgToplevelDragManagerV1 { .. },
            ) => match opcode {
                XdgToplevelDragManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::XdgToplevelDragManagerV1Destroy(
                        XdgToplevelDragManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XdgToplevelDragManagerV1GetXdgToplevelDragRequest::OPCODE => {
                    Ok(Self::XdgToplevelDragManagerV1GetXdgToplevelDrag(
                        XdgToplevelDragManagerV1GetXdgToplevelDragRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XdgToplevelDrag(XdgToplevelDragObjectInteface::XdgToplevelDragV1 {
                ..
            }) => match opcode {
                XdgToplevelDragV1DestroyRequest::OPCODE => Ok(Self::XdgToplevelDragV1Destroy(
                    XdgToplevelDragV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgToplevelDragV1AttachRequest::OPCODE => Ok(Self::XdgToplevelDragV1Attach(
                    XdgToplevelDragV1AttachRequest::read_with_interface(
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
            Self::XdgToplevelDragManagerV1Destroy(message) => message.object_id(),
            Self::XdgToplevelDragManagerV1GetXdgToplevelDrag(message) => message.object_id(),
            Self::XdgToplevelDragV1Destroy(message) => message.object_id(),
            Self::XdgToplevelDragV1Attach(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XdgToplevelDragManagerV1Destroy(message) => message.opcode(),
            Self::XdgToplevelDragManagerV1GetXdgToplevelDrag(message) => message.opcode(),
            Self::XdgToplevelDragV1Destroy(message) => message.opcode(),
            Self::XdgToplevelDragV1Attach(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XdgToplevelDragManagerV1Destroy(message) => message.payload_size(),
            Self::XdgToplevelDragManagerV1GetXdgToplevelDrag(message) => message.payload_size(),
            Self::XdgToplevelDragV1Destroy(message) => message.payload_size(),
            Self::XdgToplevelDragV1Attach(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XdgToplevelDragManagerV1Destroy(message) => message.write(writer, state).await,
            Self::XdgToplevelDragManagerV1GetXdgToplevelDrag(message) => {
                message.write(writer, state).await
            }
            Self::XdgToplevelDragV1Destroy(message) => message.write(writer, state).await,
            Self::XdgToplevelDragV1Attach(message) => message.write(writer, state).await,
        }
    }
}
