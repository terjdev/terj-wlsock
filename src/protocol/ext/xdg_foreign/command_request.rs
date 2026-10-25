use super::ObjectInteface as XdgForeignObjectInteface;
use super::{
    ZxdgExportedV1DestroyRequest, ZxdgExportedV2DestroyRequest, ZxdgExporterV1DestroyRequest,
    ZxdgExporterV1ExportRequest, ZxdgExporterV2DestroyRequest, ZxdgExporterV2ExportToplevelRequest,
    ZxdgImportedV1DestroyRequest, ZxdgImportedV1SetParentOfRequest, ZxdgImportedV2DestroyRequest,
    ZxdgImportedV2SetParentOfRequest, ZxdgImporterV1DestroyRequest, ZxdgImporterV1ImportRequest,
    ZxdgImporterV2DestroyRequest, ZxdgImporterV2ImportToplevelRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches xdg foreign requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZxdgExporterV1DestroyRequest].
    ZxdgExporterV1Destroy(ZxdgExporterV1DestroyRequest),
    /// Message [ZxdgExporterV1ExportRequest].
    ZxdgExporterV1Export(ZxdgExporterV1ExportRequest),
    /// Message [ZxdgImporterV1DestroyRequest].
    ZxdgImporterV1Destroy(ZxdgImporterV1DestroyRequest),
    /// Message [ZxdgImporterV1ImportRequest].
    ZxdgImporterV1Import(ZxdgImporterV1ImportRequest),
    /// Message [ZxdgExportedV1DestroyRequest].
    ZxdgExportedV1Destroy(ZxdgExportedV1DestroyRequest),
    /// Message [ZxdgImportedV1DestroyRequest].
    ZxdgImportedV1Destroy(ZxdgImportedV1DestroyRequest),
    /// Message [ZxdgImportedV1SetParentOfRequest].
    ZxdgImportedV1SetParentOf(ZxdgImportedV1SetParentOfRequest),
    /// Message [ZxdgExporterV2DestroyRequest].
    ZxdgExporterV2Destroy(ZxdgExporterV2DestroyRequest),
    /// Message [ZxdgExporterV2ExportToplevelRequest].
    ZxdgExporterV2ExportToplevel(ZxdgExporterV2ExportToplevelRequest),
    /// Message [ZxdgImporterV2DestroyRequest].
    ZxdgImporterV2Destroy(ZxdgImporterV2DestroyRequest),
    /// Message [ZxdgImporterV2ImportToplevelRequest].
    ZxdgImporterV2ImportToplevel(ZxdgImporterV2ImportToplevelRequest),
    /// Message [ZxdgExportedV2DestroyRequest].
    ZxdgExportedV2Destroy(ZxdgExportedV2DestroyRequest),
    /// Message [ZxdgImportedV2DestroyRequest].
    ZxdgImportedV2Destroy(ZxdgImportedV2DestroyRequest),
    /// Message [ZxdgImportedV2SetParentOfRequest].
    ZxdgImportedV2SetParentOf(ZxdgImportedV2SetParentOfRequest),
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
            ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgExporterV1 { .. }) => {
                match opcode {
                    ZxdgExporterV1DestroyRequest::OPCODE => Ok(Self::ZxdgExporterV1Destroy(
                        ZxdgExporterV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZxdgExporterV1ExportRequest::OPCODE => Ok(Self::ZxdgExporterV1Export(
                        ZxdgExporterV1ExportRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgImporterV1 { .. }) => {
                match opcode {
                    ZxdgImporterV1DestroyRequest::OPCODE => Ok(Self::ZxdgImporterV1Destroy(
                        ZxdgImporterV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZxdgImporterV1ImportRequest::OPCODE => Ok(Self::ZxdgImporterV1Import(
                        ZxdgImporterV1ImportRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgExportedV1 { .. }) => {
                match opcode {
                    ZxdgExportedV1DestroyRequest::OPCODE => Ok(Self::ZxdgExportedV1Destroy(
                        ZxdgExportedV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgImportedV1 { .. }) => {
                match opcode {
                    ZxdgImportedV1DestroyRequest::OPCODE => Ok(Self::ZxdgImportedV1Destroy(
                        ZxdgImportedV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZxdgImportedV1SetParentOfRequest::OPCODE => {
                        Ok(Self::ZxdgImportedV1SetParentOf(
                            ZxdgImportedV1SetParentOfRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgExporterV2 { .. }) => {
                match opcode {
                    ZxdgExporterV2DestroyRequest::OPCODE => Ok(Self::ZxdgExporterV2Destroy(
                        ZxdgExporterV2DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZxdgExporterV2ExportToplevelRequest::OPCODE => {
                        Ok(Self::ZxdgExporterV2ExportToplevel(
                            ZxdgExporterV2ExportToplevelRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgImporterV2 { .. }) => {
                match opcode {
                    ZxdgImporterV2DestroyRequest::OPCODE => Ok(Self::ZxdgImporterV2Destroy(
                        ZxdgImporterV2DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZxdgImporterV2ImportToplevelRequest::OPCODE => {
                        Ok(Self::ZxdgImporterV2ImportToplevel(
                            ZxdgImporterV2ImportToplevelRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgExportedV2 { .. }) => {
                match opcode {
                    ZxdgExportedV2DestroyRequest::OPCODE => Ok(Self::ZxdgExportedV2Destroy(
                        ZxdgExportedV2DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgImportedV2 { .. }) => {
                match opcode {
                    ZxdgImportedV2DestroyRequest::OPCODE => Ok(Self::ZxdgImportedV2Destroy(
                        ZxdgImportedV2DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZxdgImportedV2SetParentOfRequest::OPCODE => {
                        Ok(Self::ZxdgImportedV2SetParentOf(
                            ZxdgImportedV2SetParentOfRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            _ => Err(FdReaderError::InvalidProtocol),
        }
    }
}
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::ZxdgExporterV1Destroy(m) => m.object_id(),
            Self::ZxdgExporterV1Export(m) => m.object_id(),
            Self::ZxdgImporterV1Destroy(m) => m.object_id(),
            Self::ZxdgImporterV1Import(m) => m.object_id(),
            Self::ZxdgExportedV1Destroy(m) => m.object_id(),
            Self::ZxdgImportedV1Destroy(m) => m.object_id(),
            Self::ZxdgImportedV1SetParentOf(m) => m.object_id(),
            Self::ZxdgExporterV2Destroy(m) => m.object_id(),
            Self::ZxdgExporterV2ExportToplevel(m) => m.object_id(),
            Self::ZxdgImporterV2Destroy(m) => m.object_id(),
            Self::ZxdgImporterV2ImportToplevel(m) => m.object_id(),
            Self::ZxdgExportedV2Destroy(m) => m.object_id(),
            Self::ZxdgImportedV2Destroy(m) => m.object_id(),
            Self::ZxdgImportedV2SetParentOf(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZxdgExporterV1Destroy(m) => m.opcode(),
            Self::ZxdgExporterV1Export(m) => m.opcode(),
            Self::ZxdgImporterV1Destroy(m) => m.opcode(),
            Self::ZxdgImporterV1Import(m) => m.opcode(),
            Self::ZxdgExportedV1Destroy(m) => m.opcode(),
            Self::ZxdgImportedV1Destroy(m) => m.opcode(),
            Self::ZxdgImportedV1SetParentOf(m) => m.opcode(),
            Self::ZxdgExporterV2Destroy(m) => m.opcode(),
            Self::ZxdgExporterV2ExportToplevel(m) => m.opcode(),
            Self::ZxdgImporterV2Destroy(m) => m.opcode(),
            Self::ZxdgImporterV2ImportToplevel(m) => m.opcode(),
            Self::ZxdgExportedV2Destroy(m) => m.opcode(),
            Self::ZxdgImportedV2Destroy(m) => m.opcode(),
            Self::ZxdgImportedV2SetParentOf(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZxdgExporterV1Destroy(m) => m.payload_size(),
            Self::ZxdgExporterV1Export(m) => m.payload_size(),
            Self::ZxdgImporterV1Destroy(m) => m.payload_size(),
            Self::ZxdgImporterV1Import(m) => m.payload_size(),
            Self::ZxdgExportedV1Destroy(m) => m.payload_size(),
            Self::ZxdgImportedV1Destroy(m) => m.payload_size(),
            Self::ZxdgImportedV1SetParentOf(m) => m.payload_size(),
            Self::ZxdgExporterV2Destroy(m) => m.payload_size(),
            Self::ZxdgExporterV2ExportToplevel(m) => m.payload_size(),
            Self::ZxdgImporterV2Destroy(m) => m.payload_size(),
            Self::ZxdgImporterV2ImportToplevel(m) => m.payload_size(),
            Self::ZxdgExportedV2Destroy(m) => m.payload_size(),
            Self::ZxdgImportedV2Destroy(m) => m.payload_size(),
            Self::ZxdgImportedV2SetParentOf(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZxdgExporterV1Destroy(m) => m.write(writer, state).await,
            Self::ZxdgExporterV1Export(m) => m.write(writer, state).await,
            Self::ZxdgImporterV1Destroy(m) => m.write(writer, state).await,
            Self::ZxdgImporterV1Import(m) => m.write(writer, state).await,
            Self::ZxdgExportedV1Destroy(m) => m.write(writer, state).await,
            Self::ZxdgImportedV1Destroy(m) => m.write(writer, state).await,
            Self::ZxdgImportedV1SetParentOf(m) => m.write(writer, state).await,
            Self::ZxdgExporterV2Destroy(m) => m.write(writer, state).await,
            Self::ZxdgExporterV2ExportToplevel(m) => m.write(writer, state).await,
            Self::ZxdgImporterV2Destroy(m) => m.write(writer, state).await,
            Self::ZxdgImporterV2ImportToplevel(m) => m.write(writer, state).await,
            Self::ZxdgExportedV2Destroy(m) => m.write(writer, state).await,
            Self::ZxdgImportedV2Destroy(m) => m.write(writer, state).await,
            Self::ZxdgImportedV2SetParentOf(m) => m.write(writer, state).await,
        }
    }
}
