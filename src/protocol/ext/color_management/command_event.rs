use super::{
    WpColorManagementOutputV1ImageDescriptionChangedEvent,
    WpColorManagementSurfaceFeedbackV1PreferredChanged2Event,
    WpColorManagementSurfaceFeedbackV1PreferredChangedEvent, WpColorManagerV1DoneEvent,
    WpColorManagerV1SupportedFeatureEvent, WpColorManagerV1SupportedIntentEvent,
    WpColorManagerV1SupportedPrimariesNamedEvent, WpColorManagerV1SupportedTfNamedEvent,
    WpImageDescriptionInfoV1DoneEvent, WpImageDescriptionInfoV1IccFileEvent,
    WpImageDescriptionInfoV1LuminancesEvent, WpImageDescriptionInfoV1PrimariesEvent,
    WpImageDescriptionInfoV1PrimariesNamedEvent, WpImageDescriptionInfoV1TargetLuminanceEvent,
    WpImageDescriptionInfoV1TargetMaxCllEvent, WpImageDescriptionInfoV1TargetMaxFallEvent,
    WpImageDescriptionInfoV1TargetPrimariesEvent, WpImageDescriptionInfoV1TfNamedEvent,
    WpImageDescriptionInfoV1TfPowerEvent, WpImageDescriptionV1FailedEvent,
    WpImageDescriptionV1Ready2Event, WpImageDescriptionV1ReadyEvent,
};
use crate::protocol::ext::color_management::ObjectInteface as ColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Color management events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [WpColorManagerV1SupportedIntentEvent].
    WpColorManagerV1SupportedIntent(WpColorManagerV1SupportedIntentEvent),
    /// Message [WpColorManagerV1SupportedFeatureEvent].
    WpColorManagerV1SupportedFeature(WpColorManagerV1SupportedFeatureEvent),
    /// Message [WpColorManagerV1SupportedTfNamedEvent].
    WpColorManagerV1SupportedTfNamed(WpColorManagerV1SupportedTfNamedEvent),
    /// Message [WpColorManagerV1SupportedPrimariesNamedEvent].
    WpColorManagerV1SupportedPrimariesNamed(WpColorManagerV1SupportedPrimariesNamedEvent),
    /// Message [WpColorManagerV1DoneEvent].
    WpColorManagerV1Done(WpColorManagerV1DoneEvent),
    /// Message [WpColorManagementOutputV1ImageDescriptionChangedEvent].
    WpColorManagementOutputV1ImageDescriptionChanged(
        WpColorManagementOutputV1ImageDescriptionChangedEvent,
    ),
    /// Message [WpColorManagementSurfaceFeedbackV1PreferredChangedEvent].
    WpColorManagementSurfaceFeedbackV1PreferredChanged(
        WpColorManagementSurfaceFeedbackV1PreferredChangedEvent,
    ),
    /// Message [WpColorManagementSurfaceFeedbackV1PreferredChanged2Event].
    WpColorManagementSurfaceFeedbackV1PreferredChanged2(
        WpColorManagementSurfaceFeedbackV1PreferredChanged2Event,
    ),
    /// Message [WpImageDescriptionV1FailedEvent].
    WpImageDescriptionV1Failed(WpImageDescriptionV1FailedEvent),
    /// Message [WpImageDescriptionV1ReadyEvent].
    WpImageDescriptionV1Ready(WpImageDescriptionV1ReadyEvent),
    /// Message [WpImageDescriptionV1Ready2Event].
    WpImageDescriptionV1Ready2(WpImageDescriptionV1Ready2Event),
    /// Message [WpImageDescriptionInfoV1DoneEvent].
    WpImageDescriptionInfoV1Done(WpImageDescriptionInfoV1DoneEvent),
    /// Message [WpImageDescriptionInfoV1IccFileEvent].
    WpImageDescriptionInfoV1IccFile(WpImageDescriptionInfoV1IccFileEvent),
    /// Message [WpImageDescriptionInfoV1PrimariesEvent].
    WpImageDescriptionInfoV1Primaries(WpImageDescriptionInfoV1PrimariesEvent),
    /// Message [WpImageDescriptionInfoV1PrimariesNamedEvent].
    WpImageDescriptionInfoV1PrimariesNamed(WpImageDescriptionInfoV1PrimariesNamedEvent),
    /// Message [WpImageDescriptionInfoV1TfPowerEvent].
    WpImageDescriptionInfoV1TfPower(WpImageDescriptionInfoV1TfPowerEvent),
    /// Message [WpImageDescriptionInfoV1TfNamedEvent].
    WpImageDescriptionInfoV1TfNamed(WpImageDescriptionInfoV1TfNamedEvent),
    /// Message [WpImageDescriptionInfoV1LuminancesEvent].
    WpImageDescriptionInfoV1Luminances(WpImageDescriptionInfoV1LuminancesEvent),
    /// Message [WpImageDescriptionInfoV1TargetPrimariesEvent].
    WpImageDescriptionInfoV1TargetPrimaries(WpImageDescriptionInfoV1TargetPrimariesEvent),
    /// Message [WpImageDescriptionInfoV1TargetLuminanceEvent].
    WpImageDescriptionInfoV1TargetLuminance(WpImageDescriptionInfoV1TargetLuminanceEvent),
    /// Message [WpImageDescriptionInfoV1TargetMaxCllEvent].
    WpImageDescriptionInfoV1TargetMaxCll(WpImageDescriptionInfoV1TargetMaxCllEvent),
    /// Message [WpImageDescriptionInfoV1TargetMaxFallEvent].
    WpImageDescriptionInfoV1TargetMaxFall(WpImageDescriptionInfoV1TargetMaxFallEvent),
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
ObjectInteface::ColorManagement(ColorManagementObjectInteface::WpColorManagerV1{..})=>match opcode {
WpColorManagerV1SupportedIntentEvent::OPCODE=>Ok(Self::WpColorManagerV1SupportedIntent(WpColorManagerV1SupportedIntentEvent::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorManagerV1SupportedFeatureEvent::OPCODE=>Ok(Self::WpColorManagerV1SupportedFeature(WpColorManagerV1SupportedFeatureEvent::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorManagerV1SupportedTfNamedEvent::OPCODE=>Ok(Self::WpColorManagerV1SupportedTfNamed(WpColorManagerV1SupportedTfNamedEvent::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorManagerV1SupportedPrimariesNamedEvent::OPCODE=>Ok(Self::WpColorManagerV1SupportedPrimariesNamed(WpColorManagerV1SupportedPrimariesNamedEvent::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorManagerV1DoneEvent::OPCODE=>Ok(Self::WpColorManagerV1Done(WpColorManagerV1DoneEvent::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::ColorManagement(ColorManagementObjectInteface::WpColorManagementOutputV1{..})=>match opcode {
WpColorManagementOutputV1ImageDescriptionChangedEvent::OPCODE=>Ok(Self::WpColorManagementOutputV1ImageDescriptionChanged(WpColorManagementOutputV1ImageDescriptionChangedEvent::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::ColorManagement(ColorManagementObjectInteface::WpColorManagementSurfaceFeedbackV1{..})=>match opcode {
WpColorManagementSurfaceFeedbackV1PreferredChangedEvent::OPCODE=>Ok(Self::WpColorManagementSurfaceFeedbackV1PreferredChanged(WpColorManagementSurfaceFeedbackV1PreferredChangedEvent::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorManagementSurfaceFeedbackV1PreferredChanged2Event::OPCODE=>Ok(Self::WpColorManagementSurfaceFeedbackV1PreferredChanged2(WpColorManagementSurfaceFeedbackV1PreferredChanged2Event::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::ColorManagement(ColorManagementObjectInteface::WpImageDescriptionV1{..})=>match opcode {
WpImageDescriptionV1FailedEvent::OPCODE=>Ok(Self::WpImageDescriptionV1Failed(WpImageDescriptionV1FailedEvent::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionV1ReadyEvent::OPCODE=>Ok(Self::WpImageDescriptionV1Ready(WpImageDescriptionV1ReadyEvent::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionV1Ready2Event::OPCODE=>Ok(Self::WpImageDescriptionV1Ready2(WpImageDescriptionV1Ready2Event::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::ColorManagement(ColorManagementObjectInteface::WpImageDescriptionInfoV1{..})=>match opcode {
WpImageDescriptionInfoV1DoneEvent::OPCODE=>Ok(Self::WpImageDescriptionInfoV1Done(WpImageDescriptionInfoV1DoneEvent::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionInfoV1IccFileEvent::OPCODE=>Ok(Self::WpImageDescriptionInfoV1IccFile(WpImageDescriptionInfoV1IccFileEvent::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionInfoV1PrimariesEvent::OPCODE=>Ok(Self::WpImageDescriptionInfoV1Primaries(WpImageDescriptionInfoV1PrimariesEvent::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionInfoV1PrimariesNamedEvent::OPCODE=>Ok(Self::WpImageDescriptionInfoV1PrimariesNamed(WpImageDescriptionInfoV1PrimariesNamedEvent::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionInfoV1TfPowerEvent::OPCODE=>Ok(Self::WpImageDescriptionInfoV1TfPower(WpImageDescriptionInfoV1TfPowerEvent::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionInfoV1TfNamedEvent::OPCODE=>Ok(Self::WpImageDescriptionInfoV1TfNamed(WpImageDescriptionInfoV1TfNamedEvent::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionInfoV1LuminancesEvent::OPCODE=>Ok(Self::WpImageDescriptionInfoV1Luminances(WpImageDescriptionInfoV1LuminancesEvent::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionInfoV1TargetPrimariesEvent::OPCODE=>Ok(Self::WpImageDescriptionInfoV1TargetPrimaries(WpImageDescriptionInfoV1TargetPrimariesEvent::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionInfoV1TargetLuminanceEvent::OPCODE=>Ok(Self::WpImageDescriptionInfoV1TargetLuminance(WpImageDescriptionInfoV1TargetLuminanceEvent::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionInfoV1TargetMaxCllEvent::OPCODE=>Ok(Self::WpImageDescriptionInfoV1TargetMaxCll(WpImageDescriptionInfoV1TargetMaxCllEvent::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionInfoV1TargetMaxFallEvent::OPCODE=>Ok(Self::WpImageDescriptionInfoV1TargetMaxFall(WpImageDescriptionInfoV1TargetMaxFallEvent::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
_=>Err(FdReaderError::InvalidProtocol),}
    }
}
impl CommandWriter for CommandEvent {
    fn object_id(&self) -> u32 {
        match self {
            Self::WpColorManagerV1SupportedIntent(message) => message.object_id(),
            Self::WpColorManagerV1SupportedFeature(message) => message.object_id(),
            Self::WpColorManagerV1SupportedTfNamed(message) => message.object_id(),
            Self::WpColorManagerV1SupportedPrimariesNamed(message) => message.object_id(),
            Self::WpColorManagerV1Done(message) => message.object_id(),
            Self::WpColorManagementOutputV1ImageDescriptionChanged(message) => message.object_id(),
            Self::WpColorManagementSurfaceFeedbackV1PreferredChanged(message) => {
                message.object_id()
            }
            Self::WpColorManagementSurfaceFeedbackV1PreferredChanged2(message) => {
                message.object_id()
            }
            Self::WpImageDescriptionV1Failed(message) => message.object_id(),
            Self::WpImageDescriptionV1Ready(message) => message.object_id(),
            Self::WpImageDescriptionV1Ready2(message) => message.object_id(),
            Self::WpImageDescriptionInfoV1Done(message) => message.object_id(),
            Self::WpImageDescriptionInfoV1IccFile(message) => message.object_id(),
            Self::WpImageDescriptionInfoV1Primaries(message) => message.object_id(),
            Self::WpImageDescriptionInfoV1PrimariesNamed(message) => message.object_id(),
            Self::WpImageDescriptionInfoV1TfPower(message) => message.object_id(),
            Self::WpImageDescriptionInfoV1TfNamed(message) => message.object_id(),
            Self::WpImageDescriptionInfoV1Luminances(message) => message.object_id(),
            Self::WpImageDescriptionInfoV1TargetPrimaries(message) => message.object_id(),
            Self::WpImageDescriptionInfoV1TargetLuminance(message) => message.object_id(),
            Self::WpImageDescriptionInfoV1TargetMaxCll(message) => message.object_id(),
            Self::WpImageDescriptionInfoV1TargetMaxFall(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WpColorManagerV1SupportedIntent(message) => message.opcode(),
            Self::WpColorManagerV1SupportedFeature(message) => message.opcode(),
            Self::WpColorManagerV1SupportedTfNamed(message) => message.opcode(),
            Self::WpColorManagerV1SupportedPrimariesNamed(message) => message.opcode(),
            Self::WpColorManagerV1Done(message) => message.opcode(),
            Self::WpColorManagementOutputV1ImageDescriptionChanged(message) => message.opcode(),
            Self::WpColorManagementSurfaceFeedbackV1PreferredChanged(message) => message.opcode(),
            Self::WpColorManagementSurfaceFeedbackV1PreferredChanged2(message) => message.opcode(),
            Self::WpImageDescriptionV1Failed(message) => message.opcode(),
            Self::WpImageDescriptionV1Ready(message) => message.opcode(),
            Self::WpImageDescriptionV1Ready2(message) => message.opcode(),
            Self::WpImageDescriptionInfoV1Done(message) => message.opcode(),
            Self::WpImageDescriptionInfoV1IccFile(message) => message.opcode(),
            Self::WpImageDescriptionInfoV1Primaries(message) => message.opcode(),
            Self::WpImageDescriptionInfoV1PrimariesNamed(message) => message.opcode(),
            Self::WpImageDescriptionInfoV1TfPower(message) => message.opcode(),
            Self::WpImageDescriptionInfoV1TfNamed(message) => message.opcode(),
            Self::WpImageDescriptionInfoV1Luminances(message) => message.opcode(),
            Self::WpImageDescriptionInfoV1TargetPrimaries(message) => message.opcode(),
            Self::WpImageDescriptionInfoV1TargetLuminance(message) => message.opcode(),
            Self::WpImageDescriptionInfoV1TargetMaxCll(message) => message.opcode(),
            Self::WpImageDescriptionInfoV1TargetMaxFall(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WpColorManagerV1SupportedIntent(message) => message.payload_size(),
            Self::WpColorManagerV1SupportedFeature(message) => message.payload_size(),
            Self::WpColorManagerV1SupportedTfNamed(message) => message.payload_size(),
            Self::WpColorManagerV1SupportedPrimariesNamed(message) => message.payload_size(),
            Self::WpColorManagerV1Done(message) => message.payload_size(),
            Self::WpColorManagementOutputV1ImageDescriptionChanged(message) => {
                message.payload_size()
            }
            Self::WpColorManagementSurfaceFeedbackV1PreferredChanged(message) => {
                message.payload_size()
            }
            Self::WpColorManagementSurfaceFeedbackV1PreferredChanged2(message) => {
                message.payload_size()
            }
            Self::WpImageDescriptionV1Failed(message) => message.payload_size(),
            Self::WpImageDescriptionV1Ready(message) => message.payload_size(),
            Self::WpImageDescriptionV1Ready2(message) => message.payload_size(),
            Self::WpImageDescriptionInfoV1Done(message) => message.payload_size(),
            Self::WpImageDescriptionInfoV1IccFile(message) => message.payload_size(),
            Self::WpImageDescriptionInfoV1Primaries(message) => message.payload_size(),
            Self::WpImageDescriptionInfoV1PrimariesNamed(message) => message.payload_size(),
            Self::WpImageDescriptionInfoV1TfPower(message) => message.payload_size(),
            Self::WpImageDescriptionInfoV1TfNamed(message) => message.payload_size(),
            Self::WpImageDescriptionInfoV1Luminances(message) => message.payload_size(),
            Self::WpImageDescriptionInfoV1TargetPrimaries(message) => message.payload_size(),
            Self::WpImageDescriptionInfoV1TargetLuminance(message) => message.payload_size(),
            Self::WpImageDescriptionInfoV1TargetMaxCll(message) => message.payload_size(),
            Self::WpImageDescriptionInfoV1TargetMaxFall(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::WpColorManagerV1SupportedIntent(message) => message.write(writer, state).await,
            Self::WpColorManagerV1SupportedFeature(message) => message.write(writer, state).await,
            Self::WpColorManagerV1SupportedTfNamed(message) => message.write(writer, state).await,
            Self::WpColorManagerV1SupportedPrimariesNamed(message) => {
                message.write(writer, state).await
            }
            Self::WpColorManagerV1Done(message) => message.write(writer, state).await,
            Self::WpColorManagementOutputV1ImageDescriptionChanged(message) => {
                message.write(writer, state).await
            }
            Self::WpColorManagementSurfaceFeedbackV1PreferredChanged(message) => {
                message.write(writer, state).await
            }
            Self::WpColorManagementSurfaceFeedbackV1PreferredChanged2(message) => {
                message.write(writer, state).await
            }
            Self::WpImageDescriptionV1Failed(message) => message.write(writer, state).await,
            Self::WpImageDescriptionV1Ready(message) => message.write(writer, state).await,
            Self::WpImageDescriptionV1Ready2(message) => message.write(writer, state).await,
            Self::WpImageDescriptionInfoV1Done(message) => message.write(writer, state).await,
            Self::WpImageDescriptionInfoV1IccFile(message) => message.write(writer, state).await,
            Self::WpImageDescriptionInfoV1Primaries(message) => message.write(writer, state).await,
            Self::WpImageDescriptionInfoV1PrimariesNamed(message) => {
                message.write(writer, state).await
            }
            Self::WpImageDescriptionInfoV1TfPower(message) => message.write(writer, state).await,
            Self::WpImageDescriptionInfoV1TfNamed(message) => message.write(writer, state).await,
            Self::WpImageDescriptionInfoV1Luminances(message) => message.write(writer, state).await,
            Self::WpImageDescriptionInfoV1TargetPrimaries(message) => {
                message.write(writer, state).await
            }
            Self::WpImageDescriptionInfoV1TargetLuminance(message) => {
                message.write(writer, state).await
            }
            Self::WpImageDescriptionInfoV1TargetMaxCll(message) => {
                message.write(writer, state).await
            }
            Self::WpImageDescriptionInfoV1TargetMaxFall(message) => {
                message.write(writer, state).await
            }
        }
    }
}
