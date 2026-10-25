use crate::protocol::ext::viewporter::ObjectInteface as ViewporterObjectInteface;
use crate::protocol::ext::viewporter::{
    WpViewportDestroyRequest, WpViewportSetDestinationRequest, WpViewportSetSourceRequest,
    WpViewporterDestroyRequest, WpViewporterGetViewportRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Contains Viewporter requests and delegates their encoding and decoding.
#[derive(Debug)]
pub enum CommandRequest {
    /// Request [WpViewporterDestroyRequest].
    WpViewporterDestroy(WpViewporterDestroyRequest),
    /// Request [WpViewporterGetViewportRequest].
    WpViewporterGetViewport(WpViewporterGetViewportRequest),
    /// Request [WpViewportDestroyRequest].
    WpViewportDestroy(WpViewportDestroyRequest),
    /// Request [WpViewportSetSourceRequest].
    WpViewportSetSource(WpViewportSetSourceRequest),
    /// Request [WpViewportSetDestinationRequest].
    WpViewportSetDestination(WpViewportSetDestinationRequest),
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
            ObjectInteface::Viewporter(ViewporterObjectInteface::WpViewporter { .. }) => {
                match opcode {
                    WpViewporterDestroyRequest::OPCODE => Ok(Self::WpViewporterDestroy(
                        WpViewporterDestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    WpViewporterGetViewportRequest::OPCODE => Ok(Self::WpViewporterGetViewport(
                        WpViewporterGetViewportRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::Viewporter(ViewporterObjectInteface::WpViewport { .. }) => match opcode
            {
                WpViewportDestroyRequest::OPCODE => Ok(Self::WpViewportDestroy(
                    WpViewportDestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WpViewportSetSourceRequest::OPCODE => Ok(Self::WpViewportSetSource(
                    WpViewportSetSourceRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WpViewportSetDestinationRequest::OPCODE => Ok(Self::WpViewportSetDestination(
                    WpViewportSetDestinationRequest::read_with_interface(
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
            Self::WpViewporterDestroy(request) => request.object_id(),
            Self::WpViewporterGetViewport(request) => request.object_id(),
            Self::WpViewportDestroy(request) => request.object_id(),
            Self::WpViewportSetSource(request) => request.object_id(),
            Self::WpViewportSetDestination(request) => request.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WpViewporterDestroy(request) => request.opcode(),
            Self::WpViewporterGetViewport(request) => request.opcode(),
            Self::WpViewportDestroy(request) => request.opcode(),
            Self::WpViewportSetSource(request) => request.opcode(),
            Self::WpViewportSetDestination(request) => request.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WpViewporterDestroy(request) => request.payload_size(),
            Self::WpViewporterGetViewport(request) => request.payload_size(),
            Self::WpViewportDestroy(request) => request.payload_size(),
            Self::WpViewportSetSource(request) => request.payload_size(),
            Self::WpViewportSetDestination(request) => request.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::WpViewporterDestroy(request) => request.write(writer, state).await,
            Self::WpViewporterGetViewport(request) => request.write(writer, state).await,
            Self::WpViewportDestroy(request) => request.write(writer, state).await,
            Self::WpViewportSetSource(request) => request.write(writer, state).await,
            Self::WpViewportSetDestination(request) => request.write(writer, state).await,
        }
    }
}
