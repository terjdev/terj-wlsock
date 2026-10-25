use super::ObjectInteface as XxKeyboardFilterObjectInteface;
use super::{
    XxKeyboardFilterManagerV1BindToInputMethodRequest, XxKeyboardFilterManagerV1DestroyRequest,
    XxKeyboardFilterV1DestroyRequest, XxKeyboardFilterV1FilterRequest,
    XxKeyboardFilterV1UnbindRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches keyboard filter requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [XxKeyboardFilterV1UnbindRequest].
    XxKeyboardFilterV1Unbind(XxKeyboardFilterV1UnbindRequest),
    /// Message [XxKeyboardFilterV1FilterRequest].
    XxKeyboardFilterV1Filter(XxKeyboardFilterV1FilterRequest),
    /// Message [XxKeyboardFilterV1DestroyRequest].
    XxKeyboardFilterV1Destroy(XxKeyboardFilterV1DestroyRequest),
    /// Message [XxKeyboardFilterManagerV1BindToInputMethodRequest].
    XxKeyboardFilterManagerV1BindToInputMethod(XxKeyboardFilterManagerV1BindToInputMethodRequest),
    /// Message [XxKeyboardFilterManagerV1DestroyRequest].
    XxKeyboardFilterManagerV1Destroy(XxKeyboardFilterManagerV1DestroyRequest),
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
            ObjectInteface::XxKeyboardFilter(
                XxKeyboardFilterObjectInteface::XxKeyboardFilterV1 { .. },
            ) => match opcode {
                XxKeyboardFilterV1UnbindRequest::OPCODE => Ok(Self::XxKeyboardFilterV1Unbind(
                    XxKeyboardFilterV1UnbindRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxKeyboardFilterV1FilterRequest::OPCODE => Ok(Self::XxKeyboardFilterV1Filter(
                    XxKeyboardFilterV1FilterRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxKeyboardFilterV1DestroyRequest::OPCODE => Ok(Self::XxKeyboardFilterV1Destroy(
                    XxKeyboardFilterV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XxKeyboardFilter(
                XxKeyboardFilterObjectInteface::XxKeyboardFilterManagerV1 { .. },
            ) => match opcode {
                XxKeyboardFilterManagerV1BindToInputMethodRequest::OPCODE => {
                    Ok(Self::XxKeyboardFilterManagerV1BindToInputMethod(
                        XxKeyboardFilterManagerV1BindToInputMethodRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XxKeyboardFilterManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::XxKeyboardFilterManagerV1Destroy(
                        XxKeyboardFilterManagerV1DestroyRequest::read_with_interface(
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
            Self::XxKeyboardFilterV1Unbind(m) => m.object_id(),
            Self::XxKeyboardFilterV1Filter(m) => m.object_id(),
            Self::XxKeyboardFilterV1Destroy(m) => m.object_id(),
            Self::XxKeyboardFilterManagerV1BindToInputMethod(m) => m.object_id(),
            Self::XxKeyboardFilterManagerV1Destroy(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XxKeyboardFilterV1Unbind(m) => m.opcode(),
            Self::XxKeyboardFilterV1Filter(m) => m.opcode(),
            Self::XxKeyboardFilterV1Destroy(m) => m.opcode(),
            Self::XxKeyboardFilterManagerV1BindToInputMethod(m) => m.opcode(),
            Self::XxKeyboardFilterManagerV1Destroy(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XxKeyboardFilterV1Unbind(m) => m.payload_size(),
            Self::XxKeyboardFilterV1Filter(m) => m.payload_size(),
            Self::XxKeyboardFilterV1Destroy(m) => m.payload_size(),
            Self::XxKeyboardFilterManagerV1BindToInputMethod(m) => m.payload_size(),
            Self::XxKeyboardFilterManagerV1Destroy(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XxKeyboardFilterV1Unbind(m) => m.write(writer, state).await,
            Self::XxKeyboardFilterV1Filter(m) => m.write(writer, state).await,
            Self::XxKeyboardFilterV1Destroy(m) => m.write(writer, state).await,
            Self::XxKeyboardFilterManagerV1BindToInputMethod(m) => m.write(writer, state).await,
            Self::XxKeyboardFilterManagerV1Destroy(m) => m.write(writer, state).await,
        }
    }
}
