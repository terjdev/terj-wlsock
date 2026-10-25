use super::{
    ExtImageCopyCaptureCursorSessionV1EnterEvent, ExtImageCopyCaptureCursorSessionV1HotspotEvent,
    ExtImageCopyCaptureCursorSessionV1LeaveEvent, ExtImageCopyCaptureCursorSessionV1PositionEvent,
    ExtImageCopyCaptureFrameV1DamageEvent, ExtImageCopyCaptureFrameV1FailedEvent,
    ExtImageCopyCaptureFrameV1PresentationTimeEvent, ExtImageCopyCaptureFrameV1ReadyEvent,
    ExtImageCopyCaptureFrameV1TransformEvent, ExtImageCopyCaptureSessionV1BufferSizeEvent,
    ExtImageCopyCaptureSessionV1DmabufDeviceEvent, ExtImageCopyCaptureSessionV1DmabufFormatEvent,
    ExtImageCopyCaptureSessionV1DoneEvent, ExtImageCopyCaptureSessionV1ShmFormatEvent,
    ExtImageCopyCaptureSessionV1StoppedEvent,
};
use crate::protocol::ext::image_copy_capture::ObjectInteface as ImageCopyCaptureObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Image copy capture events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ExtImageCopyCaptureSessionV1BufferSizeEvent].
    ExtImageCopyCaptureSessionV1BufferSize(ExtImageCopyCaptureSessionV1BufferSizeEvent),
    /// Message [ExtImageCopyCaptureSessionV1ShmFormatEvent].
    ExtImageCopyCaptureSessionV1ShmFormat(ExtImageCopyCaptureSessionV1ShmFormatEvent),
    /// Message [ExtImageCopyCaptureSessionV1DmabufDeviceEvent].
    ExtImageCopyCaptureSessionV1DmabufDevice(ExtImageCopyCaptureSessionV1DmabufDeviceEvent),
    /// Message [ExtImageCopyCaptureSessionV1DmabufFormatEvent].
    ExtImageCopyCaptureSessionV1DmabufFormat(ExtImageCopyCaptureSessionV1DmabufFormatEvent),
    /// Message [ExtImageCopyCaptureSessionV1DoneEvent].
    ExtImageCopyCaptureSessionV1Done(ExtImageCopyCaptureSessionV1DoneEvent),
    /// Message [ExtImageCopyCaptureSessionV1StoppedEvent].
    ExtImageCopyCaptureSessionV1Stopped(ExtImageCopyCaptureSessionV1StoppedEvent),
    /// Message [ExtImageCopyCaptureFrameV1TransformEvent].
    ExtImageCopyCaptureFrameV1Transform(ExtImageCopyCaptureFrameV1TransformEvent),
    /// Message [ExtImageCopyCaptureFrameV1DamageEvent].
    ExtImageCopyCaptureFrameV1Damage(ExtImageCopyCaptureFrameV1DamageEvent),
    /// Message [ExtImageCopyCaptureFrameV1PresentationTimeEvent].
    ExtImageCopyCaptureFrameV1PresentationTime(ExtImageCopyCaptureFrameV1PresentationTimeEvent),
    /// Message [ExtImageCopyCaptureFrameV1ReadyEvent].
    ExtImageCopyCaptureFrameV1Ready(ExtImageCopyCaptureFrameV1ReadyEvent),
    /// Message [ExtImageCopyCaptureFrameV1FailedEvent].
    ExtImageCopyCaptureFrameV1Failed(ExtImageCopyCaptureFrameV1FailedEvent),
    /// Message [ExtImageCopyCaptureCursorSessionV1EnterEvent].
    ExtImageCopyCaptureCursorSessionV1Enter(ExtImageCopyCaptureCursorSessionV1EnterEvent),
    /// Message [ExtImageCopyCaptureCursorSessionV1LeaveEvent].
    ExtImageCopyCaptureCursorSessionV1Leave(ExtImageCopyCaptureCursorSessionV1LeaveEvent),
    /// Message [ExtImageCopyCaptureCursorSessionV1PositionEvent].
    ExtImageCopyCaptureCursorSessionV1Position(ExtImageCopyCaptureCursorSessionV1PositionEvent),
    /// Message [ExtImageCopyCaptureCursorSessionV1HotspotEvent].
    ExtImageCopyCaptureCursorSessionV1Hotspot(ExtImageCopyCaptureCursorSessionV1HotspotEvent),
}
impl CommandReader<Self> for CommandEvent {
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
impl InterfaceReader for CommandEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        match interface {
            ObjectInteface::ImageCopyCapture(
                ImageCopyCaptureObjectInteface::ExtImageCopyCaptureSessionV1 { .. },
            ) => match opcode {
                ExtImageCopyCaptureSessionV1BufferSizeEvent::OPCODE => {
                    Ok(Self::ExtImageCopyCaptureSessionV1BufferSize(
                        ExtImageCopyCaptureSessionV1BufferSizeEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtImageCopyCaptureSessionV1ShmFormatEvent::OPCODE => {
                    Ok(Self::ExtImageCopyCaptureSessionV1ShmFormat(
                        ExtImageCopyCaptureSessionV1ShmFormatEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtImageCopyCaptureSessionV1DmabufDeviceEvent::OPCODE => {
                    Ok(Self::ExtImageCopyCaptureSessionV1DmabufDevice(
                        ExtImageCopyCaptureSessionV1DmabufDeviceEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtImageCopyCaptureSessionV1DmabufFormatEvent::OPCODE => {
                    Ok(Self::ExtImageCopyCaptureSessionV1DmabufFormat(
                        ExtImageCopyCaptureSessionV1DmabufFormatEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtImageCopyCaptureSessionV1DoneEvent::OPCODE => {
                    Ok(Self::ExtImageCopyCaptureSessionV1Done(
                        ExtImageCopyCaptureSessionV1DoneEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtImageCopyCaptureSessionV1StoppedEvent::OPCODE => {
                    Ok(Self::ExtImageCopyCaptureSessionV1Stopped(
                        ExtImageCopyCaptureSessionV1StoppedEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::ImageCopyCapture(
                ImageCopyCaptureObjectInteface::ExtImageCopyCaptureFrameV1 { .. },
            ) => match opcode {
                ExtImageCopyCaptureFrameV1TransformEvent::OPCODE => {
                    Ok(Self::ExtImageCopyCaptureFrameV1Transform(
                        ExtImageCopyCaptureFrameV1TransformEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtImageCopyCaptureFrameV1DamageEvent::OPCODE => {
                    Ok(Self::ExtImageCopyCaptureFrameV1Damage(
                        ExtImageCopyCaptureFrameV1DamageEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtImageCopyCaptureFrameV1PresentationTimeEvent::OPCODE => {
                    Ok(Self::ExtImageCopyCaptureFrameV1PresentationTime(
                        ExtImageCopyCaptureFrameV1PresentationTimeEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtImageCopyCaptureFrameV1ReadyEvent::OPCODE => {
                    Ok(Self::ExtImageCopyCaptureFrameV1Ready(
                        ExtImageCopyCaptureFrameV1ReadyEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtImageCopyCaptureFrameV1FailedEvent::OPCODE => {
                    Ok(Self::ExtImageCopyCaptureFrameV1Failed(
                        ExtImageCopyCaptureFrameV1FailedEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::ImageCopyCapture(
                ImageCopyCaptureObjectInteface::ExtImageCopyCaptureCursorSessionV1 { .. },
            ) => match opcode {
                ExtImageCopyCaptureCursorSessionV1EnterEvent::OPCODE => {
                    Ok(Self::ExtImageCopyCaptureCursorSessionV1Enter(
                        ExtImageCopyCaptureCursorSessionV1EnterEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtImageCopyCaptureCursorSessionV1LeaveEvent::OPCODE => {
                    Ok(Self::ExtImageCopyCaptureCursorSessionV1Leave(
                        ExtImageCopyCaptureCursorSessionV1LeaveEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtImageCopyCaptureCursorSessionV1PositionEvent::OPCODE => {
                    Ok(Self::ExtImageCopyCaptureCursorSessionV1Position(
                        ExtImageCopyCaptureCursorSessionV1PositionEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtImageCopyCaptureCursorSessionV1HotspotEvent::OPCODE => {
                    Ok(Self::ExtImageCopyCaptureCursorSessionV1Hotspot(
                        ExtImageCopyCaptureCursorSessionV1HotspotEvent::read_with_interface(
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
impl CommandWriter for CommandEvent {
    fn object_id(&self) -> u32 {
        match self {
            Self::ExtImageCopyCaptureSessionV1BufferSize(message) => message.object_id(),
            Self::ExtImageCopyCaptureSessionV1ShmFormat(message) => message.object_id(),
            Self::ExtImageCopyCaptureSessionV1DmabufDevice(message) => message.object_id(),
            Self::ExtImageCopyCaptureSessionV1DmabufFormat(message) => message.object_id(),
            Self::ExtImageCopyCaptureSessionV1Done(message) => message.object_id(),
            Self::ExtImageCopyCaptureSessionV1Stopped(message) => message.object_id(),
            Self::ExtImageCopyCaptureFrameV1Transform(message) => message.object_id(),
            Self::ExtImageCopyCaptureFrameV1Damage(message) => message.object_id(),
            Self::ExtImageCopyCaptureFrameV1PresentationTime(message) => message.object_id(),
            Self::ExtImageCopyCaptureFrameV1Ready(message) => message.object_id(),
            Self::ExtImageCopyCaptureFrameV1Failed(message) => message.object_id(),
            Self::ExtImageCopyCaptureCursorSessionV1Enter(message) => message.object_id(),
            Self::ExtImageCopyCaptureCursorSessionV1Leave(message) => message.object_id(),
            Self::ExtImageCopyCaptureCursorSessionV1Position(message) => message.object_id(),
            Self::ExtImageCopyCaptureCursorSessionV1Hotspot(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ExtImageCopyCaptureSessionV1BufferSize(message) => message.opcode(),
            Self::ExtImageCopyCaptureSessionV1ShmFormat(message) => message.opcode(),
            Self::ExtImageCopyCaptureSessionV1DmabufDevice(message) => message.opcode(),
            Self::ExtImageCopyCaptureSessionV1DmabufFormat(message) => message.opcode(),
            Self::ExtImageCopyCaptureSessionV1Done(message) => message.opcode(),
            Self::ExtImageCopyCaptureSessionV1Stopped(message) => message.opcode(),
            Self::ExtImageCopyCaptureFrameV1Transform(message) => message.opcode(),
            Self::ExtImageCopyCaptureFrameV1Damage(message) => message.opcode(),
            Self::ExtImageCopyCaptureFrameV1PresentationTime(message) => message.opcode(),
            Self::ExtImageCopyCaptureFrameV1Ready(message) => message.opcode(),
            Self::ExtImageCopyCaptureFrameV1Failed(message) => message.opcode(),
            Self::ExtImageCopyCaptureCursorSessionV1Enter(message) => message.opcode(),
            Self::ExtImageCopyCaptureCursorSessionV1Leave(message) => message.opcode(),
            Self::ExtImageCopyCaptureCursorSessionV1Position(message) => message.opcode(),
            Self::ExtImageCopyCaptureCursorSessionV1Hotspot(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ExtImageCopyCaptureSessionV1BufferSize(message) => message.payload_size(),
            Self::ExtImageCopyCaptureSessionV1ShmFormat(message) => message.payload_size(),
            Self::ExtImageCopyCaptureSessionV1DmabufDevice(message) => message.payload_size(),
            Self::ExtImageCopyCaptureSessionV1DmabufFormat(message) => message.payload_size(),
            Self::ExtImageCopyCaptureSessionV1Done(message) => message.payload_size(),
            Self::ExtImageCopyCaptureSessionV1Stopped(message) => message.payload_size(),
            Self::ExtImageCopyCaptureFrameV1Transform(message) => message.payload_size(),
            Self::ExtImageCopyCaptureFrameV1Damage(message) => message.payload_size(),
            Self::ExtImageCopyCaptureFrameV1PresentationTime(message) => message.payload_size(),
            Self::ExtImageCopyCaptureFrameV1Ready(message) => message.payload_size(),
            Self::ExtImageCopyCaptureFrameV1Failed(message) => message.payload_size(),
            Self::ExtImageCopyCaptureCursorSessionV1Enter(message) => message.payload_size(),
            Self::ExtImageCopyCaptureCursorSessionV1Leave(message) => message.payload_size(),
            Self::ExtImageCopyCaptureCursorSessionV1Position(message) => message.payload_size(),
            Self::ExtImageCopyCaptureCursorSessionV1Hotspot(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ExtImageCopyCaptureSessionV1BufferSize(message) => {
                message.write(writer, state).await
            }
            Self::ExtImageCopyCaptureSessionV1ShmFormat(message) => {
                message.write(writer, state).await
            }
            Self::ExtImageCopyCaptureSessionV1DmabufDevice(message) => {
                message.write(writer, state).await
            }
            Self::ExtImageCopyCaptureSessionV1DmabufFormat(message) => {
                message.write(writer, state).await
            }
            Self::ExtImageCopyCaptureSessionV1Done(message) => message.write(writer, state).await,
            Self::ExtImageCopyCaptureSessionV1Stopped(message) => {
                message.write(writer, state).await
            }
            Self::ExtImageCopyCaptureFrameV1Transform(message) => {
                message.write(writer, state).await
            }
            Self::ExtImageCopyCaptureFrameV1Damage(message) => message.write(writer, state).await,
            Self::ExtImageCopyCaptureFrameV1PresentationTime(message) => {
                message.write(writer, state).await
            }
            Self::ExtImageCopyCaptureFrameV1Ready(message) => message.write(writer, state).await,
            Self::ExtImageCopyCaptureFrameV1Failed(message) => message.write(writer, state).await,
            Self::ExtImageCopyCaptureCursorSessionV1Enter(message) => {
                message.write(writer, state).await
            }
            Self::ExtImageCopyCaptureCursorSessionV1Leave(message) => {
                message.write(writer, state).await
            }
            Self::ExtImageCopyCaptureCursorSessionV1Position(message) => {
                message.write(writer, state).await
            }
            Self::ExtImageCopyCaptureCursorSessionV1Hotspot(message) => {
                message.write(writer, state).await
            }
        }
    }
}
