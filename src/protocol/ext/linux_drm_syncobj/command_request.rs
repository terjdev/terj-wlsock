use super::{
    WpLinuxDrmSyncobjManagerV1DestroyRequest, WpLinuxDrmSyncobjManagerV1GetSurfaceRequest,
    WpLinuxDrmSyncobjManagerV1ImportTimelineRequest, WpLinuxDrmSyncobjSurfaceV1DestroyRequest,
    WpLinuxDrmSyncobjSurfaceV1SetAcquirePointRequest,
    WpLinuxDrmSyncobjSurfaceV1SetReleasePointRequest, WpLinuxDrmSyncobjTimelineV1DestroyRequest,
};
use crate::protocol::ext::linux_drm_syncobj::ObjectInteface as LinuxDrmSyncobjObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Linux drm syncobj requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [WpLinuxDrmSyncobjManagerV1DestroyRequest].
    WpLinuxDrmSyncobjManagerV1Destroy(WpLinuxDrmSyncobjManagerV1DestroyRequest),
    /// Message [WpLinuxDrmSyncobjManagerV1GetSurfaceRequest].
    WpLinuxDrmSyncobjManagerV1GetSurface(WpLinuxDrmSyncobjManagerV1GetSurfaceRequest),
    /// Message [WpLinuxDrmSyncobjManagerV1ImportTimelineRequest].
    WpLinuxDrmSyncobjManagerV1ImportTimeline(WpLinuxDrmSyncobjManagerV1ImportTimelineRequest),
    /// Message [WpLinuxDrmSyncobjTimelineV1DestroyRequest].
    WpLinuxDrmSyncobjTimelineV1Destroy(WpLinuxDrmSyncobjTimelineV1DestroyRequest),
    /// Message [WpLinuxDrmSyncobjSurfaceV1DestroyRequest].
    WpLinuxDrmSyncobjSurfaceV1Destroy(WpLinuxDrmSyncobjSurfaceV1DestroyRequest),
    /// Message [WpLinuxDrmSyncobjSurfaceV1SetAcquirePointRequest].
    WpLinuxDrmSyncobjSurfaceV1SetAcquirePoint(WpLinuxDrmSyncobjSurfaceV1SetAcquirePointRequest),
    /// Message [WpLinuxDrmSyncobjSurfaceV1SetReleasePointRequest].
    WpLinuxDrmSyncobjSurfaceV1SetReleasePoint(WpLinuxDrmSyncobjSurfaceV1SetReleasePointRequest),
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
            ObjectInteface::LinuxDrmSyncobj(
                LinuxDrmSyncobjObjectInteface::WpLinuxDrmSyncobjManagerV1 { .. },
            ) => match opcode {
                WpLinuxDrmSyncobjManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::WpLinuxDrmSyncobjManagerV1Destroy(
                        WpLinuxDrmSyncobjManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WpLinuxDrmSyncobjManagerV1GetSurfaceRequest::OPCODE => {
                    Ok(Self::WpLinuxDrmSyncobjManagerV1GetSurface(
                        WpLinuxDrmSyncobjManagerV1GetSurfaceRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WpLinuxDrmSyncobjManagerV1ImportTimelineRequest::OPCODE => {
                    Ok(Self::WpLinuxDrmSyncobjManagerV1ImportTimeline(
                        WpLinuxDrmSyncobjManagerV1ImportTimelineRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::LinuxDrmSyncobj(
                LinuxDrmSyncobjObjectInteface::WpLinuxDrmSyncobjTimelineV1 { .. },
            ) => match opcode {
                WpLinuxDrmSyncobjTimelineV1DestroyRequest::OPCODE => {
                    Ok(Self::WpLinuxDrmSyncobjTimelineV1Destroy(
                        WpLinuxDrmSyncobjTimelineV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::LinuxDrmSyncobj(
                LinuxDrmSyncobjObjectInteface::WpLinuxDrmSyncobjSurfaceV1 { .. },
            ) => match opcode {
                WpLinuxDrmSyncobjSurfaceV1DestroyRequest::OPCODE => {
                    Ok(Self::WpLinuxDrmSyncobjSurfaceV1Destroy(
                        WpLinuxDrmSyncobjSurfaceV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WpLinuxDrmSyncobjSurfaceV1SetAcquirePointRequest::OPCODE => {
                    Ok(Self::WpLinuxDrmSyncobjSurfaceV1SetAcquirePoint(
                        WpLinuxDrmSyncobjSurfaceV1SetAcquirePointRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WpLinuxDrmSyncobjSurfaceV1SetReleasePointRequest::OPCODE => {
                    Ok(Self::WpLinuxDrmSyncobjSurfaceV1SetReleasePoint(
                        WpLinuxDrmSyncobjSurfaceV1SetReleasePointRequest::read_with_interface(
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
            Self::WpLinuxDrmSyncobjManagerV1Destroy(message) => message.object_id(),
            Self::WpLinuxDrmSyncobjManagerV1GetSurface(message) => message.object_id(),
            Self::WpLinuxDrmSyncobjManagerV1ImportTimeline(message) => message.object_id(),
            Self::WpLinuxDrmSyncobjTimelineV1Destroy(message) => message.object_id(),
            Self::WpLinuxDrmSyncobjSurfaceV1Destroy(message) => message.object_id(),
            Self::WpLinuxDrmSyncobjSurfaceV1SetAcquirePoint(message) => message.object_id(),
            Self::WpLinuxDrmSyncobjSurfaceV1SetReleasePoint(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WpLinuxDrmSyncobjManagerV1Destroy(message) => message.opcode(),
            Self::WpLinuxDrmSyncobjManagerV1GetSurface(message) => message.opcode(),
            Self::WpLinuxDrmSyncobjManagerV1ImportTimeline(message) => message.opcode(),
            Self::WpLinuxDrmSyncobjTimelineV1Destroy(message) => message.opcode(),
            Self::WpLinuxDrmSyncobjSurfaceV1Destroy(message) => message.opcode(),
            Self::WpLinuxDrmSyncobjSurfaceV1SetAcquirePoint(message) => message.opcode(),
            Self::WpLinuxDrmSyncobjSurfaceV1SetReleasePoint(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WpLinuxDrmSyncobjManagerV1Destroy(message) => message.payload_size(),
            Self::WpLinuxDrmSyncobjManagerV1GetSurface(message) => message.payload_size(),
            Self::WpLinuxDrmSyncobjManagerV1ImportTimeline(message) => message.payload_size(),
            Self::WpLinuxDrmSyncobjTimelineV1Destroy(message) => message.payload_size(),
            Self::WpLinuxDrmSyncobjSurfaceV1Destroy(message) => message.payload_size(),
            Self::WpLinuxDrmSyncobjSurfaceV1SetAcquirePoint(message) => message.payload_size(),
            Self::WpLinuxDrmSyncobjSurfaceV1SetReleasePoint(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::WpLinuxDrmSyncobjManagerV1Destroy(message) => message.write(writer, state).await,
            Self::WpLinuxDrmSyncobjManagerV1GetSurface(message) => {
                message.write(writer, state).await
            }
            Self::WpLinuxDrmSyncobjManagerV1ImportTimeline(message) => {
                message.write(writer, state).await
            }
            Self::WpLinuxDrmSyncobjTimelineV1Destroy(message) => message.write(writer, state).await,
            Self::WpLinuxDrmSyncobjSurfaceV1Destroy(message) => message.write(writer, state).await,
            Self::WpLinuxDrmSyncobjSurfaceV1SetAcquirePoint(message) => {
                message.write(writer, state).await
            }
            Self::WpLinuxDrmSyncobjSurfaceV1SetReleasePoint(message) => {
                message.write(writer, state).await
            }
        }
    }
}
