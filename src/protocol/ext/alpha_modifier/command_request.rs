use super::{
    WpAlphaModifierSurfaceV1DestroyRequest, WpAlphaModifierSurfaceV1SetMultiplierRequest,
    WpAlphaModifierV1DestroyRequest, WpAlphaModifierV1GetSurfaceRequest,
};
use crate::protocol::ext::alpha_modifier::ObjectInteface as AlphaModifierObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Alpha modifier requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [WpAlphaModifierV1DestroyRequest].
    WpAlphaModifierV1Destroy(WpAlphaModifierV1DestroyRequest),
    /// Message [WpAlphaModifierV1GetSurfaceRequest].
    WpAlphaModifierV1GetSurface(WpAlphaModifierV1GetSurfaceRequest),
    /// Message [WpAlphaModifierSurfaceV1DestroyRequest].
    WpAlphaModifierSurfaceV1Destroy(WpAlphaModifierSurfaceV1DestroyRequest),
    /// Message [WpAlphaModifierSurfaceV1SetMultiplierRequest].
    WpAlphaModifierSurfaceV1SetMultiplier(WpAlphaModifierSurfaceV1SetMultiplierRequest),
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
            ObjectInteface::AlphaModifier(AlphaModifierObjectInteface::WpAlphaModifierV1 {
                ..
            }) => match opcode {
                WpAlphaModifierV1DestroyRequest::OPCODE => Ok(Self::WpAlphaModifierV1Destroy(
                    WpAlphaModifierV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WpAlphaModifierV1GetSurfaceRequest::OPCODE => {
                    Ok(Self::WpAlphaModifierV1GetSurface(
                        WpAlphaModifierV1GetSurfaceRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::AlphaModifier(
                AlphaModifierObjectInteface::WpAlphaModifierSurfaceV1 { .. },
            ) => match opcode {
                WpAlphaModifierSurfaceV1DestroyRequest::OPCODE => {
                    Ok(Self::WpAlphaModifierSurfaceV1Destroy(
                        WpAlphaModifierSurfaceV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WpAlphaModifierSurfaceV1SetMultiplierRequest::OPCODE => {
                    Ok(Self::WpAlphaModifierSurfaceV1SetMultiplier(
                        WpAlphaModifierSurfaceV1SetMultiplierRequest::read_with_interface(
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
            Self::WpAlphaModifierV1Destroy(message) => message.object_id(),
            Self::WpAlphaModifierV1GetSurface(message) => message.object_id(),
            Self::WpAlphaModifierSurfaceV1Destroy(message) => message.object_id(),
            Self::WpAlphaModifierSurfaceV1SetMultiplier(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WpAlphaModifierV1Destroy(message) => message.opcode(),
            Self::WpAlphaModifierV1GetSurface(message) => message.opcode(),
            Self::WpAlphaModifierSurfaceV1Destroy(message) => message.opcode(),
            Self::WpAlphaModifierSurfaceV1SetMultiplier(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WpAlphaModifierV1Destroy(message) => message.payload_size(),
            Self::WpAlphaModifierV1GetSurface(message) => message.payload_size(),
            Self::WpAlphaModifierSurfaceV1Destroy(message) => message.payload_size(),
            Self::WpAlphaModifierSurfaceV1SetMultiplier(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::WpAlphaModifierV1Destroy(message) => message.write(writer, state).await,
            Self::WpAlphaModifierV1GetSurface(message) => message.write(writer, state).await,
            Self::WpAlphaModifierSurfaceV1Destroy(message) => message.write(writer, state).await,
            Self::WpAlphaModifierSurfaceV1SetMultiplier(message) => {
                message.write(writer, state).await
            }
        }
    }
}
