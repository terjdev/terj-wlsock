use super::ObjectInteface as XxSessionManagementObjectInteface;
use super::{
    XxSessionManagerV1DestroyRequest, XxSessionManagerV1GetSessionRequest,
    XxSessionV1AddToplevelRequest, XxSessionV1DestroyRequest, XxSessionV1RemoveRequest,
    XxSessionV1RestoreToplevelRequest, XxToplevelSessionV1DestroyRequest,
    XxToplevelSessionV1RemoveRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches session management requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [XxSessionManagerV1DestroyRequest].
    XxSessionManagerV1Destroy(XxSessionManagerV1DestroyRequest),
    /// Message [XxSessionManagerV1GetSessionRequest].
    XxSessionManagerV1GetSession(XxSessionManagerV1GetSessionRequest),
    /// Message [XxSessionV1DestroyRequest].
    XxSessionV1Destroy(XxSessionV1DestroyRequest),
    /// Message [XxSessionV1RemoveRequest].
    XxSessionV1Remove(XxSessionV1RemoveRequest),
    /// Message [XxSessionV1AddToplevelRequest].
    XxSessionV1AddToplevel(XxSessionV1AddToplevelRequest),
    /// Message [XxSessionV1RestoreToplevelRequest].
    XxSessionV1RestoreToplevel(XxSessionV1RestoreToplevelRequest),
    /// Message [XxToplevelSessionV1DestroyRequest].
    XxToplevelSessionV1Destroy(XxToplevelSessionV1DestroyRequest),
    /// Message [XxToplevelSessionV1RemoveRequest].
    XxToplevelSessionV1Remove(XxToplevelSessionV1RemoveRequest),
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
            ObjectInteface::XxSessionManagement(
                XxSessionManagementObjectInteface::XxSessionManagerV1 { .. },
            ) => match opcode {
                XxSessionManagerV1DestroyRequest::OPCODE => Ok(Self::XxSessionManagerV1Destroy(
                    XxSessionManagerV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxSessionManagerV1GetSessionRequest::OPCODE => {
                    Ok(Self::XxSessionManagerV1GetSession(
                        XxSessionManagerV1GetSessionRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XxSessionManagement(
                XxSessionManagementObjectInteface::XxSessionV1 { .. },
            ) => match opcode {
                XxSessionV1DestroyRequest::OPCODE => Ok(Self::XxSessionV1Destroy(
                    XxSessionV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxSessionV1RemoveRequest::OPCODE => Ok(Self::XxSessionV1Remove(
                    XxSessionV1RemoveRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxSessionV1AddToplevelRequest::OPCODE => Ok(Self::XxSessionV1AddToplevel(
                    XxSessionV1AddToplevelRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxSessionV1RestoreToplevelRequest::OPCODE => Ok(Self::XxSessionV1RestoreToplevel(
                    XxSessionV1RestoreToplevelRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XxSessionManagement(
                XxSessionManagementObjectInteface::XxToplevelSessionV1 { .. },
            ) => match opcode {
                XxToplevelSessionV1DestroyRequest::OPCODE => Ok(Self::XxToplevelSessionV1Destroy(
                    XxToplevelSessionV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxToplevelSessionV1RemoveRequest::OPCODE => Ok(Self::XxToplevelSessionV1Remove(
                    XxToplevelSessionV1RemoveRequest::read_with_interface(
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
            Self::XxSessionManagerV1Destroy(m) => m.object_id(),
            Self::XxSessionManagerV1GetSession(m) => m.object_id(),
            Self::XxSessionV1Destroy(m) => m.object_id(),
            Self::XxSessionV1Remove(m) => m.object_id(),
            Self::XxSessionV1AddToplevel(m) => m.object_id(),
            Self::XxSessionV1RestoreToplevel(m) => m.object_id(),
            Self::XxToplevelSessionV1Destroy(m) => m.object_id(),
            Self::XxToplevelSessionV1Remove(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XxSessionManagerV1Destroy(m) => m.opcode(),
            Self::XxSessionManagerV1GetSession(m) => m.opcode(),
            Self::XxSessionV1Destroy(m) => m.opcode(),
            Self::XxSessionV1Remove(m) => m.opcode(),
            Self::XxSessionV1AddToplevel(m) => m.opcode(),
            Self::XxSessionV1RestoreToplevel(m) => m.opcode(),
            Self::XxToplevelSessionV1Destroy(m) => m.opcode(),
            Self::XxToplevelSessionV1Remove(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XxSessionManagerV1Destroy(m) => m.payload_size(),
            Self::XxSessionManagerV1GetSession(m) => m.payload_size(),
            Self::XxSessionV1Destroy(m) => m.payload_size(),
            Self::XxSessionV1Remove(m) => m.payload_size(),
            Self::XxSessionV1AddToplevel(m) => m.payload_size(),
            Self::XxSessionV1RestoreToplevel(m) => m.payload_size(),
            Self::XxToplevelSessionV1Destroy(m) => m.payload_size(),
            Self::XxToplevelSessionV1Remove(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XxSessionManagerV1Destroy(m) => m.write(writer, state).await,
            Self::XxSessionManagerV1GetSession(m) => m.write(writer, state).await,
            Self::XxSessionV1Destroy(m) => m.write(writer, state).await,
            Self::XxSessionV1Remove(m) => m.write(writer, state).await,
            Self::XxSessionV1AddToplevel(m) => m.write(writer, state).await,
            Self::XxSessionV1RestoreToplevel(m) => m.write(writer, state).await,
            Self::XxToplevelSessionV1Destroy(m) => m.write(writer, state).await,
            Self::XxToplevelSessionV1Remove(m) => m.write(writer, state).await,
        }
    }
}
