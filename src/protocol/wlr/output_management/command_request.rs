use super::ObjectInteface as WlrOutputManagementObjectInteface;
use super::{
    ZwlrOutputConfigurationHeadV1SetAdaptiveSyncRequest,
    ZwlrOutputConfigurationHeadV1SetCustomModeRequest, ZwlrOutputConfigurationHeadV1SetModeRequest,
    ZwlrOutputConfigurationHeadV1SetPositionRequest, ZwlrOutputConfigurationHeadV1SetScaleRequest,
    ZwlrOutputConfigurationHeadV1SetTransformRequest, ZwlrOutputConfigurationV1ApplyRequest,
    ZwlrOutputConfigurationV1DestroyRequest, ZwlrOutputConfigurationV1DisableHeadRequest,
    ZwlrOutputConfigurationV1EnableHeadRequest, ZwlrOutputConfigurationV1TestRequest,
    ZwlrOutputHeadV1ReleaseRequest, ZwlrOutputManagerV1CreateConfigurationRequest,
    ZwlrOutputManagerV1StopRequest, ZwlrOutputModeV1ReleaseRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches output management requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwlrOutputManagerV1CreateConfigurationRequest].
    ZwlrOutputManagerV1CreateConfiguration(ZwlrOutputManagerV1CreateConfigurationRequest),
    /// Message [ZwlrOutputManagerV1StopRequest].
    ZwlrOutputManagerV1Stop(ZwlrOutputManagerV1StopRequest),
    /// Message [ZwlrOutputHeadV1ReleaseRequest].
    ZwlrOutputHeadV1Release(ZwlrOutputHeadV1ReleaseRequest),
    /// Message [ZwlrOutputModeV1ReleaseRequest].
    ZwlrOutputModeV1Release(ZwlrOutputModeV1ReleaseRequest),
    /// Message [ZwlrOutputConfigurationV1EnableHeadRequest].
    ZwlrOutputConfigurationV1EnableHead(ZwlrOutputConfigurationV1EnableHeadRequest),
    /// Message [ZwlrOutputConfigurationV1DisableHeadRequest].
    ZwlrOutputConfigurationV1DisableHead(ZwlrOutputConfigurationV1DisableHeadRequest),
    /// Message [ZwlrOutputConfigurationV1ApplyRequest].
    ZwlrOutputConfigurationV1Apply(ZwlrOutputConfigurationV1ApplyRequest),
    /// Message [ZwlrOutputConfigurationV1TestRequest].
    ZwlrOutputConfigurationV1Test(ZwlrOutputConfigurationV1TestRequest),
    /// Message [ZwlrOutputConfigurationV1DestroyRequest].
    ZwlrOutputConfigurationV1Destroy(ZwlrOutputConfigurationV1DestroyRequest),
    /// Message [ZwlrOutputConfigurationHeadV1SetModeRequest].
    ZwlrOutputConfigurationHeadV1SetMode(ZwlrOutputConfigurationHeadV1SetModeRequest),
    /// Message [ZwlrOutputConfigurationHeadV1SetCustomModeRequest].
    ZwlrOutputConfigurationHeadV1SetCustomMode(ZwlrOutputConfigurationHeadV1SetCustomModeRequest),
    /// Message [ZwlrOutputConfigurationHeadV1SetPositionRequest].
    ZwlrOutputConfigurationHeadV1SetPosition(ZwlrOutputConfigurationHeadV1SetPositionRequest),
    /// Message [ZwlrOutputConfigurationHeadV1SetTransformRequest].
    ZwlrOutputConfigurationHeadV1SetTransform(ZwlrOutputConfigurationHeadV1SetTransformRequest),
    /// Message [ZwlrOutputConfigurationHeadV1SetScaleRequest].
    ZwlrOutputConfigurationHeadV1SetScale(ZwlrOutputConfigurationHeadV1SetScaleRequest),
    /// Message [ZwlrOutputConfigurationHeadV1SetAdaptiveSyncRequest].
    ZwlrOutputConfigurationHeadV1SetAdaptiveSync(
        ZwlrOutputConfigurationHeadV1SetAdaptiveSyncRequest,
    ),
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
            ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputManagerV1 { .. },
            ) => match opcode {
                ZwlrOutputManagerV1CreateConfigurationRequest::OPCODE => {
                    Ok(Self::ZwlrOutputManagerV1CreateConfiguration(
                        ZwlrOutputManagerV1CreateConfigurationRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrOutputManagerV1StopRequest::OPCODE => Ok(Self::ZwlrOutputManagerV1Stop(
                    ZwlrOutputManagerV1StopRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputHeadV1 { .. },
            ) => match opcode {
                ZwlrOutputHeadV1ReleaseRequest::OPCODE => Ok(Self::ZwlrOutputHeadV1Release(
                    ZwlrOutputHeadV1ReleaseRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputModeV1 { .. },
            ) => match opcode {
                ZwlrOutputModeV1ReleaseRequest::OPCODE => Ok(Self::ZwlrOutputModeV1Release(
                    ZwlrOutputModeV1ReleaseRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputConfigurationV1 { .. },
            ) => match opcode {
                ZwlrOutputConfigurationV1EnableHeadRequest::OPCODE => {
                    Ok(Self::ZwlrOutputConfigurationV1EnableHead(
                        ZwlrOutputConfigurationV1EnableHeadRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrOutputConfigurationV1DisableHeadRequest::OPCODE => {
                    Ok(Self::ZwlrOutputConfigurationV1DisableHead(
                        ZwlrOutputConfigurationV1DisableHeadRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrOutputConfigurationV1ApplyRequest::OPCODE => {
                    Ok(Self::ZwlrOutputConfigurationV1Apply(
                        ZwlrOutputConfigurationV1ApplyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrOutputConfigurationV1TestRequest::OPCODE => {
                    Ok(Self::ZwlrOutputConfigurationV1Test(
                        ZwlrOutputConfigurationV1TestRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrOutputConfigurationV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwlrOutputConfigurationV1Destroy(
                        ZwlrOutputConfigurationV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::WlrOutputManagement(
                WlrOutputManagementObjectInteface::ZwlrOutputConfigurationHeadV1 { .. },
            ) => match opcode {
                ZwlrOutputConfigurationHeadV1SetModeRequest::OPCODE => {
                    Ok(Self::ZwlrOutputConfigurationHeadV1SetMode(
                        ZwlrOutputConfigurationHeadV1SetModeRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrOutputConfigurationHeadV1SetCustomModeRequest::OPCODE => {
                    Ok(Self::ZwlrOutputConfigurationHeadV1SetCustomMode(
                        ZwlrOutputConfigurationHeadV1SetCustomModeRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrOutputConfigurationHeadV1SetPositionRequest::OPCODE => {
                    Ok(Self::ZwlrOutputConfigurationHeadV1SetPosition(
                        ZwlrOutputConfigurationHeadV1SetPositionRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrOutputConfigurationHeadV1SetTransformRequest::OPCODE => {
                    Ok(Self::ZwlrOutputConfigurationHeadV1SetTransform(
                        ZwlrOutputConfigurationHeadV1SetTransformRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrOutputConfigurationHeadV1SetScaleRequest::OPCODE => {
                    Ok(Self::ZwlrOutputConfigurationHeadV1SetScale(
                        ZwlrOutputConfigurationHeadV1SetScaleRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrOutputConfigurationHeadV1SetAdaptiveSyncRequest::OPCODE => {
                    Ok(Self::ZwlrOutputConfigurationHeadV1SetAdaptiveSync(
                        ZwlrOutputConfigurationHeadV1SetAdaptiveSyncRequest::read_with_interface(
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
            Self::ZwlrOutputManagerV1CreateConfiguration(m) => m.object_id(),
            Self::ZwlrOutputManagerV1Stop(m) => m.object_id(),
            Self::ZwlrOutputHeadV1Release(m) => m.object_id(),
            Self::ZwlrOutputModeV1Release(m) => m.object_id(),
            Self::ZwlrOutputConfigurationV1EnableHead(m) => m.object_id(),
            Self::ZwlrOutputConfigurationV1DisableHead(m) => m.object_id(),
            Self::ZwlrOutputConfigurationV1Apply(m) => m.object_id(),
            Self::ZwlrOutputConfigurationV1Test(m) => m.object_id(),
            Self::ZwlrOutputConfigurationV1Destroy(m) => m.object_id(),
            Self::ZwlrOutputConfigurationHeadV1SetMode(m) => m.object_id(),
            Self::ZwlrOutputConfigurationHeadV1SetCustomMode(m) => m.object_id(),
            Self::ZwlrOutputConfigurationHeadV1SetPosition(m) => m.object_id(),
            Self::ZwlrOutputConfigurationHeadV1SetTransform(m) => m.object_id(),
            Self::ZwlrOutputConfigurationHeadV1SetScale(m) => m.object_id(),
            Self::ZwlrOutputConfigurationHeadV1SetAdaptiveSync(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwlrOutputManagerV1CreateConfiguration(m) => m.opcode(),
            Self::ZwlrOutputManagerV1Stop(m) => m.opcode(),
            Self::ZwlrOutputHeadV1Release(m) => m.opcode(),
            Self::ZwlrOutputModeV1Release(m) => m.opcode(),
            Self::ZwlrOutputConfigurationV1EnableHead(m) => m.opcode(),
            Self::ZwlrOutputConfigurationV1DisableHead(m) => m.opcode(),
            Self::ZwlrOutputConfigurationV1Apply(m) => m.opcode(),
            Self::ZwlrOutputConfigurationV1Test(m) => m.opcode(),
            Self::ZwlrOutputConfigurationV1Destroy(m) => m.opcode(),
            Self::ZwlrOutputConfigurationHeadV1SetMode(m) => m.opcode(),
            Self::ZwlrOutputConfigurationHeadV1SetCustomMode(m) => m.opcode(),
            Self::ZwlrOutputConfigurationHeadV1SetPosition(m) => m.opcode(),
            Self::ZwlrOutputConfigurationHeadV1SetTransform(m) => m.opcode(),
            Self::ZwlrOutputConfigurationHeadV1SetScale(m) => m.opcode(),
            Self::ZwlrOutputConfigurationHeadV1SetAdaptiveSync(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwlrOutputManagerV1CreateConfiguration(m) => m.payload_size(),
            Self::ZwlrOutputManagerV1Stop(m) => m.payload_size(),
            Self::ZwlrOutputHeadV1Release(m) => m.payload_size(),
            Self::ZwlrOutputModeV1Release(m) => m.payload_size(),
            Self::ZwlrOutputConfigurationV1EnableHead(m) => m.payload_size(),
            Self::ZwlrOutputConfigurationV1DisableHead(m) => m.payload_size(),
            Self::ZwlrOutputConfigurationV1Apply(m) => m.payload_size(),
            Self::ZwlrOutputConfigurationV1Test(m) => m.payload_size(),
            Self::ZwlrOutputConfigurationV1Destroy(m) => m.payload_size(),
            Self::ZwlrOutputConfigurationHeadV1SetMode(m) => m.payload_size(),
            Self::ZwlrOutputConfigurationHeadV1SetCustomMode(m) => m.payload_size(),
            Self::ZwlrOutputConfigurationHeadV1SetPosition(m) => m.payload_size(),
            Self::ZwlrOutputConfigurationHeadV1SetTransform(m) => m.payload_size(),
            Self::ZwlrOutputConfigurationHeadV1SetScale(m) => m.payload_size(),
            Self::ZwlrOutputConfigurationHeadV1SetAdaptiveSync(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwlrOutputManagerV1CreateConfiguration(m) => m.write(writer, state).await,
            Self::ZwlrOutputManagerV1Stop(m) => m.write(writer, state).await,
            Self::ZwlrOutputHeadV1Release(m) => m.write(writer, state).await,
            Self::ZwlrOutputModeV1Release(m) => m.write(writer, state).await,
            Self::ZwlrOutputConfigurationV1EnableHead(m) => m.write(writer, state).await,
            Self::ZwlrOutputConfigurationV1DisableHead(m) => m.write(writer, state).await,
            Self::ZwlrOutputConfigurationV1Apply(m) => m.write(writer, state).await,
            Self::ZwlrOutputConfigurationV1Test(m) => m.write(writer, state).await,
            Self::ZwlrOutputConfigurationV1Destroy(m) => m.write(writer, state).await,
            Self::ZwlrOutputConfigurationHeadV1SetMode(m) => m.write(writer, state).await,
            Self::ZwlrOutputConfigurationHeadV1SetCustomMode(m) => m.write(writer, state).await,
            Self::ZwlrOutputConfigurationHeadV1SetPosition(m) => m.write(writer, state).await,
            Self::ZwlrOutputConfigurationHeadV1SetTransform(m) => m.write(writer, state).await,
            Self::ZwlrOutputConfigurationHeadV1SetScale(m) => m.write(writer, state).await,
            Self::ZwlrOutputConfigurationHeadV1SetAdaptiveSync(m) => m.write(writer, state).await,
        }
    }
}
