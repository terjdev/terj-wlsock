use super::{
    XdgToplevelTagManagerV1DestroyRequest, XdgToplevelTagManagerV1SetToplevelDescriptionRequest,
    XdgToplevelTagManagerV1SetToplevelTagRequest,
};
use crate::protocol::ext::xdg_toplevel_tag::ObjectInteface as XdgToplevelTagObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Xdg toplevel tag requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [XdgToplevelTagManagerV1DestroyRequest].
    XdgToplevelTagManagerV1Destroy(XdgToplevelTagManagerV1DestroyRequest),
    /// Message [XdgToplevelTagManagerV1SetToplevelTagRequest].
    XdgToplevelTagManagerV1SetToplevelTag(XdgToplevelTagManagerV1SetToplevelTagRequest),
    /// Message [XdgToplevelTagManagerV1SetToplevelDescriptionRequest].
    XdgToplevelTagManagerV1SetToplevelDescription(
        XdgToplevelTagManagerV1SetToplevelDescriptionRequest,
    ),
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
            ObjectInteface::XdgToplevelTag(
                XdgToplevelTagObjectInteface::XdgToplevelTagManagerV1 { .. },
            ) => match opcode {
                XdgToplevelTagManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::XdgToplevelTagManagerV1Destroy(
                        XdgToplevelTagManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XdgToplevelTagManagerV1SetToplevelTagRequest::OPCODE => {
                    Ok(Self::XdgToplevelTagManagerV1SetToplevelTag(
                        XdgToplevelTagManagerV1SetToplevelTagRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XdgToplevelTagManagerV1SetToplevelDescriptionRequest::OPCODE => {
                    Ok(Self::XdgToplevelTagManagerV1SetToplevelDescription(
                        XdgToplevelTagManagerV1SetToplevelDescriptionRequest::read_with_interface(
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
            Self::XdgToplevelTagManagerV1Destroy(message) => message.object_id(),
            Self::XdgToplevelTagManagerV1SetToplevelTag(message) => message.object_id(),
            Self::XdgToplevelTagManagerV1SetToplevelDescription(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XdgToplevelTagManagerV1Destroy(message) => message.opcode(),
            Self::XdgToplevelTagManagerV1SetToplevelTag(message) => message.opcode(),
            Self::XdgToplevelTagManagerV1SetToplevelDescription(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XdgToplevelTagManagerV1Destroy(message) => message.payload_size(),
            Self::XdgToplevelTagManagerV1SetToplevelTag(message) => message.payload_size(),
            Self::XdgToplevelTagManagerV1SetToplevelDescription(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XdgToplevelTagManagerV1Destroy(message) => message.write(writer, state).await,
            Self::XdgToplevelTagManagerV1SetToplevelTag(message) => {
                message.write(writer, state).await
            }
            Self::XdgToplevelTagManagerV1SetToplevelDescription(message) => {
                message.write(writer, state).await
            }
        }
    }
}
