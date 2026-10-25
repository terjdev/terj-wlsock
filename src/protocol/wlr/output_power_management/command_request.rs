use super::ObjectInteface as WlrOutputPowerManagementObjectInteface;
use super::{
    ZwlrOutputPowerManagerV1DestroyRequest, ZwlrOutputPowerManagerV1GetOutputPowerRequest,
    ZwlrOutputPowerV1DestroyRequest, ZwlrOutputPowerV1SetModeRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches output power management requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwlrOutputPowerManagerV1GetOutputPowerRequest].
    ZwlrOutputPowerManagerV1GetOutputPower(ZwlrOutputPowerManagerV1GetOutputPowerRequest),
    /// Message [ZwlrOutputPowerManagerV1DestroyRequest].
    ZwlrOutputPowerManagerV1Destroy(ZwlrOutputPowerManagerV1DestroyRequest),
    /// Message [ZwlrOutputPowerV1SetModeRequest].
    ZwlrOutputPowerV1SetMode(ZwlrOutputPowerV1SetModeRequest),
    /// Message [ZwlrOutputPowerV1DestroyRequest].
    ZwlrOutputPowerV1Destroy(ZwlrOutputPowerV1DestroyRequest),
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
            ObjectInteface::WlrOutputPowerManagement(
                WlrOutputPowerManagementObjectInteface::ZwlrOutputPowerManagerV1 { .. },
            ) => match opcode {
                ZwlrOutputPowerManagerV1GetOutputPowerRequest::OPCODE => {
                    Ok(Self::ZwlrOutputPowerManagerV1GetOutputPower(
                        ZwlrOutputPowerManagerV1GetOutputPowerRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrOutputPowerManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwlrOutputPowerManagerV1Destroy(
                        ZwlrOutputPowerManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::WlrOutputPowerManagement(
                WlrOutputPowerManagementObjectInteface::ZwlrOutputPowerV1 { .. },
            ) => match opcode {
                ZwlrOutputPowerV1SetModeRequest::OPCODE => Ok(Self::ZwlrOutputPowerV1SetMode(
                    ZwlrOutputPowerV1SetModeRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrOutputPowerV1DestroyRequest::OPCODE => Ok(Self::ZwlrOutputPowerV1Destroy(
                    ZwlrOutputPowerV1DestroyRequest::read_with_interface(
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
            Self::ZwlrOutputPowerManagerV1GetOutputPower(m) => m.object_id(),
            Self::ZwlrOutputPowerManagerV1Destroy(m) => m.object_id(),
            Self::ZwlrOutputPowerV1SetMode(m) => m.object_id(),
            Self::ZwlrOutputPowerV1Destroy(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwlrOutputPowerManagerV1GetOutputPower(m) => m.opcode(),
            Self::ZwlrOutputPowerManagerV1Destroy(m) => m.opcode(),
            Self::ZwlrOutputPowerV1SetMode(m) => m.opcode(),
            Self::ZwlrOutputPowerV1Destroy(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwlrOutputPowerManagerV1GetOutputPower(m) => m.payload_size(),
            Self::ZwlrOutputPowerManagerV1Destroy(m) => m.payload_size(),
            Self::ZwlrOutputPowerV1SetMode(m) => m.payload_size(),
            Self::ZwlrOutputPowerV1Destroy(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwlrOutputPowerManagerV1GetOutputPower(m) => m.write(writer, state).await,
            Self::ZwlrOutputPowerManagerV1Destroy(m) => m.write(writer, state).await,
            Self::ZwlrOutputPowerV1SetMode(m) => m.write(writer, state).await,
            Self::ZwlrOutputPowerV1Destroy(m) => m.write(writer, state).await,
        }
    }
}
