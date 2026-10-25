use super::WpFractionalScaleV1PreferredScaleEvent;
use crate::protocol::ext::fractional_scale::ObjectInteface as FractionalScaleObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Fractional scale events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [WpFractionalScaleV1PreferredScaleEvent].
    WpFractionalScaleV1PreferredScale(WpFractionalScaleV1PreferredScaleEvent),
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
            ObjectInteface::FractionalScale(
                FractionalScaleObjectInteface::WpFractionalScaleV1 { .. },
            ) => match opcode {
                WpFractionalScaleV1PreferredScaleEvent::OPCODE => {
                    Ok(Self::WpFractionalScaleV1PreferredScale(
                        WpFractionalScaleV1PreferredScaleEvent::read_with_interface(
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
impl CommandWriter for CommandEvent {
    fn object_id(&self) -> u32 {
        match self {
            Self::WpFractionalScaleV1PreferredScale(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WpFractionalScaleV1PreferredScale(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WpFractionalScaleV1PreferredScale(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::WpFractionalScaleV1PreferredScale(message) => message.write(writer, state).await,
        }
    }
}
