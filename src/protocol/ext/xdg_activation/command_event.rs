use super::XdgActivationTokenV1DoneEvent;
use crate::protocol::ext::xdg_activation::ObjectInteface as XdgActivationObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Xdg activation events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [XdgActivationTokenV1DoneEvent].
    XdgActivationTokenV1Done(XdgActivationTokenV1DoneEvent),
}
impl CommandReader<Self> for CommandEvent {
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
impl InterfaceReader for CommandEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        match interface {
            ObjectInteface::XdgActivation(XdgActivationObjectInteface::XdgActivationTokenV1 {
                ..
            }) => match opcode {
                XdgActivationTokenV1DoneEvent::OPCODE => Ok(Self::XdgActivationTokenV1Done(
                    XdgActivationTokenV1DoneEvent::read_with_interface(
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
impl CommandWriter for CommandEvent {
    fn object_id(&self) -> u32 {
        match self {
            Self::XdgActivationTokenV1Done(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XdgActivationTokenV1Done(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XdgActivationTokenV1Done(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XdgActivationTokenV1Done(message) => message.write(writer, state).await,
        }
    }
}
