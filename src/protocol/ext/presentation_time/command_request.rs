use crate::protocol::ext::presentation_time::ObjectInteface as PresentationTimeObjectInteface;
use crate::protocol::ext::presentation_time::{
    WpPresentationDestroyRequest, WpPresentationFeedbackRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Contains Presentation time requests and delegates their encoding and decoding.
#[derive(Debug)]
pub enum CommandRequest {
    /// Request [WpPresentationDestroyRequest].
    WpPresentationDestroy(WpPresentationDestroyRequest),
    /// Request [WpPresentationFeedbackRequest].
    WpPresentationFeedback(WpPresentationFeedbackRequest),
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
            ObjectInteface::PresentationTime(PresentationTimeObjectInteface::WpPresentation {
                ..
            }) => match opcode {
                WpPresentationDestroyRequest::OPCODE => Ok(Self::WpPresentationDestroy(
                    WpPresentationDestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WpPresentationFeedbackRequest::OPCODE => Ok(Self::WpPresentationFeedback(
                    WpPresentationFeedbackRequest::read_with_interface(
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
            Self::WpPresentationDestroy(request) => request.object_id(),
            Self::WpPresentationFeedback(request) => request.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WpPresentationDestroy(request) => request.opcode(),
            Self::WpPresentationFeedback(request) => request.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WpPresentationDestroy(request) => request.payload_size(),
            Self::WpPresentationFeedback(request) => request.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::WpPresentationDestroy(request) => request.write(writer, state).await,
            Self::WpPresentationFeedback(request) => request.write(writer, state).await,
        }
    }
}
