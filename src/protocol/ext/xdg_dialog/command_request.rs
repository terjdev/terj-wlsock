use super::{
    XdgDialogV1DestroyRequest, XdgDialogV1SetModalRequest, XdgDialogV1UnsetModalRequest,
    XdgWmDialogV1DestroyRequest, XdgWmDialogV1GetXdgDialogRequest,
};
use crate::protocol::ext::xdg_dialog::ObjectInteface as XdgDialogObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Xdg dialog requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [XdgWmDialogV1DestroyRequest].
    XdgWmDialogV1Destroy(XdgWmDialogV1DestroyRequest),
    /// Message [XdgWmDialogV1GetXdgDialogRequest].
    XdgWmDialogV1GetXdgDialog(XdgWmDialogV1GetXdgDialogRequest),
    /// Message [XdgDialogV1DestroyRequest].
    XdgDialogV1Destroy(XdgDialogV1DestroyRequest),
    /// Message [XdgDialogV1SetModalRequest].
    XdgDialogV1SetModal(XdgDialogV1SetModalRequest),
    /// Message [XdgDialogV1UnsetModalRequest].
    XdgDialogV1UnsetModal(XdgDialogV1UnsetModalRequest),
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
            ObjectInteface::XdgDialog(XdgDialogObjectInteface::XdgWmDialogV1 { .. }) => {
                match opcode {
                    XdgWmDialogV1DestroyRequest::OPCODE => Ok(Self::XdgWmDialogV1Destroy(
                        XdgWmDialogV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XdgWmDialogV1GetXdgDialogRequest::OPCODE => {
                        Ok(Self::XdgWmDialogV1GetXdgDialog(
                            XdgWmDialogV1GetXdgDialogRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::XdgDialog(XdgDialogObjectInteface::XdgDialogV1 { .. }) => {
                match opcode {
                    XdgDialogV1DestroyRequest::OPCODE => Ok(Self::XdgDialogV1Destroy(
                        XdgDialogV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XdgDialogV1SetModalRequest::OPCODE => Ok(Self::XdgDialogV1SetModal(
                        XdgDialogV1SetModalRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XdgDialogV1UnsetModalRequest::OPCODE => Ok(Self::XdgDialogV1UnsetModal(
                        XdgDialogV1UnsetModalRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            _ => Err(FdReaderError::InvalidProtocol),
        }
    }
}
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::XdgWmDialogV1Destroy(message) => message.object_id(),
            Self::XdgWmDialogV1GetXdgDialog(message) => message.object_id(),
            Self::XdgDialogV1Destroy(message) => message.object_id(),
            Self::XdgDialogV1SetModal(message) => message.object_id(),
            Self::XdgDialogV1UnsetModal(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XdgWmDialogV1Destroy(message) => message.opcode(),
            Self::XdgWmDialogV1GetXdgDialog(message) => message.opcode(),
            Self::XdgDialogV1Destroy(message) => message.opcode(),
            Self::XdgDialogV1SetModal(message) => message.opcode(),
            Self::XdgDialogV1UnsetModal(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XdgWmDialogV1Destroy(message) => message.payload_size(),
            Self::XdgWmDialogV1GetXdgDialog(message) => message.payload_size(),
            Self::XdgDialogV1Destroy(message) => message.payload_size(),
            Self::XdgDialogV1SetModal(message) => message.payload_size(),
            Self::XdgDialogV1UnsetModal(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XdgWmDialogV1Destroy(message) => message.write(writer, state).await,
            Self::XdgWmDialogV1GetXdgDialog(message) => message.write(writer, state).await,
            Self::XdgDialogV1Destroy(message) => message.write(writer, state).await,
            Self::XdgDialogV1SetModal(message) => message.write(writer, state).await,
            Self::XdgDialogV1UnsetModal(message) => message.write(writer, state).await,
        }
    }
}
