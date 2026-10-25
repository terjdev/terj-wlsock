use crate::protocol::core::ObjectInteface as CoreObjectInteface;
use crate::protocol::core::*;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Contains core Wayland events and delegates their encoding and decoding.
#[derive(Debug)]
pub enum CommandEvent {
    /// Event [WlDisplayErrorEvent].
    WlDisplayError(WlDisplayErrorEvent),
    /// Event [WlDisplayDeleteIdEvent].
    WlDisplayDeleteId(WlDisplayDeleteIdEvent),
    /// Event [WlRegistryGlobalEvent].
    WlRegistryGlobal(WlRegistryGlobalEvent),
    /// Event [WlRegistryGlobalRemoveEvent].
    WlRegistryGlobalRemove(WlRegistryGlobalRemoveEvent),
    /// Event [WlCallbackDoneEvent].
    WlCallbackDone(WlCallbackDoneEvent),
    /// Event [WlShmFormatEvent].
    WlShmFormat(WlShmFormatEvent),
    /// Event [WlBufferReleaseEvent].
    WlBufferRelease(WlBufferReleaseEvent),
    /// Event [WlDataOfferOfferEvent].
    WlDataOfferOffer(WlDataOfferOfferEvent),
    /// Event [WlDataOfferSourceActionsEvent].
    WlDataOfferSourceActions(WlDataOfferSourceActionsEvent),
    /// Event [WlDataOfferActionEvent].
    WlDataOfferAction(WlDataOfferActionEvent),
    /// Event [WlDataSourceTargetEvent].
    WlDataSourceTarget(WlDataSourceTargetEvent),
    /// Event [WlDataSourceSendEvent].
    WlDataSourceSend(WlDataSourceSendEvent),
    /// Event [WlDataSourceCancelledEvent].
    WlDataSourceCancelled(WlDataSourceCancelledEvent),
    /// Event [WlDataSourceDndDropPerformedEvent].
    WlDataSourceDndDropPerformed(WlDataSourceDndDropPerformedEvent),
    /// Event [WlDataSourceDndFinishedEvent].
    WlDataSourceDndFinished(WlDataSourceDndFinishedEvent),
    /// Event [WlDataSourceActionEvent].
    WlDataSourceAction(WlDataSourceActionEvent),
    /// Event [WlDataDeviceDataOfferEvent].
    WlDataDeviceDataOffer(WlDataDeviceDataOfferEvent),
    /// Event [WlDataDeviceEnterEvent].
    WlDataDeviceEnter(WlDataDeviceEnterEvent),
    /// Event [WlDataDeviceLeaveEvent].
    WlDataDeviceLeave(WlDataDeviceLeaveEvent),
    /// Event [WlDataDeviceMotionEvent].
    WlDataDeviceMotion(WlDataDeviceMotionEvent),
    /// Event [WlDataDeviceDropEvent].
    WlDataDeviceDrop(WlDataDeviceDropEvent),
    /// Event [WlDataDeviceSelectionEvent].
    WlDataDeviceSelection(WlDataDeviceSelectionEvent),
    /// Event [WlShellSurfacePingEvent].
    WlShellSurfacePing(WlShellSurfacePingEvent),
    /// Event [WlShellSurfaceConfigureEvent].
    WlShellSurfaceConfigure(WlShellSurfaceConfigureEvent),
    /// Event [WlShellSurfacePopupDoneEvent].
    WlShellSurfacePopupDone(WlShellSurfacePopupDoneEvent),
    /// Event [WlSurfaceEnterEvent].
    WlSurfaceEnter(WlSurfaceEnterEvent),
    /// Event [WlSurfaceLeaveEvent].
    WlSurfaceLeave(WlSurfaceLeaveEvent),
    /// Event [WlSurfacePreferredBufferScaleEvent].
    WlSurfacePreferredBufferScale(WlSurfacePreferredBufferScaleEvent),
    /// Event [WlSurfacePreferredBufferTransformEvent].
    WlSurfacePreferredBufferTransform(WlSurfacePreferredBufferTransformEvent),
    /// Event [WlSeatCapabilitiesEvent].
    WlSeatCapabilities(WlSeatCapabilitiesEvent),
    /// Event [WlSeatNameEvent].
    WlSeatName(WlSeatNameEvent),
    /// Event [WlPointerEnterEvent].
    WlPointerEnter(WlPointerEnterEvent),
    /// Event [WlPointerLeaveEvent].
    WlPointerLeave(WlPointerLeaveEvent),
    /// Event [WlPointerMotionEvent].
    WlPointerMotion(WlPointerMotionEvent),
    /// Event [WlPointerButtonEvent].
    WlPointerButton(WlPointerButtonEvent),
    /// Event [WlPointerAxisEvent].
    WlPointerAxis(WlPointerAxisEvent),
    /// Event [WlPointerFrameEvent].
    WlPointerFrame(WlPointerFrameEvent),
    /// Event [WlPointerAxisSourceEvent].
    WlPointerAxisSource(WlPointerAxisSourceEvent),
    /// Event [WlPointerAxisStopEvent].
    WlPointerAxisStop(WlPointerAxisStopEvent),
    /// Event [WlPointerAxisDiscreteEvent].
    WlPointerAxisDiscrete(WlPointerAxisDiscreteEvent),
    /// Event [WlPointerAxisValue120Event].
    WlPointerAxisValue120(WlPointerAxisValue120Event),
    /// Event [WlPointerAxisRelativeDirectionEvent].
    WlPointerAxisRelativeDirection(WlPointerAxisRelativeDirectionEvent),
    /// Event [WlPointerWarpEvent].
    WlPointerWarp(WlPointerWarpEvent),
    /// Event [WlKeyboardKeymapEvent].
    WlKeyboardKeymap(WlKeyboardKeymapEvent),
    /// Event [WlKeyboardEnterEvent].
    WlKeyboardEnter(WlKeyboardEnterEvent),
    /// Event [WlKeyboardLeaveEvent].
    WlKeyboardLeave(WlKeyboardLeaveEvent),
    /// Event [WlKeyboardKeyEvent].
    WlKeyboardKey(WlKeyboardKeyEvent),
    /// Event [WlKeyboardModifiersEvent].
    WlKeyboardModifiers(WlKeyboardModifiersEvent),
    /// Event [WlKeyboardRepeatInfoEvent].
    WlKeyboardRepeatInfo(WlKeyboardRepeatInfoEvent),
    /// Event [WlTouchDownEvent].
    WlTouchDown(WlTouchDownEvent),
    /// Event [WlTouchUpEvent].
    WlTouchUp(WlTouchUpEvent),
    /// Event [WlTouchMotionEvent].
    WlTouchMotion(WlTouchMotionEvent),
    /// Event [WlTouchFrameEvent].
    WlTouchFrame(WlTouchFrameEvent),
    /// Event [WlTouchCancelEvent].
    WlTouchCancel(WlTouchCancelEvent),
    /// Event [WlTouchShapeEvent].
    WlTouchShape(WlTouchShapeEvent),
    /// Event [WlTouchOrientationEvent].
    WlTouchOrientation(WlTouchOrientationEvent),
    /// Event [WlOutputGeometryEvent].
    WlOutputGeometry(WlOutputGeometryEvent),
    /// Event [WlOutputModeEvent].
    WlOutputMode(WlOutputModeEvent),
    /// Event [WlOutputDoneEvent].
    WlOutputDone(WlOutputDoneEvent),
    /// Event [WlOutputScaleEvent].
    WlOutputScale(WlOutputScaleEvent),
    /// Event [WlOutputNameEvent].
    WlOutputName(WlOutputNameEvent),
    /// Event [WlOutputDescriptionEvent].
    WlOutputDescription(WlOutputDescriptionEvent),
}
impl CommandWriter for CommandEvent {
    fn object_id(&self) -> u32 {
        match self {
            Self::WlDisplayError(event) => event.object_id(),
            Self::WlDisplayDeleteId(event) => event.object_id(),
            Self::WlRegistryGlobal(event) => event.object_id(),
            Self::WlRegistryGlobalRemove(event) => event.object_id(),
            Self::WlCallbackDone(event) => event.object_id(),
            Self::WlShmFormat(event) => event.object_id(),
            Self::WlBufferRelease(event) => event.object_id(),
            Self::WlDataOfferOffer(event) => event.object_id(),
            Self::WlDataOfferSourceActions(event) => event.object_id(),
            Self::WlDataOfferAction(event) => event.object_id(),
            Self::WlDataSourceTarget(event) => event.object_id(),
            Self::WlDataSourceSend(event) => event.object_id(),
            Self::WlDataSourceCancelled(event) => event.object_id(),
            Self::WlDataSourceDndDropPerformed(event) => event.object_id(),
            Self::WlDataSourceDndFinished(event) => event.object_id(),
            Self::WlDataSourceAction(event) => event.object_id(),
            Self::WlDataDeviceDataOffer(event) => event.object_id(),
            Self::WlDataDeviceEnter(event) => event.object_id(),
            Self::WlDataDeviceLeave(event) => event.object_id(),
            Self::WlDataDeviceMotion(event) => event.object_id(),
            Self::WlDataDeviceDrop(event) => event.object_id(),
            Self::WlDataDeviceSelection(event) => event.object_id(),
            Self::WlShellSurfacePing(event) => event.object_id(),
            Self::WlShellSurfaceConfigure(event) => event.object_id(),
            Self::WlShellSurfacePopupDone(event) => event.object_id(),
            Self::WlSurfaceEnter(event) => event.object_id(),
            Self::WlSurfaceLeave(event) => event.object_id(),
            Self::WlSurfacePreferredBufferScale(event) => event.object_id(),
            Self::WlSurfacePreferredBufferTransform(event) => event.object_id(),
            Self::WlSeatCapabilities(event) => event.object_id(),
            Self::WlSeatName(event) => event.object_id(),
            Self::WlPointerEnter(event) => event.object_id(),
            Self::WlPointerLeave(event) => event.object_id(),
            Self::WlPointerMotion(event) => event.object_id(),
            Self::WlPointerButton(event) => event.object_id(),
            Self::WlPointerAxis(event) => event.object_id(),
            Self::WlPointerFrame(event) => event.object_id(),
            Self::WlPointerAxisSource(event) => event.object_id(),
            Self::WlPointerAxisStop(event) => event.object_id(),
            Self::WlPointerAxisDiscrete(event) => event.object_id(),
            Self::WlPointerAxisValue120(event) => event.object_id(),
            Self::WlPointerAxisRelativeDirection(event) => event.object_id(),
            Self::WlPointerWarp(event) => event.object_id(),
            Self::WlKeyboardKeymap(event) => event.object_id(),
            Self::WlKeyboardEnter(event) => event.object_id(),
            Self::WlKeyboardLeave(event) => event.object_id(),
            Self::WlKeyboardKey(event) => event.object_id(),
            Self::WlKeyboardModifiers(event) => event.object_id(),
            Self::WlKeyboardRepeatInfo(event) => event.object_id(),
            Self::WlTouchDown(event) => event.object_id(),
            Self::WlTouchUp(event) => event.object_id(),
            Self::WlTouchMotion(event) => event.object_id(),
            Self::WlTouchFrame(event) => event.object_id(),
            Self::WlTouchCancel(event) => event.object_id(),
            Self::WlTouchShape(event) => event.object_id(),
            Self::WlTouchOrientation(event) => event.object_id(),
            Self::WlOutputGeometry(event) => event.object_id(),
            Self::WlOutputMode(event) => event.object_id(),
            Self::WlOutputDone(event) => event.object_id(),
            Self::WlOutputScale(event) => event.object_id(),
            Self::WlOutputName(event) => event.object_id(),
            Self::WlOutputDescription(event) => event.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WlDisplayError(event) => event.opcode(),
            Self::WlDisplayDeleteId(event) => event.opcode(),
            Self::WlRegistryGlobal(event) => event.opcode(),
            Self::WlRegistryGlobalRemove(event) => event.opcode(),
            Self::WlCallbackDone(event) => event.opcode(),
            Self::WlShmFormat(event) => event.opcode(),
            Self::WlBufferRelease(event) => event.opcode(),
            Self::WlDataOfferOffer(event) => event.opcode(),
            Self::WlDataOfferSourceActions(event) => event.opcode(),
            Self::WlDataOfferAction(event) => event.opcode(),
            Self::WlDataSourceTarget(event) => event.opcode(),
            Self::WlDataSourceSend(event) => event.opcode(),
            Self::WlDataSourceCancelled(event) => event.opcode(),
            Self::WlDataSourceDndDropPerformed(event) => event.opcode(),
            Self::WlDataSourceDndFinished(event) => event.opcode(),
            Self::WlDataSourceAction(event) => event.opcode(),
            Self::WlDataDeviceDataOffer(event) => event.opcode(),
            Self::WlDataDeviceEnter(event) => event.opcode(),
            Self::WlDataDeviceLeave(event) => event.opcode(),
            Self::WlDataDeviceMotion(event) => event.opcode(),
            Self::WlDataDeviceDrop(event) => event.opcode(),
            Self::WlDataDeviceSelection(event) => event.opcode(),
            Self::WlShellSurfacePing(event) => event.opcode(),
            Self::WlShellSurfaceConfigure(event) => event.opcode(),
            Self::WlShellSurfacePopupDone(event) => event.opcode(),
            Self::WlSurfaceEnter(event) => event.opcode(),
            Self::WlSurfaceLeave(event) => event.opcode(),
            Self::WlSurfacePreferredBufferScale(event) => event.opcode(),
            Self::WlSurfacePreferredBufferTransform(event) => event.opcode(),
            Self::WlSeatCapabilities(event) => event.opcode(),
            Self::WlSeatName(event) => event.opcode(),
            Self::WlPointerEnter(event) => event.opcode(),
            Self::WlPointerLeave(event) => event.opcode(),
            Self::WlPointerMotion(event) => event.opcode(),
            Self::WlPointerButton(event) => event.opcode(),
            Self::WlPointerAxis(event) => event.opcode(),
            Self::WlPointerFrame(event) => event.opcode(),
            Self::WlPointerAxisSource(event) => event.opcode(),
            Self::WlPointerAxisStop(event) => event.opcode(),
            Self::WlPointerAxisDiscrete(event) => event.opcode(),
            Self::WlPointerAxisValue120(event) => event.opcode(),
            Self::WlPointerAxisRelativeDirection(event) => event.opcode(),
            Self::WlPointerWarp(event) => event.opcode(),
            Self::WlKeyboardKeymap(event) => event.opcode(),
            Self::WlKeyboardEnter(event) => event.opcode(),
            Self::WlKeyboardLeave(event) => event.opcode(),
            Self::WlKeyboardKey(event) => event.opcode(),
            Self::WlKeyboardModifiers(event) => event.opcode(),
            Self::WlKeyboardRepeatInfo(event) => event.opcode(),
            Self::WlTouchDown(event) => event.opcode(),
            Self::WlTouchUp(event) => event.opcode(),
            Self::WlTouchMotion(event) => event.opcode(),
            Self::WlTouchFrame(event) => event.opcode(),
            Self::WlTouchCancel(event) => event.opcode(),
            Self::WlTouchShape(event) => event.opcode(),
            Self::WlTouchOrientation(event) => event.opcode(),
            Self::WlOutputGeometry(event) => event.opcode(),
            Self::WlOutputMode(event) => event.opcode(),
            Self::WlOutputDone(event) => event.opcode(),
            Self::WlOutputScale(event) => event.opcode(),
            Self::WlOutputName(event) => event.opcode(),
            Self::WlOutputDescription(event) => event.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WlDisplayError(event) => event.payload_size(),
            Self::WlDisplayDeleteId(event) => event.payload_size(),
            Self::WlRegistryGlobal(event) => event.payload_size(),
            Self::WlRegistryGlobalRemove(event) => event.payload_size(),
            Self::WlCallbackDone(event) => event.payload_size(),
            Self::WlShmFormat(event) => event.payload_size(),
            Self::WlBufferRelease(event) => event.payload_size(),
            Self::WlDataOfferOffer(event) => event.payload_size(),
            Self::WlDataOfferSourceActions(event) => event.payload_size(),
            Self::WlDataOfferAction(event) => event.payload_size(),
            Self::WlDataSourceTarget(event) => event.payload_size(),
            Self::WlDataSourceSend(event) => event.payload_size(),
            Self::WlDataSourceCancelled(event) => event.payload_size(),
            Self::WlDataSourceDndDropPerformed(event) => event.payload_size(),
            Self::WlDataSourceDndFinished(event) => event.payload_size(),
            Self::WlDataSourceAction(event) => event.payload_size(),
            Self::WlDataDeviceDataOffer(event) => event.payload_size(),
            Self::WlDataDeviceEnter(event) => event.payload_size(),
            Self::WlDataDeviceLeave(event) => event.payload_size(),
            Self::WlDataDeviceMotion(event) => event.payload_size(),
            Self::WlDataDeviceDrop(event) => event.payload_size(),
            Self::WlDataDeviceSelection(event) => event.payload_size(),
            Self::WlShellSurfacePing(event) => event.payload_size(),
            Self::WlShellSurfaceConfigure(event) => event.payload_size(),
            Self::WlShellSurfacePopupDone(event) => event.payload_size(),
            Self::WlSurfaceEnter(event) => event.payload_size(),
            Self::WlSurfaceLeave(event) => event.payload_size(),
            Self::WlSurfacePreferredBufferScale(event) => event.payload_size(),
            Self::WlSurfacePreferredBufferTransform(event) => event.payload_size(),
            Self::WlSeatCapabilities(event) => event.payload_size(),
            Self::WlSeatName(event) => event.payload_size(),
            Self::WlPointerEnter(event) => event.payload_size(),
            Self::WlPointerLeave(event) => event.payload_size(),
            Self::WlPointerMotion(event) => event.payload_size(),
            Self::WlPointerButton(event) => event.payload_size(),
            Self::WlPointerAxis(event) => event.payload_size(),
            Self::WlPointerFrame(event) => event.payload_size(),
            Self::WlPointerAxisSource(event) => event.payload_size(),
            Self::WlPointerAxisStop(event) => event.payload_size(),
            Self::WlPointerAxisDiscrete(event) => event.payload_size(),
            Self::WlPointerAxisValue120(event) => event.payload_size(),
            Self::WlPointerAxisRelativeDirection(event) => event.payload_size(),
            Self::WlPointerWarp(event) => event.payload_size(),
            Self::WlKeyboardKeymap(event) => event.payload_size(),
            Self::WlKeyboardEnter(event) => event.payload_size(),
            Self::WlKeyboardLeave(event) => event.payload_size(),
            Self::WlKeyboardKey(event) => event.payload_size(),
            Self::WlKeyboardModifiers(event) => event.payload_size(),
            Self::WlKeyboardRepeatInfo(event) => event.payload_size(),
            Self::WlTouchDown(event) => event.payload_size(),
            Self::WlTouchUp(event) => event.payload_size(),
            Self::WlTouchMotion(event) => event.payload_size(),
            Self::WlTouchFrame(event) => event.payload_size(),
            Self::WlTouchCancel(event) => event.payload_size(),
            Self::WlTouchShape(event) => event.payload_size(),
            Self::WlTouchOrientation(event) => event.payload_size(),
            Self::WlOutputGeometry(event) => event.payload_size(),
            Self::WlOutputMode(event) => event.payload_size(),
            Self::WlOutputDone(event) => event.payload_size(),
            Self::WlOutputScale(event) => event.payload_size(),
            Self::WlOutputName(event) => event.payload_size(),
            Self::WlOutputDescription(event) => event.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::WlDisplayError(event) => event.write(writer, state).await,
            Self::WlDisplayDeleteId(event) => event.write(writer, state).await,
            Self::WlRegistryGlobal(event) => event.write(writer, state).await,
            Self::WlRegistryGlobalRemove(event) => event.write(writer, state).await,
            Self::WlCallbackDone(event) => event.write(writer, state).await,
            Self::WlShmFormat(event) => event.write(writer, state).await,
            Self::WlBufferRelease(event) => event.write(writer, state).await,
            Self::WlDataOfferOffer(event) => event.write(writer, state).await,
            Self::WlDataOfferSourceActions(event) => event.write(writer, state).await,
            Self::WlDataOfferAction(event) => event.write(writer, state).await,
            Self::WlDataSourceTarget(event) => event.write(writer, state).await,
            Self::WlDataSourceSend(event) => event.write(writer, state).await,
            Self::WlDataSourceCancelled(event) => event.write(writer, state).await,
            Self::WlDataSourceDndDropPerformed(event) => event.write(writer, state).await,
            Self::WlDataSourceDndFinished(event) => event.write(writer, state).await,
            Self::WlDataSourceAction(event) => event.write(writer, state).await,
            Self::WlDataDeviceDataOffer(event) => event.write(writer, state).await,
            Self::WlDataDeviceEnter(event) => event.write(writer, state).await,
            Self::WlDataDeviceLeave(event) => event.write(writer, state).await,
            Self::WlDataDeviceMotion(event) => event.write(writer, state).await,
            Self::WlDataDeviceDrop(event) => event.write(writer, state).await,
            Self::WlDataDeviceSelection(event) => event.write(writer, state).await,
            Self::WlShellSurfacePing(event) => event.write(writer, state).await,
            Self::WlShellSurfaceConfigure(event) => event.write(writer, state).await,
            Self::WlShellSurfacePopupDone(event) => event.write(writer, state).await,
            Self::WlSurfaceEnter(event) => event.write(writer, state).await,
            Self::WlSurfaceLeave(event) => event.write(writer, state).await,
            Self::WlSurfacePreferredBufferScale(event) => event.write(writer, state).await,
            Self::WlSurfacePreferredBufferTransform(event) => event.write(writer, state).await,
            Self::WlSeatCapabilities(event) => event.write(writer, state).await,
            Self::WlSeatName(event) => event.write(writer, state).await,
            Self::WlPointerEnter(event) => event.write(writer, state).await,
            Self::WlPointerLeave(event) => event.write(writer, state).await,
            Self::WlPointerMotion(event) => event.write(writer, state).await,
            Self::WlPointerButton(event) => event.write(writer, state).await,
            Self::WlPointerAxis(event) => event.write(writer, state).await,
            Self::WlPointerFrame(event) => event.write(writer, state).await,
            Self::WlPointerAxisSource(event) => event.write(writer, state).await,
            Self::WlPointerAxisStop(event) => event.write(writer, state).await,
            Self::WlPointerAxisDiscrete(event) => event.write(writer, state).await,
            Self::WlPointerAxisValue120(event) => event.write(writer, state).await,
            Self::WlPointerAxisRelativeDirection(event) => event.write(writer, state).await,
            Self::WlPointerWarp(event) => event.write(writer, state).await,
            Self::WlKeyboardKeymap(event) => event.write(writer, state).await,
            Self::WlKeyboardEnter(event) => event.write(writer, state).await,
            Self::WlKeyboardLeave(event) => event.write(writer, state).await,
            Self::WlKeyboardKey(event) => event.write(writer, state).await,
            Self::WlKeyboardModifiers(event) => event.write(writer, state).await,
            Self::WlKeyboardRepeatInfo(event) => event.write(writer, state).await,
            Self::WlTouchDown(event) => event.write(writer, state).await,
            Self::WlTouchUp(event) => event.write(writer, state).await,
            Self::WlTouchMotion(event) => event.write(writer, state).await,
            Self::WlTouchFrame(event) => event.write(writer, state).await,
            Self::WlTouchCancel(event) => event.write(writer, state).await,
            Self::WlTouchShape(event) => event.write(writer, state).await,
            Self::WlTouchOrientation(event) => event.write(writer, state).await,
            Self::WlOutputGeometry(event) => event.write(writer, state).await,
            Self::WlOutputMode(event) => event.write(writer, state).await,
            Self::WlOutputDone(event) => event.write(writer, state).await,
            Self::WlOutputScale(event) => event.write(writer, state).await,
            Self::WlOutputName(event) => event.write(writer, state).await,
            Self::WlOutputDescription(event) => event.write(writer, state).await,
        }
    }
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
            ObjectInteface::Core(CoreObjectInteface::WlBuffer { .. }) => match opcode {
                WlBufferReleaseEvent::OPCODE => Ok(Self::WlBufferRelease(
                    WlBufferReleaseEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Core(CoreObjectInteface::WlCallback { .. }) => match opcode {
                WlCallbackDoneEvent::OPCODE => Ok(Self::WlCallbackDone(
                    WlCallbackDoneEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Core(CoreObjectInteface::WlDataDevice { .. }) => match opcode {
                WlDataDeviceDataOfferEvent::OPCODE => Ok(Self::WlDataDeviceDataOffer(
                    WlDataDeviceDataOfferEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlDataDeviceDropEvent::OPCODE => Ok(Self::WlDataDeviceDrop(
                    WlDataDeviceDropEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlDataDeviceEnterEvent::OPCODE => Ok(Self::WlDataDeviceEnter(
                    WlDataDeviceEnterEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlDataDeviceLeaveEvent::OPCODE => Ok(Self::WlDataDeviceLeave(
                    WlDataDeviceLeaveEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlDataDeviceMotionEvent::OPCODE => Ok(Self::WlDataDeviceMotion(
                    WlDataDeviceMotionEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlDataDeviceSelectionEvent::OPCODE => Ok(Self::WlDataDeviceSelection(
                    WlDataDeviceSelectionEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Core(CoreObjectInteface::WlDataOffer { .. }) => match opcode {
                WlDataOfferActionEvent::OPCODE => Ok(Self::WlDataOfferAction(
                    WlDataOfferActionEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlDataOfferOfferEvent::OPCODE => Ok(Self::WlDataOfferOffer(
                    WlDataOfferOfferEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlDataOfferSourceActionsEvent::OPCODE => Ok(Self::WlDataOfferSourceActions(
                    WlDataOfferSourceActionsEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Core(CoreObjectInteface::WlDataSource { .. }) => match opcode {
                WlDataSourceActionEvent::OPCODE => Ok(Self::WlDataSourceAction(
                    WlDataSourceActionEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlDataSourceCancelledEvent::OPCODE => Ok(Self::WlDataSourceCancelled(
                    WlDataSourceCancelledEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlDataSourceDndDropPerformedEvent::OPCODE => {
                    Ok(Self::WlDataSourceDndDropPerformed(
                        WlDataSourceDndDropPerformedEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WlDataSourceDndFinishedEvent::OPCODE => Ok(Self::WlDataSourceDndFinished(
                    WlDataSourceDndFinishedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlDataSourceSendEvent::OPCODE => Ok(Self::WlDataSourceSend(
                    WlDataSourceSendEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlDataSourceTargetEvent::OPCODE => Ok(Self::WlDataSourceTarget(
                    WlDataSourceTargetEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Core(CoreObjectInteface::WlDisplay { .. }) => match opcode {
                WlDisplayDeleteIdEvent::OPCODE => Ok(Self::WlDisplayDeleteId(
                    WlDisplayDeleteIdEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlDisplayErrorEvent::OPCODE => Ok(Self::WlDisplayError(
                    WlDisplayErrorEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Core(CoreObjectInteface::WlKeyboard { .. }) => match opcode {
                WlKeyboardEnterEvent::OPCODE => Ok(Self::WlKeyboardEnter(
                    WlKeyboardEnterEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlKeyboardKeyEvent::OPCODE => Ok(Self::WlKeyboardKey(
                    WlKeyboardKeyEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlKeyboardKeymapEvent::OPCODE => Ok(Self::WlKeyboardKeymap(
                    WlKeyboardKeymapEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlKeyboardLeaveEvent::OPCODE => Ok(Self::WlKeyboardLeave(
                    WlKeyboardLeaveEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlKeyboardModifiersEvent::OPCODE => Ok(Self::WlKeyboardModifiers(
                    WlKeyboardModifiersEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlKeyboardRepeatInfoEvent::OPCODE => Ok(Self::WlKeyboardRepeatInfo(
                    WlKeyboardRepeatInfoEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Core(CoreObjectInteface::WlOutput { .. }) => match opcode {
                WlOutputDescriptionEvent::OPCODE => Ok(Self::WlOutputDescription(
                    WlOutputDescriptionEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlOutputDoneEvent::OPCODE => Ok(Self::WlOutputDone(
                    WlOutputDoneEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlOutputGeometryEvent::OPCODE => Ok(Self::WlOutputGeometry(
                    WlOutputGeometryEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlOutputModeEvent::OPCODE => Ok(Self::WlOutputMode(
                    WlOutputModeEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlOutputNameEvent::OPCODE => Ok(Self::WlOutputName(
                    WlOutputNameEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlOutputScaleEvent::OPCODE => Ok(Self::WlOutputScale(
                    WlOutputScaleEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Core(CoreObjectInteface::WlPointer { .. }) => match opcode {
                WlPointerAxisDiscreteEvent::OPCODE => Ok(Self::WlPointerAxisDiscrete(
                    WlPointerAxisDiscreteEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlPointerAxisEvent::OPCODE => Ok(Self::WlPointerAxis(
                    WlPointerAxisEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlPointerAxisRelativeDirectionEvent::OPCODE => {
                    Ok(Self::WlPointerAxisRelativeDirection(
                        WlPointerAxisRelativeDirectionEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WlPointerAxisSourceEvent::OPCODE => Ok(Self::WlPointerAxisSource(
                    WlPointerAxisSourceEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlPointerAxisStopEvent::OPCODE => Ok(Self::WlPointerAxisStop(
                    WlPointerAxisStopEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlPointerAxisValue120Event::OPCODE => Ok(Self::WlPointerAxisValue120(
                    WlPointerAxisValue120Event::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlPointerButtonEvent::OPCODE => Ok(Self::WlPointerButton(
                    WlPointerButtonEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlPointerEnterEvent::OPCODE => Ok(Self::WlPointerEnter(
                    WlPointerEnterEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlPointerFrameEvent::OPCODE => Ok(Self::WlPointerFrame(
                    WlPointerFrameEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlPointerLeaveEvent::OPCODE => Ok(Self::WlPointerLeave(
                    WlPointerLeaveEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlPointerMotionEvent::OPCODE => Ok(Self::WlPointerMotion(
                    WlPointerMotionEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlPointerWarpEvent::OPCODE => Ok(Self::WlPointerWarp(
                    WlPointerWarpEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Core(CoreObjectInteface::WlRegistry { .. }) => match opcode {
                WlRegistryGlobalEvent::OPCODE => Ok(Self::WlRegistryGlobal(
                    WlRegistryGlobalEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlRegistryGlobalRemoveEvent::OPCODE => Ok(Self::WlRegistryGlobalRemove(
                    WlRegistryGlobalRemoveEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Core(CoreObjectInteface::WlSeat { .. }) => match opcode {
                WlSeatCapabilitiesEvent::OPCODE => Ok(Self::WlSeatCapabilities(
                    WlSeatCapabilitiesEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlSeatNameEvent::OPCODE => Ok(Self::WlSeatName(
                    WlSeatNameEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Core(CoreObjectInteface::WlShellSurface { .. }) => match opcode {
                WlShellSurfaceConfigureEvent::OPCODE => Ok(Self::WlShellSurfaceConfigure(
                    WlShellSurfaceConfigureEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlShellSurfacePingEvent::OPCODE => Ok(Self::WlShellSurfacePing(
                    WlShellSurfacePingEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlShellSurfacePopupDoneEvent::OPCODE => Ok(Self::WlShellSurfacePopupDone(
                    WlShellSurfacePopupDoneEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Core(CoreObjectInteface::WlShm { .. }) => match opcode {
                WlShmFormatEvent::OPCODE => Ok(Self::WlShmFormat(
                    WlShmFormatEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Core(CoreObjectInteface::WlSurface { .. }) => match opcode {
                WlSurfaceEnterEvent::OPCODE => Ok(Self::WlSurfaceEnter(
                    WlSurfaceEnterEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlSurfaceLeaveEvent::OPCODE => Ok(Self::WlSurfaceLeave(
                    WlSurfaceLeaveEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlSurfacePreferredBufferScaleEvent::OPCODE => {
                    Ok(Self::WlSurfacePreferredBufferScale(
                        WlSurfacePreferredBufferScaleEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WlSurfacePreferredBufferTransformEvent::OPCODE => {
                    Ok(Self::WlSurfacePreferredBufferTransform(
                        WlSurfacePreferredBufferTransformEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Core(CoreObjectInteface::WlTouch { .. }) => match opcode {
                WlTouchCancelEvent::OPCODE => Ok(Self::WlTouchCancel(
                    WlTouchCancelEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlTouchDownEvent::OPCODE => Ok(Self::WlTouchDown(
                    WlTouchDownEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlTouchFrameEvent::OPCODE => Ok(Self::WlTouchFrame(
                    WlTouchFrameEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlTouchMotionEvent::OPCODE => Ok(Self::WlTouchMotion(
                    WlTouchMotionEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlTouchOrientationEvent::OPCODE => Ok(Self::WlTouchOrientation(
                    WlTouchOrientationEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlTouchShapeEvent::OPCODE => Ok(Self::WlTouchShape(
                    WlTouchShapeEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WlTouchUpEvent::OPCODE => Ok(Self::WlTouchUp(
                    WlTouchUpEvent::read_with_interface(
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
