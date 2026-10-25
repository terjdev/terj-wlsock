use crate::protocol::core;
#[cfg(feature = "ext-presentation-time")]
use crate::protocol::ext::presentation_time;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Groups enabled protocol events by their defining protocol.
#[derive(Debug)]
pub enum CommandEvent {
    /// Core Wayland protocol commands.
    Core(core::CommandEvent),
    /// Presentation time extension commands.
    #[cfg(feature = "ext-presentation-time")]
    PresentationTime(presentation_time::CommandEvent),
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
            ObjectInteface::Core(_) => Ok(Self::Core(
                core::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-presentation-time")]
            ObjectInteface::PresentationTime(_) => Ok(Self::PresentationTime(
                presentation_time::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-viewporter")]
            ObjectInteface::Viewporter(_) => Err(FdReaderError::InvalidProtocol),
        }
    }
}

impl CommandWriter for CommandEvent {
    fn object_id(&self) -> u32 {
        match self {
            Self::Core(command) => command.object_id(),
            #[cfg(feature = "ext-presentation-time")]
            Self::PresentationTime(command) => command.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::Core(command) => command.opcode(),
            #[cfg(feature = "ext-presentation-time")]
            Self::PresentationTime(command) => command.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::Core(command) => command.payload_size(),
            #[cfg(feature = "ext-presentation-time")]
            Self::PresentationTime(command) => command.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::Core(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-presentation-time")]
            Self::PresentationTime(command) => command.write(writer, state).await,
        }
    }
}
