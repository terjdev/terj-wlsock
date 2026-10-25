use super::{
    ZwpTabletManagerV2DestroyRequest, ZwpTabletManagerV2GetTabletSeatRequest,
    ZwpTabletPadDialV2DestroyRequest, ZwpTabletPadDialV2SetFeedbackRequest,
    ZwpTabletPadGroupV2DestroyRequest, ZwpTabletPadRingV2DestroyRequest,
    ZwpTabletPadRingV2SetFeedbackRequest, ZwpTabletPadStripV2DestroyRequest,
    ZwpTabletPadStripV2SetFeedbackRequest, ZwpTabletPadV2DestroyRequest,
    ZwpTabletPadV2SetFeedbackRequest, ZwpTabletSeatV2DestroyRequest, ZwpTabletToolV2DestroyRequest,
    ZwpTabletToolV2SetCursorRequest, ZwpTabletV2DestroyRequest,
};
use crate::protocol::ext::tablet::ObjectInteface as TabletObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Tablet requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwpTabletManagerV2GetTabletSeatRequest].
    ZwpTabletManagerV2GetTabletSeat(ZwpTabletManagerV2GetTabletSeatRequest),
    /// Message [ZwpTabletManagerV2DestroyRequest].
    ZwpTabletManagerV2Destroy(ZwpTabletManagerV2DestroyRequest),
    /// Message [ZwpTabletSeatV2DestroyRequest].
    ZwpTabletSeatV2Destroy(ZwpTabletSeatV2DestroyRequest),
    /// Message [ZwpTabletToolV2SetCursorRequest].
    ZwpTabletToolV2SetCursor(ZwpTabletToolV2SetCursorRequest),
    /// Message [ZwpTabletToolV2DestroyRequest].
    ZwpTabletToolV2Destroy(ZwpTabletToolV2DestroyRequest),
    /// Message [ZwpTabletV2DestroyRequest].
    ZwpTabletV2Destroy(ZwpTabletV2DestroyRequest),
    /// Message [ZwpTabletPadRingV2SetFeedbackRequest].
    ZwpTabletPadRingV2SetFeedback(ZwpTabletPadRingV2SetFeedbackRequest),
    /// Message [ZwpTabletPadRingV2DestroyRequest].
    ZwpTabletPadRingV2Destroy(ZwpTabletPadRingV2DestroyRequest),
    /// Message [ZwpTabletPadStripV2SetFeedbackRequest].
    ZwpTabletPadStripV2SetFeedback(ZwpTabletPadStripV2SetFeedbackRequest),
    /// Message [ZwpTabletPadStripV2DestroyRequest].
    ZwpTabletPadStripV2Destroy(ZwpTabletPadStripV2DestroyRequest),
    /// Message [ZwpTabletPadGroupV2DestroyRequest].
    ZwpTabletPadGroupV2Destroy(ZwpTabletPadGroupV2DestroyRequest),
    /// Message [ZwpTabletPadV2SetFeedbackRequest].
    ZwpTabletPadV2SetFeedback(ZwpTabletPadV2SetFeedbackRequest),
    /// Message [ZwpTabletPadV2DestroyRequest].
    ZwpTabletPadV2Destroy(ZwpTabletPadV2DestroyRequest),
    /// Message [ZwpTabletPadDialV2SetFeedbackRequest].
    ZwpTabletPadDialV2SetFeedback(ZwpTabletPadDialV2SetFeedbackRequest),
    /// Message [ZwpTabletPadDialV2DestroyRequest].
    ZwpTabletPadDialV2Destroy(ZwpTabletPadDialV2DestroyRequest),
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
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletManagerV2 { .. }) => match opcode
            {
                ZwpTabletManagerV2GetTabletSeatRequest::OPCODE => {
                    Ok(Self::ZwpTabletManagerV2GetTabletSeat(
                        ZwpTabletManagerV2GetTabletSeatRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpTabletManagerV2DestroyRequest::OPCODE => Ok(Self::ZwpTabletManagerV2Destroy(
                    ZwpTabletManagerV2DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletSeatV2 { .. }) => match opcode {
                ZwpTabletSeatV2DestroyRequest::OPCODE => Ok(Self::ZwpTabletSeatV2Destroy(
                    ZwpTabletSeatV2DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletToolV2 { .. }) => match opcode {
                ZwpTabletToolV2SetCursorRequest::OPCODE => Ok(Self::ZwpTabletToolV2SetCursor(
                    ZwpTabletToolV2SetCursorRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV2DestroyRequest::OPCODE => Ok(Self::ZwpTabletToolV2Destroy(
                    ZwpTabletToolV2DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletV2 { .. }) => match opcode {
                ZwpTabletV2DestroyRequest::OPCODE => Ok(Self::ZwpTabletV2Destroy(
                    ZwpTabletV2DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletPadRingV2 { .. }) => match opcode
            {
                ZwpTabletPadRingV2SetFeedbackRequest::OPCODE => {
                    Ok(Self::ZwpTabletPadRingV2SetFeedback(
                        ZwpTabletPadRingV2SetFeedbackRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpTabletPadRingV2DestroyRequest::OPCODE => Ok(Self::ZwpTabletPadRingV2Destroy(
                    ZwpTabletPadRingV2DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletPadStripV2 { .. }) => {
                match opcode {
                    ZwpTabletPadStripV2SetFeedbackRequest::OPCODE => {
                        Ok(Self::ZwpTabletPadStripV2SetFeedback(
                            ZwpTabletPadStripV2SetFeedbackRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTabletPadStripV2DestroyRequest::OPCODE => {
                        Ok(Self::ZwpTabletPadStripV2Destroy(
                            ZwpTabletPadStripV2DestroyRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletPadGroupV2 { .. }) => {
                match opcode {
                    ZwpTabletPadGroupV2DestroyRequest::OPCODE => {
                        Ok(Self::ZwpTabletPadGroupV2Destroy(
                            ZwpTabletPadGroupV2DestroyRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletPadV2 { .. }) => match opcode {
                ZwpTabletPadV2SetFeedbackRequest::OPCODE => Ok(Self::ZwpTabletPadV2SetFeedback(
                    ZwpTabletPadV2SetFeedbackRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletPadV2DestroyRequest::OPCODE => Ok(Self::ZwpTabletPadV2Destroy(
                    ZwpTabletPadV2DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletPadDialV2 { .. }) => match opcode
            {
                ZwpTabletPadDialV2SetFeedbackRequest::OPCODE => {
                    Ok(Self::ZwpTabletPadDialV2SetFeedback(
                        ZwpTabletPadDialV2SetFeedbackRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpTabletPadDialV2DestroyRequest::OPCODE => Ok(Self::ZwpTabletPadDialV2Destroy(
                    ZwpTabletPadDialV2DestroyRequest::read_with_interface(
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
            Self::ZwpTabletManagerV2GetTabletSeat(message) => message.object_id(),
            Self::ZwpTabletManagerV2Destroy(message) => message.object_id(),
            Self::ZwpTabletSeatV2Destroy(message) => message.object_id(),
            Self::ZwpTabletToolV2SetCursor(message) => message.object_id(),
            Self::ZwpTabletToolV2Destroy(message) => message.object_id(),
            Self::ZwpTabletV2Destroy(message) => message.object_id(),
            Self::ZwpTabletPadRingV2SetFeedback(message) => message.object_id(),
            Self::ZwpTabletPadRingV2Destroy(message) => message.object_id(),
            Self::ZwpTabletPadStripV2SetFeedback(message) => message.object_id(),
            Self::ZwpTabletPadStripV2Destroy(message) => message.object_id(),
            Self::ZwpTabletPadGroupV2Destroy(message) => message.object_id(),
            Self::ZwpTabletPadV2SetFeedback(message) => message.object_id(),
            Self::ZwpTabletPadV2Destroy(message) => message.object_id(),
            Self::ZwpTabletPadDialV2SetFeedback(message) => message.object_id(),
            Self::ZwpTabletPadDialV2Destroy(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpTabletManagerV2GetTabletSeat(message) => message.opcode(),
            Self::ZwpTabletManagerV2Destroy(message) => message.opcode(),
            Self::ZwpTabletSeatV2Destroy(message) => message.opcode(),
            Self::ZwpTabletToolV2SetCursor(message) => message.opcode(),
            Self::ZwpTabletToolV2Destroy(message) => message.opcode(),
            Self::ZwpTabletV2Destroy(message) => message.opcode(),
            Self::ZwpTabletPadRingV2SetFeedback(message) => message.opcode(),
            Self::ZwpTabletPadRingV2Destroy(message) => message.opcode(),
            Self::ZwpTabletPadStripV2SetFeedback(message) => message.opcode(),
            Self::ZwpTabletPadStripV2Destroy(message) => message.opcode(),
            Self::ZwpTabletPadGroupV2Destroy(message) => message.opcode(),
            Self::ZwpTabletPadV2SetFeedback(message) => message.opcode(),
            Self::ZwpTabletPadV2Destroy(message) => message.opcode(),
            Self::ZwpTabletPadDialV2SetFeedback(message) => message.opcode(),
            Self::ZwpTabletPadDialV2Destroy(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpTabletManagerV2GetTabletSeat(message) => message.payload_size(),
            Self::ZwpTabletManagerV2Destroy(message) => message.payload_size(),
            Self::ZwpTabletSeatV2Destroy(message) => message.payload_size(),
            Self::ZwpTabletToolV2SetCursor(message) => message.payload_size(),
            Self::ZwpTabletToolV2Destroy(message) => message.payload_size(),
            Self::ZwpTabletV2Destroy(message) => message.payload_size(),
            Self::ZwpTabletPadRingV2SetFeedback(message) => message.payload_size(),
            Self::ZwpTabletPadRingV2Destroy(message) => message.payload_size(),
            Self::ZwpTabletPadStripV2SetFeedback(message) => message.payload_size(),
            Self::ZwpTabletPadStripV2Destroy(message) => message.payload_size(),
            Self::ZwpTabletPadGroupV2Destroy(message) => message.payload_size(),
            Self::ZwpTabletPadV2SetFeedback(message) => message.payload_size(),
            Self::ZwpTabletPadV2Destroy(message) => message.payload_size(),
            Self::ZwpTabletPadDialV2SetFeedback(message) => message.payload_size(),
            Self::ZwpTabletPadDialV2Destroy(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpTabletManagerV2GetTabletSeat(message) => message.write(writer, state).await,
            Self::ZwpTabletManagerV2Destroy(message) => message.write(writer, state).await,
            Self::ZwpTabletSeatV2Destroy(message) => message.write(writer, state).await,
            Self::ZwpTabletToolV2SetCursor(message) => message.write(writer, state).await,
            Self::ZwpTabletToolV2Destroy(message) => message.write(writer, state).await,
            Self::ZwpTabletV2Destroy(message) => message.write(writer, state).await,
            Self::ZwpTabletPadRingV2SetFeedback(message) => message.write(writer, state).await,
            Self::ZwpTabletPadRingV2Destroy(message) => message.write(writer, state).await,
            Self::ZwpTabletPadStripV2SetFeedback(message) => message.write(writer, state).await,
            Self::ZwpTabletPadStripV2Destroy(message) => message.write(writer, state).await,
            Self::ZwpTabletPadGroupV2Destroy(message) => message.write(writer, state).await,
            Self::ZwpTabletPadV2SetFeedback(message) => message.write(writer, state).await,
            Self::ZwpTabletPadV2Destroy(message) => message.write(writer, state).await,
            Self::ZwpTabletPadDialV2SetFeedback(message) => message.write(writer, state).await,
            Self::ZwpTabletPadDialV2Destroy(message) => message.write(writer, state).await,
        }
    }
}
