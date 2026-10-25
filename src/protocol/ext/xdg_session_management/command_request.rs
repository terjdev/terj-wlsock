use super::{
    XdgSessionManagerV1DestroyRequest, XdgSessionManagerV1GetSessionRequest,
    XdgSessionV1AddToplevelRequest, XdgSessionV1DestroyRequest, XdgSessionV1RemoveRequest,
    XdgSessionV1RemoveToplevelRequest, XdgSessionV1RestoreToplevelRequest,
    XdgToplevelSessionV1DestroyRequest, XdgToplevelSessionV1RenameRequest,
};
use crate::protocol::ext::xdg_session_management::ObjectInteface as XdgXxSessionManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Xdg session management requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [XdgSessionManagerV1DestroyRequest].
    XdgSessionManagerV1Destroy(XdgSessionManagerV1DestroyRequest),
    /// Message [XdgSessionManagerV1GetSessionRequest].
    XdgSessionManagerV1GetSession(XdgSessionManagerV1GetSessionRequest),
    /// Message [XdgSessionV1DestroyRequest].
    XdgSessionV1Destroy(XdgSessionV1DestroyRequest),
    /// Message [XdgSessionV1RemoveRequest].
    XdgSessionV1Remove(XdgSessionV1RemoveRequest),
    /// Message [XdgSessionV1AddToplevelRequest].
    XdgSessionV1AddToplevel(XdgSessionV1AddToplevelRequest),
    /// Message [XdgSessionV1RestoreToplevelRequest].
    XdgSessionV1RestoreToplevel(XdgSessionV1RestoreToplevelRequest),
    /// Message [XdgSessionV1RemoveToplevelRequest].
    XdgSessionV1RemoveToplevel(XdgSessionV1RemoveToplevelRequest),
    /// Message [XdgToplevelSessionV1DestroyRequest].
    XdgToplevelSessionV1Destroy(XdgToplevelSessionV1DestroyRequest),
    /// Message [XdgToplevelSessionV1RenameRequest].
    XdgToplevelSessionV1Rename(XdgToplevelSessionV1RenameRequest),
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
            ObjectInteface::XdgSessionManagement(
                XdgXxSessionManagementObjectInteface::XdgSessionManagerV1 { .. },
            ) => match opcode {
                XdgSessionManagerV1DestroyRequest::OPCODE => Ok(Self::XdgSessionManagerV1Destroy(
                    XdgSessionManagerV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgSessionManagerV1GetSessionRequest::OPCODE => {
                    Ok(Self::XdgSessionManagerV1GetSession(
                        XdgSessionManagerV1GetSessionRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XdgSessionManagement(
                XdgXxSessionManagementObjectInteface::XdgSessionV1 { .. },
            ) => match opcode {
                XdgSessionV1DestroyRequest::OPCODE => Ok(Self::XdgSessionV1Destroy(
                    XdgSessionV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgSessionV1RemoveRequest::OPCODE => Ok(Self::XdgSessionV1Remove(
                    XdgSessionV1RemoveRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgSessionV1AddToplevelRequest::OPCODE => Ok(Self::XdgSessionV1AddToplevel(
                    XdgSessionV1AddToplevelRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgSessionV1RestoreToplevelRequest::OPCODE => {
                    Ok(Self::XdgSessionV1RestoreToplevel(
                        XdgSessionV1RestoreToplevelRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XdgSessionV1RemoveToplevelRequest::OPCODE => Ok(Self::XdgSessionV1RemoveToplevel(
                    XdgSessionV1RemoveToplevelRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XdgSessionManagement(
                XdgXxSessionManagementObjectInteface::XdgToplevelSessionV1 { .. },
            ) => match opcode {
                XdgToplevelSessionV1DestroyRequest::OPCODE => {
                    Ok(Self::XdgToplevelSessionV1Destroy(
                        XdgToplevelSessionV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XdgToplevelSessionV1RenameRequest::OPCODE => Ok(Self::XdgToplevelSessionV1Rename(
                    XdgToplevelSessionV1RenameRequest::read_with_interface(
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
            Self::XdgSessionManagerV1Destroy(message) => message.object_id(),
            Self::XdgSessionManagerV1GetSession(message) => message.object_id(),
            Self::XdgSessionV1Destroy(message) => message.object_id(),
            Self::XdgSessionV1Remove(message) => message.object_id(),
            Self::XdgSessionV1AddToplevel(message) => message.object_id(),
            Self::XdgSessionV1RestoreToplevel(message) => message.object_id(),
            Self::XdgSessionV1RemoveToplevel(message) => message.object_id(),
            Self::XdgToplevelSessionV1Destroy(message) => message.object_id(),
            Self::XdgToplevelSessionV1Rename(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XdgSessionManagerV1Destroy(message) => message.opcode(),
            Self::XdgSessionManagerV1GetSession(message) => message.opcode(),
            Self::XdgSessionV1Destroy(message) => message.opcode(),
            Self::XdgSessionV1Remove(message) => message.opcode(),
            Self::XdgSessionV1AddToplevel(message) => message.opcode(),
            Self::XdgSessionV1RestoreToplevel(message) => message.opcode(),
            Self::XdgSessionV1RemoveToplevel(message) => message.opcode(),
            Self::XdgToplevelSessionV1Destroy(message) => message.opcode(),
            Self::XdgToplevelSessionV1Rename(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XdgSessionManagerV1Destroy(message) => message.payload_size(),
            Self::XdgSessionManagerV1GetSession(message) => message.payload_size(),
            Self::XdgSessionV1Destroy(message) => message.payload_size(),
            Self::XdgSessionV1Remove(message) => message.payload_size(),
            Self::XdgSessionV1AddToplevel(message) => message.payload_size(),
            Self::XdgSessionV1RestoreToplevel(message) => message.payload_size(),
            Self::XdgSessionV1RemoveToplevel(message) => message.payload_size(),
            Self::XdgToplevelSessionV1Destroy(message) => message.payload_size(),
            Self::XdgToplevelSessionV1Rename(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XdgSessionManagerV1Destroy(message) => message.write(writer, state).await,
            Self::XdgSessionManagerV1GetSession(message) => message.write(writer, state).await,
            Self::XdgSessionV1Destroy(message) => message.write(writer, state).await,
            Self::XdgSessionV1Remove(message) => message.write(writer, state).await,
            Self::XdgSessionV1AddToplevel(message) => message.write(writer, state).await,
            Self::XdgSessionV1RestoreToplevel(message) => message.write(writer, state).await,
            Self::XdgSessionV1RemoveToplevel(message) => message.write(writer, state).await,
            Self::XdgToplevelSessionV1Destroy(message) => message.write(writer, state).await,
            Self::XdgToplevelSessionV1Rename(message) => message.write(writer, state).await,
        }
    }
}
