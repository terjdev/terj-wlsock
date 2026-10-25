use super::ObjectInteface as XxHotkeyObjectInteface;
use super::{
    XxHotkeyManagerV1CreateHotkeyRequest, XxHotkeyManagerV1DestroyRequest,
    XxHotkeyManagerV1SetAppIdRequest, XxHotkeyV1CommitRequest, XxHotkeyV1DestroyRequest,
    XxHotkeyV1SetButtonTriggerRequest, XxHotkeyV1SetDescriptionRequest,
    XxHotkeyV1SetKeyTriggerRequest, XxHotkeyV1SetSeatRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches hotkey requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [XxHotkeyManagerV1DestroyRequest].
    XxHotkeyManagerV1Destroy(XxHotkeyManagerV1DestroyRequest),
    /// Message [XxHotkeyManagerV1SetAppIdRequest].
    XxHotkeyManagerV1SetAppId(XxHotkeyManagerV1SetAppIdRequest),
    /// Message [XxHotkeyManagerV1CreateHotkeyRequest].
    XxHotkeyManagerV1CreateHotkey(XxHotkeyManagerV1CreateHotkeyRequest),
    /// Message [XxHotkeyV1DestroyRequest].
    XxHotkeyV1Destroy(XxHotkeyV1DestroyRequest),
    /// Message [XxHotkeyV1SetDescriptionRequest].
    XxHotkeyV1SetDescription(XxHotkeyV1SetDescriptionRequest),
    /// Message [XxHotkeyV1SetSeatRequest].
    XxHotkeyV1SetSeat(XxHotkeyV1SetSeatRequest),
    /// Message [XxHotkeyV1SetKeyTriggerRequest].
    XxHotkeyV1SetKeyTrigger(XxHotkeyV1SetKeyTriggerRequest),
    /// Message [XxHotkeyV1SetButtonTriggerRequest].
    XxHotkeyV1SetButtonTrigger(XxHotkeyV1SetButtonTriggerRequest),
    /// Message [XxHotkeyV1CommitRequest].
    XxHotkeyV1Commit(XxHotkeyV1CommitRequest),
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
            ObjectInteface::XxHotkey(XxHotkeyObjectInteface::XxHotkeyManagerV1 { .. }) => {
                match opcode {
                    XxHotkeyManagerV1DestroyRequest::OPCODE => Ok(Self::XxHotkeyManagerV1Destroy(
                        XxHotkeyManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XxHotkeyManagerV1SetAppIdRequest::OPCODE => {
                        Ok(Self::XxHotkeyManagerV1SetAppId(
                            XxHotkeyManagerV1SetAppIdRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    XxHotkeyManagerV1CreateHotkeyRequest::OPCODE => {
                        Ok(Self::XxHotkeyManagerV1CreateHotkey(
                            XxHotkeyManagerV1CreateHotkeyRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::XxHotkey(XxHotkeyObjectInteface::XxHotkeyV1 { .. }) => match opcode {
                XxHotkeyV1DestroyRequest::OPCODE => Ok(Self::XxHotkeyV1Destroy(
                    XxHotkeyV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxHotkeyV1SetDescriptionRequest::OPCODE => Ok(Self::XxHotkeyV1SetDescription(
                    XxHotkeyV1SetDescriptionRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxHotkeyV1SetSeatRequest::OPCODE => Ok(Self::XxHotkeyV1SetSeat(
                    XxHotkeyV1SetSeatRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxHotkeyV1SetKeyTriggerRequest::OPCODE => Ok(Self::XxHotkeyV1SetKeyTrigger(
                    XxHotkeyV1SetKeyTriggerRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxHotkeyV1SetButtonTriggerRequest::OPCODE => Ok(Self::XxHotkeyV1SetButtonTrigger(
                    XxHotkeyV1SetButtonTriggerRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxHotkeyV1CommitRequest::OPCODE => Ok(Self::XxHotkeyV1Commit(
                    XxHotkeyV1CommitRequest::read_with_interface(
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
            Self::XxHotkeyManagerV1Destroy(m) => m.object_id(),
            Self::XxHotkeyManagerV1SetAppId(m) => m.object_id(),
            Self::XxHotkeyManagerV1CreateHotkey(m) => m.object_id(),
            Self::XxHotkeyV1Destroy(m) => m.object_id(),
            Self::XxHotkeyV1SetDescription(m) => m.object_id(),
            Self::XxHotkeyV1SetSeat(m) => m.object_id(),
            Self::XxHotkeyV1SetKeyTrigger(m) => m.object_id(),
            Self::XxHotkeyV1SetButtonTrigger(m) => m.object_id(),
            Self::XxHotkeyV1Commit(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XxHotkeyManagerV1Destroy(m) => m.opcode(),
            Self::XxHotkeyManagerV1SetAppId(m) => m.opcode(),
            Self::XxHotkeyManagerV1CreateHotkey(m) => m.opcode(),
            Self::XxHotkeyV1Destroy(m) => m.opcode(),
            Self::XxHotkeyV1SetDescription(m) => m.opcode(),
            Self::XxHotkeyV1SetSeat(m) => m.opcode(),
            Self::XxHotkeyV1SetKeyTrigger(m) => m.opcode(),
            Self::XxHotkeyV1SetButtonTrigger(m) => m.opcode(),
            Self::XxHotkeyV1Commit(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XxHotkeyManagerV1Destroy(m) => m.payload_size(),
            Self::XxHotkeyManagerV1SetAppId(m) => m.payload_size(),
            Self::XxHotkeyManagerV1CreateHotkey(m) => m.payload_size(),
            Self::XxHotkeyV1Destroy(m) => m.payload_size(),
            Self::XxHotkeyV1SetDescription(m) => m.payload_size(),
            Self::XxHotkeyV1SetSeat(m) => m.payload_size(),
            Self::XxHotkeyV1SetKeyTrigger(m) => m.payload_size(),
            Self::XxHotkeyV1SetButtonTrigger(m) => m.payload_size(),
            Self::XxHotkeyV1Commit(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XxHotkeyManagerV1Destroy(m) => m.write(writer, state).await,
            Self::XxHotkeyManagerV1SetAppId(m) => m.write(writer, state).await,
            Self::XxHotkeyManagerV1CreateHotkey(m) => m.write(writer, state).await,
            Self::XxHotkeyV1Destroy(m) => m.write(writer, state).await,
            Self::XxHotkeyV1SetDescription(m) => m.write(writer, state).await,
            Self::XxHotkeyV1SetSeat(m) => m.write(writer, state).await,
            Self::XxHotkeyV1SetKeyTrigger(m) => m.write(writer, state).await,
            Self::XxHotkeyV1SetButtonTrigger(m) => m.write(writer, state).await,
            Self::XxHotkeyV1Commit(m) => m.write(writer, state).await,
        }
    }
}
