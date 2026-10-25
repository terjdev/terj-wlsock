use super::{
    ZwpTabletPadDialV2DeltaEvent, ZwpTabletPadDialV2FrameEvent, ZwpTabletPadGroupV2ButtonsEvent,
    ZwpTabletPadGroupV2DialEvent, ZwpTabletPadGroupV2DoneEvent, ZwpTabletPadGroupV2ModeSwitchEvent,
    ZwpTabletPadGroupV2ModesEvent, ZwpTabletPadGroupV2RingEvent, ZwpTabletPadGroupV2StripEvent,
    ZwpTabletPadRingV2AngleEvent, ZwpTabletPadRingV2FrameEvent, ZwpTabletPadRingV2SourceEvent,
    ZwpTabletPadRingV2StopEvent, ZwpTabletPadStripV2FrameEvent, ZwpTabletPadStripV2PositionEvent,
    ZwpTabletPadStripV2SourceEvent, ZwpTabletPadStripV2StopEvent, ZwpTabletPadV2ButtonEvent,
    ZwpTabletPadV2ButtonsEvent, ZwpTabletPadV2DoneEvent, ZwpTabletPadV2EnterEvent,
    ZwpTabletPadV2GroupEvent, ZwpTabletPadV2LeaveEvent, ZwpTabletPadV2PathEvent,
    ZwpTabletPadV2RemovedEvent, ZwpTabletSeatV2PadAddedEvent, ZwpTabletSeatV2TabletAddedEvent,
    ZwpTabletSeatV2ToolAddedEvent, ZwpTabletToolV2ButtonEvent, ZwpTabletToolV2CapabilityEvent,
    ZwpTabletToolV2DistanceEvent, ZwpTabletToolV2DoneEvent, ZwpTabletToolV2DownEvent,
    ZwpTabletToolV2FrameEvent, ZwpTabletToolV2HardwareIdWacomEvent,
    ZwpTabletToolV2HardwareSerialEvent, ZwpTabletToolV2MotionEvent, ZwpTabletToolV2PressureEvent,
    ZwpTabletToolV2ProximityInEvent, ZwpTabletToolV2ProximityOutEvent, ZwpTabletToolV2RemovedEvent,
    ZwpTabletToolV2RotationEvent, ZwpTabletToolV2SliderEvent, ZwpTabletToolV2TiltEvent,
    ZwpTabletToolV2TypeEvent, ZwpTabletToolV2UpEvent, ZwpTabletToolV2WheelEvent,
    ZwpTabletV2BustypeEvent, ZwpTabletV2DoneEvent, ZwpTabletV2IdEvent, ZwpTabletV2NameEvent,
    ZwpTabletV2PathEvent, ZwpTabletV2RemovedEvent,
};
use crate::protocol::ext::tablet::ObjectInteface as TabletObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Tablet events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZwpTabletSeatV2TabletAddedEvent].
    ZwpTabletSeatV2TabletAdded(ZwpTabletSeatV2TabletAddedEvent),
    /// Message [ZwpTabletSeatV2ToolAddedEvent].
    ZwpTabletSeatV2ToolAdded(ZwpTabletSeatV2ToolAddedEvent),
    /// Message [ZwpTabletSeatV2PadAddedEvent].
    ZwpTabletSeatV2PadAdded(ZwpTabletSeatV2PadAddedEvent),
    /// Message [ZwpTabletToolV2TypeEvent].
    ZwpTabletToolV2Type(ZwpTabletToolV2TypeEvent),
    /// Message [ZwpTabletToolV2HardwareSerialEvent].
    ZwpTabletToolV2HardwareSerial(ZwpTabletToolV2HardwareSerialEvent),
    /// Message [ZwpTabletToolV2HardwareIdWacomEvent].
    ZwpTabletToolV2HardwareIdWacom(ZwpTabletToolV2HardwareIdWacomEvent),
    /// Message [ZwpTabletToolV2CapabilityEvent].
    ZwpTabletToolV2Capability(ZwpTabletToolV2CapabilityEvent),
    /// Message [ZwpTabletToolV2DoneEvent].
    ZwpTabletToolV2Done(ZwpTabletToolV2DoneEvent),
    /// Message [ZwpTabletToolV2RemovedEvent].
    ZwpTabletToolV2Removed(ZwpTabletToolV2RemovedEvent),
    /// Message [ZwpTabletToolV2ProximityInEvent].
    ZwpTabletToolV2ProximityIn(ZwpTabletToolV2ProximityInEvent),
    /// Message [ZwpTabletToolV2ProximityOutEvent].
    ZwpTabletToolV2ProximityOut(ZwpTabletToolV2ProximityOutEvent),
    /// Message [ZwpTabletToolV2DownEvent].
    ZwpTabletToolV2Down(ZwpTabletToolV2DownEvent),
    /// Message [ZwpTabletToolV2UpEvent].
    ZwpTabletToolV2Up(ZwpTabletToolV2UpEvent),
    /// Message [ZwpTabletToolV2MotionEvent].
    ZwpTabletToolV2Motion(ZwpTabletToolV2MotionEvent),
    /// Message [ZwpTabletToolV2PressureEvent].
    ZwpTabletToolV2Pressure(ZwpTabletToolV2PressureEvent),
    /// Message [ZwpTabletToolV2DistanceEvent].
    ZwpTabletToolV2Distance(ZwpTabletToolV2DistanceEvent),
    /// Message [ZwpTabletToolV2TiltEvent].
    ZwpTabletToolV2Tilt(ZwpTabletToolV2TiltEvent),
    /// Message [ZwpTabletToolV2RotationEvent].
    ZwpTabletToolV2Rotation(ZwpTabletToolV2RotationEvent),
    /// Message [ZwpTabletToolV2SliderEvent].
    ZwpTabletToolV2Slider(ZwpTabletToolV2SliderEvent),
    /// Message [ZwpTabletToolV2WheelEvent].
    ZwpTabletToolV2Wheel(ZwpTabletToolV2WheelEvent),
    /// Message [ZwpTabletToolV2ButtonEvent].
    ZwpTabletToolV2Button(ZwpTabletToolV2ButtonEvent),
    /// Message [ZwpTabletToolV2FrameEvent].
    ZwpTabletToolV2Frame(ZwpTabletToolV2FrameEvent),
    /// Message [ZwpTabletV2NameEvent].
    ZwpTabletV2Name(ZwpTabletV2NameEvent),
    /// Message [ZwpTabletV2IdEvent].
    ZwpTabletV2Id(ZwpTabletV2IdEvent),
    /// Message [ZwpTabletV2PathEvent].
    ZwpTabletV2Path(ZwpTabletV2PathEvent),
    /// Message [ZwpTabletV2DoneEvent].
    ZwpTabletV2Done(ZwpTabletV2DoneEvent),
    /// Message [ZwpTabletV2RemovedEvent].
    ZwpTabletV2Removed(ZwpTabletV2RemovedEvent),
    /// Message [ZwpTabletV2BustypeEvent].
    ZwpTabletV2Bustype(ZwpTabletV2BustypeEvent),
    /// Message [ZwpTabletPadRingV2SourceEvent].
    ZwpTabletPadRingV2Source(ZwpTabletPadRingV2SourceEvent),
    /// Message [ZwpTabletPadRingV2AngleEvent].
    ZwpTabletPadRingV2Angle(ZwpTabletPadRingV2AngleEvent),
    /// Message [ZwpTabletPadRingV2StopEvent].
    ZwpTabletPadRingV2Stop(ZwpTabletPadRingV2StopEvent),
    /// Message [ZwpTabletPadRingV2FrameEvent].
    ZwpTabletPadRingV2Frame(ZwpTabletPadRingV2FrameEvent),
    /// Message [ZwpTabletPadStripV2SourceEvent].
    ZwpTabletPadStripV2Source(ZwpTabletPadStripV2SourceEvent),
    /// Message [ZwpTabletPadStripV2PositionEvent].
    ZwpTabletPadStripV2Position(ZwpTabletPadStripV2PositionEvent),
    /// Message [ZwpTabletPadStripV2StopEvent].
    ZwpTabletPadStripV2Stop(ZwpTabletPadStripV2StopEvent),
    /// Message [ZwpTabletPadStripV2FrameEvent].
    ZwpTabletPadStripV2Frame(ZwpTabletPadStripV2FrameEvent),
    /// Message [ZwpTabletPadGroupV2ButtonsEvent].
    ZwpTabletPadGroupV2Buttons(ZwpTabletPadGroupV2ButtonsEvent),
    /// Message [ZwpTabletPadGroupV2RingEvent].
    ZwpTabletPadGroupV2Ring(ZwpTabletPadGroupV2RingEvent),
    /// Message [ZwpTabletPadGroupV2StripEvent].
    ZwpTabletPadGroupV2Strip(ZwpTabletPadGroupV2StripEvent),
    /// Message [ZwpTabletPadGroupV2ModesEvent].
    ZwpTabletPadGroupV2Modes(ZwpTabletPadGroupV2ModesEvent),
    /// Message [ZwpTabletPadGroupV2DoneEvent].
    ZwpTabletPadGroupV2Done(ZwpTabletPadGroupV2DoneEvent),
    /// Message [ZwpTabletPadGroupV2ModeSwitchEvent].
    ZwpTabletPadGroupV2ModeSwitch(ZwpTabletPadGroupV2ModeSwitchEvent),
    /// Message [ZwpTabletPadGroupV2DialEvent].
    ZwpTabletPadGroupV2Dial(ZwpTabletPadGroupV2DialEvent),
    /// Message [ZwpTabletPadV2GroupEvent].
    ZwpTabletPadV2Group(ZwpTabletPadV2GroupEvent),
    /// Message [ZwpTabletPadV2PathEvent].
    ZwpTabletPadV2Path(ZwpTabletPadV2PathEvent),
    /// Message [ZwpTabletPadV2ButtonsEvent].
    ZwpTabletPadV2Buttons(ZwpTabletPadV2ButtonsEvent),
    /// Message [ZwpTabletPadV2DoneEvent].
    ZwpTabletPadV2Done(ZwpTabletPadV2DoneEvent),
    /// Message [ZwpTabletPadV2ButtonEvent].
    ZwpTabletPadV2Button(ZwpTabletPadV2ButtonEvent),
    /// Message [ZwpTabletPadV2EnterEvent].
    ZwpTabletPadV2Enter(ZwpTabletPadV2EnterEvent),
    /// Message [ZwpTabletPadV2LeaveEvent].
    ZwpTabletPadV2Leave(ZwpTabletPadV2LeaveEvent),
    /// Message [ZwpTabletPadV2RemovedEvent].
    ZwpTabletPadV2Removed(ZwpTabletPadV2RemovedEvent),
    /// Message [ZwpTabletPadDialV2DeltaEvent].
    ZwpTabletPadDialV2Delta(ZwpTabletPadDialV2DeltaEvent),
    /// Message [ZwpTabletPadDialV2FrameEvent].
    ZwpTabletPadDialV2Frame(ZwpTabletPadDialV2FrameEvent),
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
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletSeatV2 { .. }) => match opcode {
                ZwpTabletSeatV2TabletAddedEvent::OPCODE => Ok(Self::ZwpTabletSeatV2TabletAdded(
                    ZwpTabletSeatV2TabletAddedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletSeatV2ToolAddedEvent::OPCODE => Ok(Self::ZwpTabletSeatV2ToolAdded(
                    ZwpTabletSeatV2ToolAddedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletSeatV2PadAddedEvent::OPCODE => Ok(Self::ZwpTabletSeatV2PadAdded(
                    ZwpTabletSeatV2PadAddedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletToolV2 { .. }) => match opcode {
                ZwpTabletToolV2TypeEvent::OPCODE => Ok(Self::ZwpTabletToolV2Type(
                    ZwpTabletToolV2TypeEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV2HardwareSerialEvent::OPCODE => {
                    Ok(Self::ZwpTabletToolV2HardwareSerial(
                        ZwpTabletToolV2HardwareSerialEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpTabletToolV2HardwareIdWacomEvent::OPCODE => {
                    Ok(Self::ZwpTabletToolV2HardwareIdWacom(
                        ZwpTabletToolV2HardwareIdWacomEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpTabletToolV2CapabilityEvent::OPCODE => Ok(Self::ZwpTabletToolV2Capability(
                    ZwpTabletToolV2CapabilityEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV2DoneEvent::OPCODE => Ok(Self::ZwpTabletToolV2Done(
                    ZwpTabletToolV2DoneEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV2RemovedEvent::OPCODE => Ok(Self::ZwpTabletToolV2Removed(
                    ZwpTabletToolV2RemovedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV2ProximityInEvent::OPCODE => Ok(Self::ZwpTabletToolV2ProximityIn(
                    ZwpTabletToolV2ProximityInEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV2ProximityOutEvent::OPCODE => Ok(Self::ZwpTabletToolV2ProximityOut(
                    ZwpTabletToolV2ProximityOutEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV2DownEvent::OPCODE => Ok(Self::ZwpTabletToolV2Down(
                    ZwpTabletToolV2DownEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV2UpEvent::OPCODE => Ok(Self::ZwpTabletToolV2Up(
                    ZwpTabletToolV2UpEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV2MotionEvent::OPCODE => Ok(Self::ZwpTabletToolV2Motion(
                    ZwpTabletToolV2MotionEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV2PressureEvent::OPCODE => Ok(Self::ZwpTabletToolV2Pressure(
                    ZwpTabletToolV2PressureEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV2DistanceEvent::OPCODE => Ok(Self::ZwpTabletToolV2Distance(
                    ZwpTabletToolV2DistanceEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV2TiltEvent::OPCODE => Ok(Self::ZwpTabletToolV2Tilt(
                    ZwpTabletToolV2TiltEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV2RotationEvent::OPCODE => Ok(Self::ZwpTabletToolV2Rotation(
                    ZwpTabletToolV2RotationEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV2SliderEvent::OPCODE => Ok(Self::ZwpTabletToolV2Slider(
                    ZwpTabletToolV2SliderEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV2WheelEvent::OPCODE => Ok(Self::ZwpTabletToolV2Wheel(
                    ZwpTabletToolV2WheelEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV2ButtonEvent::OPCODE => Ok(Self::ZwpTabletToolV2Button(
                    ZwpTabletToolV2ButtonEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletToolV2FrameEvent::OPCODE => Ok(Self::ZwpTabletToolV2Frame(
                    ZwpTabletToolV2FrameEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletV2 { .. }) => match opcode {
                ZwpTabletV2NameEvent::OPCODE => Ok(Self::ZwpTabletV2Name(
                    ZwpTabletV2NameEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletV2IdEvent::OPCODE => Ok(Self::ZwpTabletV2Id(
                    ZwpTabletV2IdEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletV2PathEvent::OPCODE => Ok(Self::ZwpTabletV2Path(
                    ZwpTabletV2PathEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletV2DoneEvent::OPCODE => Ok(Self::ZwpTabletV2Done(
                    ZwpTabletV2DoneEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletV2RemovedEvent::OPCODE => Ok(Self::ZwpTabletV2Removed(
                    ZwpTabletV2RemovedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletV2BustypeEvent::OPCODE => Ok(Self::ZwpTabletV2Bustype(
                    ZwpTabletV2BustypeEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletPadRingV2 { .. }) => match opcode
            {
                ZwpTabletPadRingV2SourceEvent::OPCODE => Ok(Self::ZwpTabletPadRingV2Source(
                    ZwpTabletPadRingV2SourceEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletPadRingV2AngleEvent::OPCODE => Ok(Self::ZwpTabletPadRingV2Angle(
                    ZwpTabletPadRingV2AngleEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletPadRingV2StopEvent::OPCODE => Ok(Self::ZwpTabletPadRingV2Stop(
                    ZwpTabletPadRingV2StopEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletPadRingV2FrameEvent::OPCODE => Ok(Self::ZwpTabletPadRingV2Frame(
                    ZwpTabletPadRingV2FrameEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletPadStripV2 { .. }) => {
                match opcode {
                    ZwpTabletPadStripV2SourceEvent::OPCODE => Ok(Self::ZwpTabletPadStripV2Source(
                        ZwpTabletPadStripV2SourceEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTabletPadStripV2PositionEvent::OPCODE => {
                        Ok(Self::ZwpTabletPadStripV2Position(
                            ZwpTabletPadStripV2PositionEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTabletPadStripV2StopEvent::OPCODE => Ok(Self::ZwpTabletPadStripV2Stop(
                        ZwpTabletPadStripV2StopEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTabletPadStripV2FrameEvent::OPCODE => Ok(Self::ZwpTabletPadStripV2Frame(
                        ZwpTabletPadStripV2FrameEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletPadGroupV2 { .. }) => {
                match opcode {
                    ZwpTabletPadGroupV2ButtonsEvent::OPCODE => {
                        Ok(Self::ZwpTabletPadGroupV2Buttons(
                            ZwpTabletPadGroupV2ButtonsEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTabletPadGroupV2RingEvent::OPCODE => Ok(Self::ZwpTabletPadGroupV2Ring(
                        ZwpTabletPadGroupV2RingEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTabletPadGroupV2StripEvent::OPCODE => Ok(Self::ZwpTabletPadGroupV2Strip(
                        ZwpTabletPadGroupV2StripEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTabletPadGroupV2ModesEvent::OPCODE => Ok(Self::ZwpTabletPadGroupV2Modes(
                        ZwpTabletPadGroupV2ModesEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTabletPadGroupV2DoneEvent::OPCODE => Ok(Self::ZwpTabletPadGroupV2Done(
                        ZwpTabletPadGroupV2DoneEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTabletPadGroupV2ModeSwitchEvent::OPCODE => {
                        Ok(Self::ZwpTabletPadGroupV2ModeSwitch(
                            ZwpTabletPadGroupV2ModeSwitchEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTabletPadGroupV2DialEvent::OPCODE => Ok(Self::ZwpTabletPadGroupV2Dial(
                        ZwpTabletPadGroupV2DialEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletPadV2 { .. }) => match opcode {
                ZwpTabletPadV2GroupEvent::OPCODE => Ok(Self::ZwpTabletPadV2Group(
                    ZwpTabletPadV2GroupEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletPadV2PathEvent::OPCODE => Ok(Self::ZwpTabletPadV2Path(
                    ZwpTabletPadV2PathEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletPadV2ButtonsEvent::OPCODE => Ok(Self::ZwpTabletPadV2Buttons(
                    ZwpTabletPadV2ButtonsEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletPadV2DoneEvent::OPCODE => Ok(Self::ZwpTabletPadV2Done(
                    ZwpTabletPadV2DoneEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletPadV2ButtonEvent::OPCODE => Ok(Self::ZwpTabletPadV2Button(
                    ZwpTabletPadV2ButtonEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletPadV2EnterEvent::OPCODE => Ok(Self::ZwpTabletPadV2Enter(
                    ZwpTabletPadV2EnterEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletPadV2LeaveEvent::OPCODE => Ok(Self::ZwpTabletPadV2Leave(
                    ZwpTabletPadV2LeaveEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletPadV2RemovedEvent::OPCODE => Ok(Self::ZwpTabletPadV2Removed(
                    ZwpTabletPadV2RemovedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletPadDialV2 { .. }) => match opcode
            {
                ZwpTabletPadDialV2DeltaEvent::OPCODE => Ok(Self::ZwpTabletPadDialV2Delta(
                    ZwpTabletPadDialV2DeltaEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpTabletPadDialV2FrameEvent::OPCODE => Ok(Self::ZwpTabletPadDialV2Frame(
                    ZwpTabletPadDialV2FrameEvent::read_with_interface(
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
            Self::ZwpTabletSeatV2TabletAdded(message) => message.object_id(),
            Self::ZwpTabletSeatV2ToolAdded(message) => message.object_id(),
            Self::ZwpTabletSeatV2PadAdded(message) => message.object_id(),
            Self::ZwpTabletToolV2Type(message) => message.object_id(),
            Self::ZwpTabletToolV2HardwareSerial(message) => message.object_id(),
            Self::ZwpTabletToolV2HardwareIdWacom(message) => message.object_id(),
            Self::ZwpTabletToolV2Capability(message) => message.object_id(),
            Self::ZwpTabletToolV2Done(message) => message.object_id(),
            Self::ZwpTabletToolV2Removed(message) => message.object_id(),
            Self::ZwpTabletToolV2ProximityIn(message) => message.object_id(),
            Self::ZwpTabletToolV2ProximityOut(message) => message.object_id(),
            Self::ZwpTabletToolV2Down(message) => message.object_id(),
            Self::ZwpTabletToolV2Up(message) => message.object_id(),
            Self::ZwpTabletToolV2Motion(message) => message.object_id(),
            Self::ZwpTabletToolV2Pressure(message) => message.object_id(),
            Self::ZwpTabletToolV2Distance(message) => message.object_id(),
            Self::ZwpTabletToolV2Tilt(message) => message.object_id(),
            Self::ZwpTabletToolV2Rotation(message) => message.object_id(),
            Self::ZwpTabletToolV2Slider(message) => message.object_id(),
            Self::ZwpTabletToolV2Wheel(message) => message.object_id(),
            Self::ZwpTabletToolV2Button(message) => message.object_id(),
            Self::ZwpTabletToolV2Frame(message) => message.object_id(),
            Self::ZwpTabletV2Name(message) => message.object_id(),
            Self::ZwpTabletV2Id(message) => message.object_id(),
            Self::ZwpTabletV2Path(message) => message.object_id(),
            Self::ZwpTabletV2Done(message) => message.object_id(),
            Self::ZwpTabletV2Removed(message) => message.object_id(),
            Self::ZwpTabletV2Bustype(message) => message.object_id(),
            Self::ZwpTabletPadRingV2Source(message) => message.object_id(),
            Self::ZwpTabletPadRingV2Angle(message) => message.object_id(),
            Self::ZwpTabletPadRingV2Stop(message) => message.object_id(),
            Self::ZwpTabletPadRingV2Frame(message) => message.object_id(),
            Self::ZwpTabletPadStripV2Source(message) => message.object_id(),
            Self::ZwpTabletPadStripV2Position(message) => message.object_id(),
            Self::ZwpTabletPadStripV2Stop(message) => message.object_id(),
            Self::ZwpTabletPadStripV2Frame(message) => message.object_id(),
            Self::ZwpTabletPadGroupV2Buttons(message) => message.object_id(),
            Self::ZwpTabletPadGroupV2Ring(message) => message.object_id(),
            Self::ZwpTabletPadGroupV2Strip(message) => message.object_id(),
            Self::ZwpTabletPadGroupV2Modes(message) => message.object_id(),
            Self::ZwpTabletPadGroupV2Done(message) => message.object_id(),
            Self::ZwpTabletPadGroupV2ModeSwitch(message) => message.object_id(),
            Self::ZwpTabletPadGroupV2Dial(message) => message.object_id(),
            Self::ZwpTabletPadV2Group(message) => message.object_id(),
            Self::ZwpTabletPadV2Path(message) => message.object_id(),
            Self::ZwpTabletPadV2Buttons(message) => message.object_id(),
            Self::ZwpTabletPadV2Done(message) => message.object_id(),
            Self::ZwpTabletPadV2Button(message) => message.object_id(),
            Self::ZwpTabletPadV2Enter(message) => message.object_id(),
            Self::ZwpTabletPadV2Leave(message) => message.object_id(),
            Self::ZwpTabletPadV2Removed(message) => message.object_id(),
            Self::ZwpTabletPadDialV2Delta(message) => message.object_id(),
            Self::ZwpTabletPadDialV2Frame(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpTabletSeatV2TabletAdded(message) => message.opcode(),
            Self::ZwpTabletSeatV2ToolAdded(message) => message.opcode(),
            Self::ZwpTabletSeatV2PadAdded(message) => message.opcode(),
            Self::ZwpTabletToolV2Type(message) => message.opcode(),
            Self::ZwpTabletToolV2HardwareSerial(message) => message.opcode(),
            Self::ZwpTabletToolV2HardwareIdWacom(message) => message.opcode(),
            Self::ZwpTabletToolV2Capability(message) => message.opcode(),
            Self::ZwpTabletToolV2Done(message) => message.opcode(),
            Self::ZwpTabletToolV2Removed(message) => message.opcode(),
            Self::ZwpTabletToolV2ProximityIn(message) => message.opcode(),
            Self::ZwpTabletToolV2ProximityOut(message) => message.opcode(),
            Self::ZwpTabletToolV2Down(message) => message.opcode(),
            Self::ZwpTabletToolV2Up(message) => message.opcode(),
            Self::ZwpTabletToolV2Motion(message) => message.opcode(),
            Self::ZwpTabletToolV2Pressure(message) => message.opcode(),
            Self::ZwpTabletToolV2Distance(message) => message.opcode(),
            Self::ZwpTabletToolV2Tilt(message) => message.opcode(),
            Self::ZwpTabletToolV2Rotation(message) => message.opcode(),
            Self::ZwpTabletToolV2Slider(message) => message.opcode(),
            Self::ZwpTabletToolV2Wheel(message) => message.opcode(),
            Self::ZwpTabletToolV2Button(message) => message.opcode(),
            Self::ZwpTabletToolV2Frame(message) => message.opcode(),
            Self::ZwpTabletV2Name(message) => message.opcode(),
            Self::ZwpTabletV2Id(message) => message.opcode(),
            Self::ZwpTabletV2Path(message) => message.opcode(),
            Self::ZwpTabletV2Done(message) => message.opcode(),
            Self::ZwpTabletV2Removed(message) => message.opcode(),
            Self::ZwpTabletV2Bustype(message) => message.opcode(),
            Self::ZwpTabletPadRingV2Source(message) => message.opcode(),
            Self::ZwpTabletPadRingV2Angle(message) => message.opcode(),
            Self::ZwpTabletPadRingV2Stop(message) => message.opcode(),
            Self::ZwpTabletPadRingV2Frame(message) => message.opcode(),
            Self::ZwpTabletPadStripV2Source(message) => message.opcode(),
            Self::ZwpTabletPadStripV2Position(message) => message.opcode(),
            Self::ZwpTabletPadStripV2Stop(message) => message.opcode(),
            Self::ZwpTabletPadStripV2Frame(message) => message.opcode(),
            Self::ZwpTabletPadGroupV2Buttons(message) => message.opcode(),
            Self::ZwpTabletPadGroupV2Ring(message) => message.opcode(),
            Self::ZwpTabletPadGroupV2Strip(message) => message.opcode(),
            Self::ZwpTabletPadGroupV2Modes(message) => message.opcode(),
            Self::ZwpTabletPadGroupV2Done(message) => message.opcode(),
            Self::ZwpTabletPadGroupV2ModeSwitch(message) => message.opcode(),
            Self::ZwpTabletPadGroupV2Dial(message) => message.opcode(),
            Self::ZwpTabletPadV2Group(message) => message.opcode(),
            Self::ZwpTabletPadV2Path(message) => message.opcode(),
            Self::ZwpTabletPadV2Buttons(message) => message.opcode(),
            Self::ZwpTabletPadV2Done(message) => message.opcode(),
            Self::ZwpTabletPadV2Button(message) => message.opcode(),
            Self::ZwpTabletPadV2Enter(message) => message.opcode(),
            Self::ZwpTabletPadV2Leave(message) => message.opcode(),
            Self::ZwpTabletPadV2Removed(message) => message.opcode(),
            Self::ZwpTabletPadDialV2Delta(message) => message.opcode(),
            Self::ZwpTabletPadDialV2Frame(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpTabletSeatV2TabletAdded(message) => message.payload_size(),
            Self::ZwpTabletSeatV2ToolAdded(message) => message.payload_size(),
            Self::ZwpTabletSeatV2PadAdded(message) => message.payload_size(),
            Self::ZwpTabletToolV2Type(message) => message.payload_size(),
            Self::ZwpTabletToolV2HardwareSerial(message) => message.payload_size(),
            Self::ZwpTabletToolV2HardwareIdWacom(message) => message.payload_size(),
            Self::ZwpTabletToolV2Capability(message) => message.payload_size(),
            Self::ZwpTabletToolV2Done(message) => message.payload_size(),
            Self::ZwpTabletToolV2Removed(message) => message.payload_size(),
            Self::ZwpTabletToolV2ProximityIn(message) => message.payload_size(),
            Self::ZwpTabletToolV2ProximityOut(message) => message.payload_size(),
            Self::ZwpTabletToolV2Down(message) => message.payload_size(),
            Self::ZwpTabletToolV2Up(message) => message.payload_size(),
            Self::ZwpTabletToolV2Motion(message) => message.payload_size(),
            Self::ZwpTabletToolV2Pressure(message) => message.payload_size(),
            Self::ZwpTabletToolV2Distance(message) => message.payload_size(),
            Self::ZwpTabletToolV2Tilt(message) => message.payload_size(),
            Self::ZwpTabletToolV2Rotation(message) => message.payload_size(),
            Self::ZwpTabletToolV2Slider(message) => message.payload_size(),
            Self::ZwpTabletToolV2Wheel(message) => message.payload_size(),
            Self::ZwpTabletToolV2Button(message) => message.payload_size(),
            Self::ZwpTabletToolV2Frame(message) => message.payload_size(),
            Self::ZwpTabletV2Name(message) => message.payload_size(),
            Self::ZwpTabletV2Id(message) => message.payload_size(),
            Self::ZwpTabletV2Path(message) => message.payload_size(),
            Self::ZwpTabletV2Done(message) => message.payload_size(),
            Self::ZwpTabletV2Removed(message) => message.payload_size(),
            Self::ZwpTabletV2Bustype(message) => message.payload_size(),
            Self::ZwpTabletPadRingV2Source(message) => message.payload_size(),
            Self::ZwpTabletPadRingV2Angle(message) => message.payload_size(),
            Self::ZwpTabletPadRingV2Stop(message) => message.payload_size(),
            Self::ZwpTabletPadRingV2Frame(message) => message.payload_size(),
            Self::ZwpTabletPadStripV2Source(message) => message.payload_size(),
            Self::ZwpTabletPadStripV2Position(message) => message.payload_size(),
            Self::ZwpTabletPadStripV2Stop(message) => message.payload_size(),
            Self::ZwpTabletPadStripV2Frame(message) => message.payload_size(),
            Self::ZwpTabletPadGroupV2Buttons(message) => message.payload_size(),
            Self::ZwpTabletPadGroupV2Ring(message) => message.payload_size(),
            Self::ZwpTabletPadGroupV2Strip(message) => message.payload_size(),
            Self::ZwpTabletPadGroupV2Modes(message) => message.payload_size(),
            Self::ZwpTabletPadGroupV2Done(message) => message.payload_size(),
            Self::ZwpTabletPadGroupV2ModeSwitch(message) => message.payload_size(),
            Self::ZwpTabletPadGroupV2Dial(message) => message.payload_size(),
            Self::ZwpTabletPadV2Group(message) => message.payload_size(),
            Self::ZwpTabletPadV2Path(message) => message.payload_size(),
            Self::ZwpTabletPadV2Buttons(message) => message.payload_size(),
            Self::ZwpTabletPadV2Done(message) => message.payload_size(),
            Self::ZwpTabletPadV2Button(message) => message.payload_size(),
            Self::ZwpTabletPadV2Enter(message) => message.payload_size(),
            Self::ZwpTabletPadV2Leave(message) => message.payload_size(),
            Self::ZwpTabletPadV2Removed(message) => message.payload_size(),
            Self::ZwpTabletPadDialV2Delta(message) => message.payload_size(),
            Self::ZwpTabletPadDialV2Frame(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpTabletSeatV2TabletAdded(message) => message.write(writer, state).await,
            Self::ZwpTabletSeatV2ToolAdded(message) => message.write(writer, state).await,
            Self::ZwpTabletSeatV2PadAdded(message) => message.write(writer, state).await,
            Self::ZwpTabletToolV2Type(message) => message.write(writer, state).await,
            Self::ZwpTabletToolV2HardwareSerial(message) => message.write(writer, state).await,
            Self::ZwpTabletToolV2HardwareIdWacom(message) => message.write(writer, state).await,
            Self::ZwpTabletToolV2Capability(message) => message.write(writer, state).await,
            Self::ZwpTabletToolV2Done(message) => message.write(writer, state).await,
            Self::ZwpTabletToolV2Removed(message) => message.write(writer, state).await,
            Self::ZwpTabletToolV2ProximityIn(message) => message.write(writer, state).await,
            Self::ZwpTabletToolV2ProximityOut(message) => message.write(writer, state).await,
            Self::ZwpTabletToolV2Down(message) => message.write(writer, state).await,
            Self::ZwpTabletToolV2Up(message) => message.write(writer, state).await,
            Self::ZwpTabletToolV2Motion(message) => message.write(writer, state).await,
            Self::ZwpTabletToolV2Pressure(message) => message.write(writer, state).await,
            Self::ZwpTabletToolV2Distance(message) => message.write(writer, state).await,
            Self::ZwpTabletToolV2Tilt(message) => message.write(writer, state).await,
            Self::ZwpTabletToolV2Rotation(message) => message.write(writer, state).await,
            Self::ZwpTabletToolV2Slider(message) => message.write(writer, state).await,
            Self::ZwpTabletToolV2Wheel(message) => message.write(writer, state).await,
            Self::ZwpTabletToolV2Button(message) => message.write(writer, state).await,
            Self::ZwpTabletToolV2Frame(message) => message.write(writer, state).await,
            Self::ZwpTabletV2Name(message) => message.write(writer, state).await,
            Self::ZwpTabletV2Id(message) => message.write(writer, state).await,
            Self::ZwpTabletV2Path(message) => message.write(writer, state).await,
            Self::ZwpTabletV2Done(message) => message.write(writer, state).await,
            Self::ZwpTabletV2Removed(message) => message.write(writer, state).await,
            Self::ZwpTabletV2Bustype(message) => message.write(writer, state).await,
            Self::ZwpTabletPadRingV2Source(message) => message.write(writer, state).await,
            Self::ZwpTabletPadRingV2Angle(message) => message.write(writer, state).await,
            Self::ZwpTabletPadRingV2Stop(message) => message.write(writer, state).await,
            Self::ZwpTabletPadRingV2Frame(message) => message.write(writer, state).await,
            Self::ZwpTabletPadStripV2Source(message) => message.write(writer, state).await,
            Self::ZwpTabletPadStripV2Position(message) => message.write(writer, state).await,
            Self::ZwpTabletPadStripV2Stop(message) => message.write(writer, state).await,
            Self::ZwpTabletPadStripV2Frame(message) => message.write(writer, state).await,
            Self::ZwpTabletPadGroupV2Buttons(message) => message.write(writer, state).await,
            Self::ZwpTabletPadGroupV2Ring(message) => message.write(writer, state).await,
            Self::ZwpTabletPadGroupV2Strip(message) => message.write(writer, state).await,
            Self::ZwpTabletPadGroupV2Modes(message) => message.write(writer, state).await,
            Self::ZwpTabletPadGroupV2Done(message) => message.write(writer, state).await,
            Self::ZwpTabletPadGroupV2ModeSwitch(message) => message.write(writer, state).await,
            Self::ZwpTabletPadGroupV2Dial(message) => message.write(writer, state).await,
            Self::ZwpTabletPadV2Group(message) => message.write(writer, state).await,
            Self::ZwpTabletPadV2Path(message) => message.write(writer, state).await,
            Self::ZwpTabletPadV2Buttons(message) => message.write(writer, state).await,
            Self::ZwpTabletPadV2Done(message) => message.write(writer, state).await,
            Self::ZwpTabletPadV2Button(message) => message.write(writer, state).await,
            Self::ZwpTabletPadV2Enter(message) => message.write(writer, state).await,
            Self::ZwpTabletPadV2Leave(message) => message.write(writer, state).await,
            Self::ZwpTabletPadV2Removed(message) => message.write(writer, state).await,
            Self::ZwpTabletPadDialV2Delta(message) => message.write(writer, state).await,
            Self::ZwpTabletPadDialV2Frame(message) => message.write(writer, state).await,
        }
    }
}
