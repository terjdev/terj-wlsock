use super::{
    XdgToplevelIconManagerV1CreateIconRequest, XdgToplevelIconManagerV1DestroyRequest,
    XdgToplevelIconManagerV1SetIconRequest, XdgToplevelIconV1AddBufferRequest,
    XdgToplevelIconV1DestroyRequest, XdgToplevelIconV1SetNameRequest,
};
use crate::protocol::ext::xdg_toplevel_icon::ObjectInteface as XdgToplevelIconObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Xdg toplevel icon requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [XdgToplevelIconManagerV1DestroyRequest].
    XdgToplevelIconManagerV1Destroy(XdgToplevelIconManagerV1DestroyRequest),
    /// Message [XdgToplevelIconManagerV1CreateIconRequest].
    XdgToplevelIconManagerV1CreateIcon(XdgToplevelIconManagerV1CreateIconRequest),
    /// Message [XdgToplevelIconManagerV1SetIconRequest].
    XdgToplevelIconManagerV1SetIcon(XdgToplevelIconManagerV1SetIconRequest),
    /// Message [XdgToplevelIconV1DestroyRequest].
    XdgToplevelIconV1Destroy(XdgToplevelIconV1DestroyRequest),
    /// Message [XdgToplevelIconV1SetNameRequest].
    XdgToplevelIconV1SetName(XdgToplevelIconV1SetNameRequest),
    /// Message [XdgToplevelIconV1AddBufferRequest].
    XdgToplevelIconV1AddBuffer(XdgToplevelIconV1AddBufferRequest),
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
            ObjectInteface::XdgToplevelIcon(
                XdgToplevelIconObjectInteface::XdgToplevelIconManagerV1 { .. },
            ) => match opcode {
                XdgToplevelIconManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::XdgToplevelIconManagerV1Destroy(
                        XdgToplevelIconManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XdgToplevelIconManagerV1CreateIconRequest::OPCODE => {
                    Ok(Self::XdgToplevelIconManagerV1CreateIcon(
                        XdgToplevelIconManagerV1CreateIconRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XdgToplevelIconManagerV1SetIconRequest::OPCODE => {
                    Ok(Self::XdgToplevelIconManagerV1SetIcon(
                        XdgToplevelIconManagerV1SetIconRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XdgToplevelIcon(XdgToplevelIconObjectInteface::XdgToplevelIconV1 {
                ..
            }) => match opcode {
                XdgToplevelIconV1DestroyRequest::OPCODE => Ok(Self::XdgToplevelIconV1Destroy(
                    XdgToplevelIconV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgToplevelIconV1SetNameRequest::OPCODE => Ok(Self::XdgToplevelIconV1SetName(
                    XdgToplevelIconV1SetNameRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgToplevelIconV1AddBufferRequest::OPCODE => Ok(Self::XdgToplevelIconV1AddBuffer(
                    XdgToplevelIconV1AddBufferRequest::read_with_interface(
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
            Self::XdgToplevelIconManagerV1Destroy(message) => message.object_id(),
            Self::XdgToplevelIconManagerV1CreateIcon(message) => message.object_id(),
            Self::XdgToplevelIconManagerV1SetIcon(message) => message.object_id(),
            Self::XdgToplevelIconV1Destroy(message) => message.object_id(),
            Self::XdgToplevelIconV1SetName(message) => message.object_id(),
            Self::XdgToplevelIconV1AddBuffer(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XdgToplevelIconManagerV1Destroy(message) => message.opcode(),
            Self::XdgToplevelIconManagerV1CreateIcon(message) => message.opcode(),
            Self::XdgToplevelIconManagerV1SetIcon(message) => message.opcode(),
            Self::XdgToplevelIconV1Destroy(message) => message.opcode(),
            Self::XdgToplevelIconV1SetName(message) => message.opcode(),
            Self::XdgToplevelIconV1AddBuffer(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XdgToplevelIconManagerV1Destroy(message) => message.payload_size(),
            Self::XdgToplevelIconManagerV1CreateIcon(message) => message.payload_size(),
            Self::XdgToplevelIconManagerV1SetIcon(message) => message.payload_size(),
            Self::XdgToplevelIconV1Destroy(message) => message.payload_size(),
            Self::XdgToplevelIconV1SetName(message) => message.payload_size(),
            Self::XdgToplevelIconV1AddBuffer(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XdgToplevelIconManagerV1Destroy(message) => message.write(writer, state).await,
            Self::XdgToplevelIconManagerV1CreateIcon(message) => message.write(writer, state).await,
            Self::XdgToplevelIconManagerV1SetIcon(message) => message.write(writer, state).await,
            Self::XdgToplevelIconV1Destroy(message) => message.write(writer, state).await,
            Self::XdgToplevelIconV1SetName(message) => message.write(writer, state).await,
            Self::XdgToplevelIconV1AddBuffer(message) => message.write(writer, state).await,
        }
    }
}
