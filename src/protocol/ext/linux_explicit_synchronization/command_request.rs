use super::ObjectInteface as LinuxExplicitSynchronizationObjectInteface;
use super::{
    ZwpLinuxExplicitSynchronizationV1DestroyRequest,
    ZwpLinuxExplicitSynchronizationV1GetSynchronizationRequest,
    ZwpLinuxSurfaceSynchronizationV1DestroyRequest,
    ZwpLinuxSurfaceSynchronizationV1GetReleaseRequest,
    ZwpLinuxSurfaceSynchronizationV1SetAcquireFenceRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches linux explicit synchronization requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwpLinuxExplicitSynchronizationV1DestroyRequest].
    ZwpLinuxExplicitSynchronizationV1Destroy(ZwpLinuxExplicitSynchronizationV1DestroyRequest),
    /// Message [ZwpLinuxExplicitSynchronizationV1GetSynchronizationRequest].
    ZwpLinuxExplicitSynchronizationV1GetSynchronization(
        ZwpLinuxExplicitSynchronizationV1GetSynchronizationRequest,
    ),
    /// Message [ZwpLinuxSurfaceSynchronizationV1DestroyRequest].
    ZwpLinuxSurfaceSynchronizationV1Destroy(ZwpLinuxSurfaceSynchronizationV1DestroyRequest),
    /// Message [ZwpLinuxSurfaceSynchronizationV1SetAcquireFenceRequest].
    ZwpLinuxSurfaceSynchronizationV1SetAcquireFence(
        ZwpLinuxSurfaceSynchronizationV1SetAcquireFenceRequest,
    ),
    /// Message [ZwpLinuxSurfaceSynchronizationV1GetReleaseRequest].
    ZwpLinuxSurfaceSynchronizationV1GetRelease(ZwpLinuxSurfaceSynchronizationV1GetReleaseRequest),
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
        match interface{
ObjectInteface::LinuxExplicitSynchronization(LinuxExplicitSynchronizationObjectInteface::ZwpLinuxExplicitSynchronizationV1{..})=>match opcode{ZwpLinuxExplicitSynchronizationV1DestroyRequest::OPCODE=>Ok(Self::ZwpLinuxExplicitSynchronizationV1Destroy(ZwpLinuxExplicitSynchronizationV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),ZwpLinuxExplicitSynchronizationV1GetSynchronizationRequest::OPCODE=>Ok(Self::ZwpLinuxExplicitSynchronizationV1GetSynchronization(ZwpLinuxExplicitSynchronizationV1GetSynchronizationRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::LinuxExplicitSynchronization(LinuxExplicitSynchronizationObjectInteface::ZwpLinuxSurfaceSynchronizationV1{..})=>match opcode{ZwpLinuxSurfaceSynchronizationV1DestroyRequest::OPCODE=>Ok(Self::ZwpLinuxSurfaceSynchronizationV1Destroy(ZwpLinuxSurfaceSynchronizationV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),ZwpLinuxSurfaceSynchronizationV1SetAcquireFenceRequest::OPCODE=>Ok(Self::ZwpLinuxSurfaceSynchronizationV1SetAcquireFence(ZwpLinuxSurfaceSynchronizationV1SetAcquireFenceRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),ZwpLinuxSurfaceSynchronizationV1GetReleaseRequest::OPCODE=>Ok(Self::ZwpLinuxSurfaceSynchronizationV1GetRelease(ZwpLinuxSurfaceSynchronizationV1GetReleaseRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),_=>Err(FdReaderError::InvalidProtocol),},
_=>Err(FdReaderError::InvalidProtocol),}
    }
}
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::ZwpLinuxExplicitSynchronizationV1Destroy(m) => m.object_id(),
            Self::ZwpLinuxExplicitSynchronizationV1GetSynchronization(m) => m.object_id(),
            Self::ZwpLinuxSurfaceSynchronizationV1Destroy(m) => m.object_id(),
            Self::ZwpLinuxSurfaceSynchronizationV1SetAcquireFence(m) => m.object_id(),
            Self::ZwpLinuxSurfaceSynchronizationV1GetRelease(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpLinuxExplicitSynchronizationV1Destroy(m) => m.opcode(),
            Self::ZwpLinuxExplicitSynchronizationV1GetSynchronization(m) => m.opcode(),
            Self::ZwpLinuxSurfaceSynchronizationV1Destroy(m) => m.opcode(),
            Self::ZwpLinuxSurfaceSynchronizationV1SetAcquireFence(m) => m.opcode(),
            Self::ZwpLinuxSurfaceSynchronizationV1GetRelease(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpLinuxExplicitSynchronizationV1Destroy(m) => m.payload_size(),
            Self::ZwpLinuxExplicitSynchronizationV1GetSynchronization(m) => m.payload_size(),
            Self::ZwpLinuxSurfaceSynchronizationV1Destroy(m) => m.payload_size(),
            Self::ZwpLinuxSurfaceSynchronizationV1SetAcquireFence(m) => m.payload_size(),
            Self::ZwpLinuxSurfaceSynchronizationV1GetRelease(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpLinuxExplicitSynchronizationV1Destroy(m) => m.write(writer, state).await,
            Self::ZwpLinuxExplicitSynchronizationV1GetSynchronization(m) => {
                m.write(writer, state).await
            }
            Self::ZwpLinuxSurfaceSynchronizationV1Destroy(m) => m.write(writer, state).await,
            Self::ZwpLinuxSurfaceSynchronizationV1SetAcquireFence(m) => {
                m.write(writer, state).await
            }
            Self::ZwpLinuxSurfaceSynchronizationV1GetRelease(m) => m.write(writer, state).await,
        }
    }
}
