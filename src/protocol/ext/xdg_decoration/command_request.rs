use super::ObjectInteface as XdgDecorationObjectInteface;
use super::{
    ZxdgDecorationManagerV1DestroyRequest, ZxdgDecorationManagerV1GetToplevelDecorationRequest,
    ZxdgToplevelDecorationV1DestroyRequest, ZxdgToplevelDecorationV1SetModeRequest,
    ZxdgToplevelDecorationV1UnsetModeRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches xdg decoration requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZxdgDecorationManagerV1DestroyRequest].
    ZxdgDecorationManagerV1Destroy(ZxdgDecorationManagerV1DestroyRequest),
    /// Message [ZxdgDecorationManagerV1GetToplevelDecorationRequest].
    ZxdgDecorationManagerV1GetToplevelDecoration(
        ZxdgDecorationManagerV1GetToplevelDecorationRequest,
    ),
    /// Message [ZxdgToplevelDecorationV1DestroyRequest].
    ZxdgToplevelDecorationV1Destroy(ZxdgToplevelDecorationV1DestroyRequest),
    /// Message [ZxdgToplevelDecorationV1SetModeRequest].
    ZxdgToplevelDecorationV1SetMode(ZxdgToplevelDecorationV1SetModeRequest),
    /// Message [ZxdgToplevelDecorationV1UnsetModeRequest].
    ZxdgToplevelDecorationV1UnsetMode(ZxdgToplevelDecorationV1UnsetModeRequest),
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
            ObjectInteface::XdgDecoration(
                XdgDecorationObjectInteface::ZxdgDecorationManagerV1 { .. },
            ) => match opcode {
                ZxdgDecorationManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::ZxdgDecorationManagerV1Destroy(
                        ZxdgDecorationManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZxdgDecorationManagerV1GetToplevelDecorationRequest::OPCODE => {
                    Ok(Self::ZxdgDecorationManagerV1GetToplevelDecoration(
                        ZxdgDecorationManagerV1GetToplevelDecorationRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XdgDecoration(
                XdgDecorationObjectInteface::ZxdgToplevelDecorationV1 { .. },
            ) => match opcode {
                ZxdgToplevelDecorationV1DestroyRequest::OPCODE => {
                    Ok(Self::ZxdgToplevelDecorationV1Destroy(
                        ZxdgToplevelDecorationV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZxdgToplevelDecorationV1SetModeRequest::OPCODE => {
                    Ok(Self::ZxdgToplevelDecorationV1SetMode(
                        ZxdgToplevelDecorationV1SetModeRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZxdgToplevelDecorationV1UnsetModeRequest::OPCODE => {
                    Ok(Self::ZxdgToplevelDecorationV1UnsetMode(
                        ZxdgToplevelDecorationV1UnsetModeRequest::read_with_interface(
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
            Self::ZxdgDecorationManagerV1Destroy(m) => m.object_id(),
            Self::ZxdgDecorationManagerV1GetToplevelDecoration(m) => m.object_id(),
            Self::ZxdgToplevelDecorationV1Destroy(m) => m.object_id(),
            Self::ZxdgToplevelDecorationV1SetMode(m) => m.object_id(),
            Self::ZxdgToplevelDecorationV1UnsetMode(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZxdgDecorationManagerV1Destroy(m) => m.opcode(),
            Self::ZxdgDecorationManagerV1GetToplevelDecoration(m) => m.opcode(),
            Self::ZxdgToplevelDecorationV1Destroy(m) => m.opcode(),
            Self::ZxdgToplevelDecorationV1SetMode(m) => m.opcode(),
            Self::ZxdgToplevelDecorationV1UnsetMode(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZxdgDecorationManagerV1Destroy(m) => m.payload_size(),
            Self::ZxdgDecorationManagerV1GetToplevelDecoration(m) => m.payload_size(),
            Self::ZxdgToplevelDecorationV1Destroy(m) => m.payload_size(),
            Self::ZxdgToplevelDecorationV1SetMode(m) => m.payload_size(),
            Self::ZxdgToplevelDecorationV1UnsetMode(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZxdgDecorationManagerV1Destroy(m) => m.write(writer, state).await,
            Self::ZxdgDecorationManagerV1GetToplevelDecoration(m) => m.write(writer, state).await,
            Self::ZxdgToplevelDecorationV1Destroy(m) => m.write(writer, state).await,
            Self::ZxdgToplevelDecorationV1SetMode(m) => m.write(writer, state).await,
            Self::ZxdgToplevelDecorationV1UnsetMode(m) => m.write(writer, state).await,
        }
    }
}
