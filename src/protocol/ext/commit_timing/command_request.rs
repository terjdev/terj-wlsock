use super::{
    WpCommitTimerV1DestroyRequest, WpCommitTimerV1SetTimestampRequest,
    WpCommitTimingManagerV1DestroyRequest, WpCommitTimingManagerV1GetTimerRequest,
};
use crate::protocol::ext::commit_timing::ObjectInteface as CommitTimingObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Commit timing requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [WpCommitTimingManagerV1DestroyRequest].
    WpCommitTimingManagerV1Destroy(WpCommitTimingManagerV1DestroyRequest),
    /// Message [WpCommitTimingManagerV1GetTimerRequest].
    WpCommitTimingManagerV1GetTimer(WpCommitTimingManagerV1GetTimerRequest),
    /// Message [WpCommitTimerV1SetTimestampRequest].
    WpCommitTimerV1SetTimestamp(WpCommitTimerV1SetTimestampRequest),
    /// Message [WpCommitTimerV1DestroyRequest].
    WpCommitTimerV1Destroy(WpCommitTimerV1DestroyRequest),
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
            ObjectInteface::CommitTiming(CommitTimingObjectInteface::WpCommitTimingManagerV1 {
                ..
            }) => match opcode {
                WpCommitTimingManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::WpCommitTimingManagerV1Destroy(
                        WpCommitTimingManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WpCommitTimingManagerV1GetTimerRequest::OPCODE => {
                    Ok(Self::WpCommitTimingManagerV1GetTimer(
                        WpCommitTimingManagerV1GetTimerRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::CommitTiming(CommitTimingObjectInteface::WpCommitTimerV1 {
                ..
            }) => match opcode {
                WpCommitTimerV1SetTimestampRequest::OPCODE => {
                    Ok(Self::WpCommitTimerV1SetTimestamp(
                        WpCommitTimerV1SetTimestampRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WpCommitTimerV1DestroyRequest::OPCODE => Ok(Self::WpCommitTimerV1Destroy(
                    WpCommitTimerV1DestroyRequest::read_with_interface(
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
            Self::WpCommitTimingManagerV1Destroy(message) => message.object_id(),
            Self::WpCommitTimingManagerV1GetTimer(message) => message.object_id(),
            Self::WpCommitTimerV1SetTimestamp(message) => message.object_id(),
            Self::WpCommitTimerV1Destroy(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WpCommitTimingManagerV1Destroy(message) => message.opcode(),
            Self::WpCommitTimingManagerV1GetTimer(message) => message.opcode(),
            Self::WpCommitTimerV1SetTimestamp(message) => message.opcode(),
            Self::WpCommitTimerV1Destroy(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WpCommitTimingManagerV1Destroy(message) => message.payload_size(),
            Self::WpCommitTimingManagerV1GetTimer(message) => message.payload_size(),
            Self::WpCommitTimerV1SetTimestamp(message) => message.payload_size(),
            Self::WpCommitTimerV1Destroy(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::WpCommitTimingManagerV1Destroy(message) => message.write(writer, state).await,
            Self::WpCommitTimingManagerV1GetTimer(message) => message.write(writer, state).await,
            Self::WpCommitTimerV1SetTimestamp(message) => message.write(writer, state).await,
            Self::WpCommitTimerV1Destroy(message) => message.write(writer, state).await,
        }
    }
}
