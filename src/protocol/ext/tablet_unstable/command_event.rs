use super::ObjectInteface as TabletUnstableObjectInteface;
use super::{
    ZwpTabletSeatV1TabletAddedEvent, ZwpTabletSeatV1ToolAddedEvent, ZwpTabletToolV1ButtonEvent,
    ZwpTabletToolV1CapabilityEvent, ZwpTabletToolV1DistanceEvent, ZwpTabletToolV1DoneEvent,
    ZwpTabletToolV1DownEvent, ZwpTabletToolV1FrameEvent, ZwpTabletToolV1HardwareIdWacomEvent,
    ZwpTabletToolV1HardwareSerialEvent, ZwpTabletToolV1MotionEvent, ZwpTabletToolV1PressureEvent,
    ZwpTabletToolV1ProximityInEvent, ZwpTabletToolV1ProximityOutEvent, ZwpTabletToolV1RemovedEvent,
    ZwpTabletToolV1RotationEvent, ZwpTabletToolV1SliderEvent, ZwpTabletToolV1TiltEvent,
    ZwpTabletToolV1TypeEvent, ZwpTabletToolV1UpEvent, ZwpTabletToolV1WheelEvent,
    ZwpTabletV1DoneEvent, ZwpTabletV1IdEvent, ZwpTabletV1NameEvent, ZwpTabletV1PathEvent,
    ZwpTabletV1RemovedEvent,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches tablet unstable events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZwpTabletSeatV1TabletAddedEvent].
    ZwpTabletSeatV1TabletAdded(ZwpTabletSeatV1TabletAddedEvent),
    /// Message [ZwpTabletSeatV1ToolAddedEvent].
    ZwpTabletSeatV1ToolAdded(ZwpTabletSeatV1ToolAddedEvent),
    /// Message [ZwpTabletToolV1TypeEvent].
    ZwpTabletToolV1Type(ZwpTabletToolV1TypeEvent),
    /// Message [ZwpTabletToolV1HardwareSerialEvent].
    ZwpTabletToolV1HardwareSerial(ZwpTabletToolV1HardwareSerialEvent),
    /// Message [ZwpTabletToolV1HardwareIdWacomEvent].
    ZwpTabletToolV1HardwareIdWacom(ZwpTabletToolV1HardwareIdWacomEvent),
    /// Message [ZwpTabletToolV1CapabilityEvent].
    ZwpTabletToolV1Capability(ZwpTabletToolV1CapabilityEvent),
    /// Message [ZwpTabletToolV1DoneEvent].
    ZwpTabletToolV1Done(ZwpTabletToolV1DoneEvent),
    /// Message [ZwpTabletToolV1RemovedEvent].
    ZwpTabletToolV1Removed(ZwpTabletToolV1RemovedEvent),
    /// Message [ZwpTabletToolV1ProximityInEvent].
    ZwpTabletToolV1ProximityIn(ZwpTabletToolV1ProximityInEvent),
    /// Message [ZwpTabletToolV1ProximityOutEvent].
    ZwpTabletToolV1ProximityOut(ZwpTabletToolV1ProximityOutEvent),
    /// Message [ZwpTabletToolV1DownEvent].
    ZwpTabletToolV1Down(ZwpTabletToolV1DownEvent),
    /// Message [ZwpTabletToolV1UpEvent].
    ZwpTabletToolV1Up(ZwpTabletToolV1UpEvent),
    /// Message [ZwpTabletToolV1MotionEvent].
    ZwpTabletToolV1Motion(ZwpTabletToolV1MotionEvent),
    /// Message [ZwpTabletToolV1PressureEvent].
    ZwpTabletToolV1Pressure(ZwpTabletToolV1PressureEvent),
    /// Message [ZwpTabletToolV1DistanceEvent].
    ZwpTabletToolV1Distance(ZwpTabletToolV1DistanceEvent),
    /// Message [ZwpTabletToolV1TiltEvent].
    ZwpTabletToolV1Tilt(ZwpTabletToolV1TiltEvent),
    /// Message [ZwpTabletToolV1RotationEvent].
    ZwpTabletToolV1Rotation(ZwpTabletToolV1RotationEvent),
    /// Message [ZwpTabletToolV1SliderEvent].
    ZwpTabletToolV1Slider(ZwpTabletToolV1SliderEvent),
    /// Message [ZwpTabletToolV1WheelEvent].
    ZwpTabletToolV1Wheel(ZwpTabletToolV1WheelEvent),
    /// Message [ZwpTabletToolV1ButtonEvent].
    ZwpTabletToolV1Button(ZwpTabletToolV1ButtonEvent),
    /// Message [ZwpTabletToolV1FrameEvent].
    ZwpTabletToolV1Frame(ZwpTabletToolV1FrameEvent),
    /// Message [ZwpTabletV1NameEvent].
    ZwpTabletV1Name(ZwpTabletV1NameEvent),
    /// Message [ZwpTabletV1IdEvent].
    ZwpTabletV1Id(ZwpTabletV1IdEvent),
    /// Message [ZwpTabletV1PathEvent].
    ZwpTabletV1Path(ZwpTabletV1PathEvent),
    /// Message [ZwpTabletV1DoneEvent].
    ZwpTabletV1Done(ZwpTabletV1DoneEvent),
    /// Message [ZwpTabletV1RemovedEvent].
    ZwpTabletV1Removed(ZwpTabletV1RemovedEvent),
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
            ObjectInteface::TabletUnstable(TabletUnstableObjectInteface::ZwpTabletSeatV1 {
                ..
            }) => match opcode {
                ZwpTabletSeatV1TabletAddedEvent::OPCODE => Ok(Self::ZwpTabletSeatV1TabletAdded(
                    ZwpTabletSeatV1TabletAddedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletSeatV1ToolAddedEvent::OPCODE => Ok(Self::ZwpTabletSeatV1ToolAdded(
                    ZwpTabletSeatV1ToolAddedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::TabletUnstable(TabletUnstableObjectInteface::ZwpTabletToolV1 {
                ..
            }) => match opcode {
                ZwpTabletToolV1TypeEvent::OPCODE => Ok(Self::ZwpTabletToolV1Type(
                    ZwpTabletToolV1TypeEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV1HardwareSerialEvent::OPCODE => {
                    Ok(Self::ZwpTabletToolV1HardwareSerial(
                        ZwpTabletToolV1HardwareSerialEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpTabletToolV1HardwareIdWacomEvent::OPCODE => {
                    Ok(Self::ZwpTabletToolV1HardwareIdWacom(
                        ZwpTabletToolV1HardwareIdWacomEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpTabletToolV1CapabilityEvent::OPCODE => Ok(Self::ZwpTabletToolV1Capability(
                    ZwpTabletToolV1CapabilityEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV1DoneEvent::OPCODE => Ok(Self::ZwpTabletToolV1Done(
                    ZwpTabletToolV1DoneEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV1RemovedEvent::OPCODE => Ok(Self::ZwpTabletToolV1Removed(
                    ZwpTabletToolV1RemovedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV1ProximityInEvent::OPCODE => Ok(Self::ZwpTabletToolV1ProximityIn(
                    ZwpTabletToolV1ProximityInEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV1ProximityOutEvent::OPCODE => Ok(Self::ZwpTabletToolV1ProximityOut(
                    ZwpTabletToolV1ProximityOutEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV1DownEvent::OPCODE => Ok(Self::ZwpTabletToolV1Down(
                    ZwpTabletToolV1DownEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV1UpEvent::OPCODE => Ok(Self::ZwpTabletToolV1Up(
                    ZwpTabletToolV1UpEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV1MotionEvent::OPCODE => Ok(Self::ZwpTabletToolV1Motion(
                    ZwpTabletToolV1MotionEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV1PressureEvent::OPCODE => Ok(Self::ZwpTabletToolV1Pressure(
                    ZwpTabletToolV1PressureEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV1DistanceEvent::OPCODE => Ok(Self::ZwpTabletToolV1Distance(
                    ZwpTabletToolV1DistanceEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV1TiltEvent::OPCODE => Ok(Self::ZwpTabletToolV1Tilt(
                    ZwpTabletToolV1TiltEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV1RotationEvent::OPCODE => Ok(Self::ZwpTabletToolV1Rotation(
                    ZwpTabletToolV1RotationEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV1SliderEvent::OPCODE => Ok(Self::ZwpTabletToolV1Slider(
                    ZwpTabletToolV1SliderEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV1WheelEvent::OPCODE => Ok(Self::ZwpTabletToolV1Wheel(
                    ZwpTabletToolV1WheelEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV1ButtonEvent::OPCODE => Ok(Self::ZwpTabletToolV1Button(
                    ZwpTabletToolV1ButtonEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV1FrameEvent::OPCODE => Ok(Self::ZwpTabletToolV1Frame(
                    ZwpTabletToolV1FrameEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::TabletUnstable(TabletUnstableObjectInteface::ZwpTabletV1 {
                ..
            }) => match opcode {
                ZwpTabletV1NameEvent::OPCODE => Ok(Self::ZwpTabletV1Name(
                    ZwpTabletV1NameEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletV1IdEvent::OPCODE => Ok(Self::ZwpTabletV1Id(
                    ZwpTabletV1IdEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletV1PathEvent::OPCODE => Ok(Self::ZwpTabletV1Path(
                    ZwpTabletV1PathEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletV1DoneEvent::OPCODE => Ok(Self::ZwpTabletV1Done(
                    ZwpTabletV1DoneEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletV1RemovedEvent::OPCODE => Ok(Self::ZwpTabletV1Removed(
                    ZwpTabletV1RemovedEvent::read_with_interface(
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
impl CommandWriter for CommandEvent {
    fn object_id(&self) -> u32 {
        match self {
            Self::ZwpTabletSeatV1TabletAdded(m) => m.object_id(),
            Self::ZwpTabletSeatV1ToolAdded(m) => m.object_id(),
            Self::ZwpTabletToolV1Type(m) => m.object_id(),
            Self::ZwpTabletToolV1HardwareSerial(m) => m.object_id(),
            Self::ZwpTabletToolV1HardwareIdWacom(m) => m.object_id(),
            Self::ZwpTabletToolV1Capability(m) => m.object_id(),
            Self::ZwpTabletToolV1Done(m) => m.object_id(),
            Self::ZwpTabletToolV1Removed(m) => m.object_id(),
            Self::ZwpTabletToolV1ProximityIn(m) => m.object_id(),
            Self::ZwpTabletToolV1ProximityOut(m) => m.object_id(),
            Self::ZwpTabletToolV1Down(m) => m.object_id(),
            Self::ZwpTabletToolV1Up(m) => m.object_id(),
            Self::ZwpTabletToolV1Motion(m) => m.object_id(),
            Self::ZwpTabletToolV1Pressure(m) => m.object_id(),
            Self::ZwpTabletToolV1Distance(m) => m.object_id(),
            Self::ZwpTabletToolV1Tilt(m) => m.object_id(),
            Self::ZwpTabletToolV1Rotation(m) => m.object_id(),
            Self::ZwpTabletToolV1Slider(m) => m.object_id(),
            Self::ZwpTabletToolV1Wheel(m) => m.object_id(),
            Self::ZwpTabletToolV1Button(m) => m.object_id(),
            Self::ZwpTabletToolV1Frame(m) => m.object_id(),
            Self::ZwpTabletV1Name(m) => m.object_id(),
            Self::ZwpTabletV1Id(m) => m.object_id(),
            Self::ZwpTabletV1Path(m) => m.object_id(),
            Self::ZwpTabletV1Done(m) => m.object_id(),
            Self::ZwpTabletV1Removed(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpTabletSeatV1TabletAdded(m) => m.opcode(),
            Self::ZwpTabletSeatV1ToolAdded(m) => m.opcode(),
            Self::ZwpTabletToolV1Type(m) => m.opcode(),
            Self::ZwpTabletToolV1HardwareSerial(m) => m.opcode(),
            Self::ZwpTabletToolV1HardwareIdWacom(m) => m.opcode(),
            Self::ZwpTabletToolV1Capability(m) => m.opcode(),
            Self::ZwpTabletToolV1Done(m) => m.opcode(),
            Self::ZwpTabletToolV1Removed(m) => m.opcode(),
            Self::ZwpTabletToolV1ProximityIn(m) => m.opcode(),
            Self::ZwpTabletToolV1ProximityOut(m) => m.opcode(),
            Self::ZwpTabletToolV1Down(m) => m.opcode(),
            Self::ZwpTabletToolV1Up(m) => m.opcode(),
            Self::ZwpTabletToolV1Motion(m) => m.opcode(),
            Self::ZwpTabletToolV1Pressure(m) => m.opcode(),
            Self::ZwpTabletToolV1Distance(m) => m.opcode(),
            Self::ZwpTabletToolV1Tilt(m) => m.opcode(),
            Self::ZwpTabletToolV1Rotation(m) => m.opcode(),
            Self::ZwpTabletToolV1Slider(m) => m.opcode(),
            Self::ZwpTabletToolV1Wheel(m) => m.opcode(),
            Self::ZwpTabletToolV1Button(m) => m.opcode(),
            Self::ZwpTabletToolV1Frame(m) => m.opcode(),
            Self::ZwpTabletV1Name(m) => m.opcode(),
            Self::ZwpTabletV1Id(m) => m.opcode(),
            Self::ZwpTabletV1Path(m) => m.opcode(),
            Self::ZwpTabletV1Done(m) => m.opcode(),
            Self::ZwpTabletV1Removed(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpTabletSeatV1TabletAdded(m) => m.payload_size(),
            Self::ZwpTabletSeatV1ToolAdded(m) => m.payload_size(),
            Self::ZwpTabletToolV1Type(m) => m.payload_size(),
            Self::ZwpTabletToolV1HardwareSerial(m) => m.payload_size(),
            Self::ZwpTabletToolV1HardwareIdWacom(m) => m.payload_size(),
            Self::ZwpTabletToolV1Capability(m) => m.payload_size(),
            Self::ZwpTabletToolV1Done(m) => m.payload_size(),
            Self::ZwpTabletToolV1Removed(m) => m.payload_size(),
            Self::ZwpTabletToolV1ProximityIn(m) => m.payload_size(),
            Self::ZwpTabletToolV1ProximityOut(m) => m.payload_size(),
            Self::ZwpTabletToolV1Down(m) => m.payload_size(),
            Self::ZwpTabletToolV1Up(m) => m.payload_size(),
            Self::ZwpTabletToolV1Motion(m) => m.payload_size(),
            Self::ZwpTabletToolV1Pressure(m) => m.payload_size(),
            Self::ZwpTabletToolV1Distance(m) => m.payload_size(),
            Self::ZwpTabletToolV1Tilt(m) => m.payload_size(),
            Self::ZwpTabletToolV1Rotation(m) => m.payload_size(),
            Self::ZwpTabletToolV1Slider(m) => m.payload_size(),
            Self::ZwpTabletToolV1Wheel(m) => m.payload_size(),
            Self::ZwpTabletToolV1Button(m) => m.payload_size(),
            Self::ZwpTabletToolV1Frame(m) => m.payload_size(),
            Self::ZwpTabletV1Name(m) => m.payload_size(),
            Self::ZwpTabletV1Id(m) => m.payload_size(),
            Self::ZwpTabletV1Path(m) => m.payload_size(),
            Self::ZwpTabletV1Done(m) => m.payload_size(),
            Self::ZwpTabletV1Removed(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpTabletSeatV1TabletAdded(m) => m.write(writer, state).await,
            Self::ZwpTabletSeatV1ToolAdded(m) => m.write(writer, state).await,
            Self::ZwpTabletToolV1Type(m) => m.write(writer, state).await,
            Self::ZwpTabletToolV1HardwareSerial(m) => m.write(writer, state).await,
            Self::ZwpTabletToolV1HardwareIdWacom(m) => m.write(writer, state).await,
            Self::ZwpTabletToolV1Capability(m) => m.write(writer, state).await,
            Self::ZwpTabletToolV1Done(m) => m.write(writer, state).await,
            Self::ZwpTabletToolV1Removed(m) => m.write(writer, state).await,
            Self::ZwpTabletToolV1ProximityIn(m) => m.write(writer, state).await,
            Self::ZwpTabletToolV1ProximityOut(m) => m.write(writer, state).await,
            Self::ZwpTabletToolV1Down(m) => m.write(writer, state).await,
            Self::ZwpTabletToolV1Up(m) => m.write(writer, state).await,
            Self::ZwpTabletToolV1Motion(m) => m.write(writer, state).await,
            Self::ZwpTabletToolV1Pressure(m) => m.write(writer, state).await,
            Self::ZwpTabletToolV1Distance(m) => m.write(writer, state).await,
            Self::ZwpTabletToolV1Tilt(m) => m.write(writer, state).await,
            Self::ZwpTabletToolV1Rotation(m) => m.write(writer, state).await,
            Self::ZwpTabletToolV1Slider(m) => m.write(writer, state).await,
            Self::ZwpTabletToolV1Wheel(m) => m.write(writer, state).await,
            Self::ZwpTabletToolV1Button(m) => m.write(writer, state).await,
            Self::ZwpTabletToolV1Frame(m) => m.write(writer, state).await,
            Self::ZwpTabletV1Name(m) => m.write(writer, state).await,
            Self::ZwpTabletV1Id(m) => m.write(writer, state).await,
            Self::ZwpTabletV1Path(m) => m.write(writer, state).await,
            Self::ZwpTabletV1Done(m) => m.write(writer, state).await,
            Self::ZwpTabletV1Removed(m) => m.write(writer, state).await,
        }
    }
}
