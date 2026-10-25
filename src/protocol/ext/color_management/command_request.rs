use super::{
    WpColorManagementOutputV1DestroyRequest, WpColorManagementOutputV1GetImageDescriptionRequest,
    WpColorManagementSurfaceFeedbackV1DestroyRequest,
    WpColorManagementSurfaceFeedbackV1GetPreferredParametricRequest,
    WpColorManagementSurfaceFeedbackV1GetPreferredRequest,
    WpColorManagementSurfaceV1DestroyRequest, WpColorManagementSurfaceV1SetImageDescriptionRequest,
    WpColorManagementSurfaceV1UnsetImageDescriptionRequest,
    WpColorManagerV1CreateIccCreatorRequest, WpColorManagerV1CreateParametricCreatorRequest,
    WpColorManagerV1CreateWindowsBt2100Request, WpColorManagerV1CreateWindowsScrgbRequest,
    WpColorManagerV1DestroyRequest, WpColorManagerV1GetImageDescriptionRequest,
    WpColorManagerV1GetOutputRequest, WpColorManagerV1GetSurfaceFeedbackRequest,
    WpColorManagerV1GetSurfaceRequest, WpImageDescriptionCreatorIccV1CreateRequest,
    WpImageDescriptionCreatorIccV1SetIccFileRequest,
    WpImageDescriptionCreatorParamsV1CreateRequest,
    WpImageDescriptionCreatorParamsV1SetLuminancesRequest,
    WpImageDescriptionCreatorParamsV1SetMasteringDisplayPrimariesRequest,
    WpImageDescriptionCreatorParamsV1SetMasteringLuminanceRequest,
    WpImageDescriptionCreatorParamsV1SetMaxCllRequest,
    WpImageDescriptionCreatorParamsV1SetMaxFallRequest,
    WpImageDescriptionCreatorParamsV1SetPrimariesNamedRequest,
    WpImageDescriptionCreatorParamsV1SetPrimariesRequest,
    WpImageDescriptionCreatorParamsV1SetTfNamedRequest,
    WpImageDescriptionCreatorParamsV1SetTfPowerRequest,
    WpImageDescriptionReferenceV1DestroyRequest, WpImageDescriptionV1DestroyRequest,
    WpImageDescriptionV1GetInformationRequest,
};
use crate::protocol::ext::color_management::ObjectInteface as ColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Color management requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [WpColorManagerV1DestroyRequest].
    WpColorManagerV1Destroy(WpColorManagerV1DestroyRequest),
    /// Message [WpColorManagerV1GetOutputRequest].
    WpColorManagerV1GetOutput(WpColorManagerV1GetOutputRequest),
    /// Message [WpColorManagerV1GetSurfaceRequest].
    WpColorManagerV1GetSurface(WpColorManagerV1GetSurfaceRequest),
    /// Message [WpColorManagerV1GetSurfaceFeedbackRequest].
    WpColorManagerV1GetSurfaceFeedback(WpColorManagerV1GetSurfaceFeedbackRequest),
    /// Message [WpColorManagerV1CreateIccCreatorRequest].
    WpColorManagerV1CreateIccCreator(WpColorManagerV1CreateIccCreatorRequest),
    /// Message [WpColorManagerV1CreateParametricCreatorRequest].
    WpColorManagerV1CreateParametricCreator(WpColorManagerV1CreateParametricCreatorRequest),
    /// Message [WpColorManagerV1CreateWindowsScrgbRequest].
    WpColorManagerV1CreateWindowsScrgb(WpColorManagerV1CreateWindowsScrgbRequest),
    /// Message [WpColorManagerV1GetImageDescriptionRequest].
    WpColorManagerV1GetImageDescription(WpColorManagerV1GetImageDescriptionRequest),
    /// Message [WpColorManagerV1CreateWindowsBt2100Request].
    WpColorManagerV1CreateWindowsBt2100(WpColorManagerV1CreateWindowsBt2100Request),
    /// Message [WpColorManagementOutputV1DestroyRequest].
    WpColorManagementOutputV1Destroy(WpColorManagementOutputV1DestroyRequest),
    /// Message [WpColorManagementOutputV1GetImageDescriptionRequest].
    WpColorManagementOutputV1GetImageDescription(
        WpColorManagementOutputV1GetImageDescriptionRequest,
    ),
    /// Message [WpColorManagementSurfaceV1DestroyRequest].
    WpColorManagementSurfaceV1Destroy(WpColorManagementSurfaceV1DestroyRequest),
    /// Message [WpColorManagementSurfaceV1SetImageDescriptionRequest].
    WpColorManagementSurfaceV1SetImageDescription(
        WpColorManagementSurfaceV1SetImageDescriptionRequest,
    ),
    /// Message [WpColorManagementSurfaceV1UnsetImageDescriptionRequest].
    WpColorManagementSurfaceV1UnsetImageDescription(
        WpColorManagementSurfaceV1UnsetImageDescriptionRequest,
    ),
    /// Message [WpColorManagementSurfaceFeedbackV1DestroyRequest].
    WpColorManagementSurfaceFeedbackV1Destroy(WpColorManagementSurfaceFeedbackV1DestroyRequest),
    /// Message [WpColorManagementSurfaceFeedbackV1GetPreferredRequest].
    WpColorManagementSurfaceFeedbackV1GetPreferred(
        WpColorManagementSurfaceFeedbackV1GetPreferredRequest,
    ),
    /// Message [WpColorManagementSurfaceFeedbackV1GetPreferredParametricRequest].
    WpColorManagementSurfaceFeedbackV1GetPreferredParametric(
        WpColorManagementSurfaceFeedbackV1GetPreferredParametricRequest,
    ),
    /// Message [WpImageDescriptionCreatorIccV1CreateRequest].
    WpImageDescriptionCreatorIccV1Create(WpImageDescriptionCreatorIccV1CreateRequest),
    /// Message [WpImageDescriptionCreatorIccV1SetIccFileRequest].
    WpImageDescriptionCreatorIccV1SetIccFile(WpImageDescriptionCreatorIccV1SetIccFileRequest),
    /// Message [WpImageDescriptionCreatorParamsV1CreateRequest].
    WpImageDescriptionCreatorParamsV1Create(WpImageDescriptionCreatorParamsV1CreateRequest),
    /// Message [WpImageDescriptionCreatorParamsV1SetTfNamedRequest].
    WpImageDescriptionCreatorParamsV1SetTfNamed(WpImageDescriptionCreatorParamsV1SetTfNamedRequest),
    /// Message [WpImageDescriptionCreatorParamsV1SetTfPowerRequest].
    WpImageDescriptionCreatorParamsV1SetTfPower(WpImageDescriptionCreatorParamsV1SetTfPowerRequest),
    /// Message [WpImageDescriptionCreatorParamsV1SetPrimariesNamedRequest].
    WpImageDescriptionCreatorParamsV1SetPrimariesNamed(
        WpImageDescriptionCreatorParamsV1SetPrimariesNamedRequest,
    ),
    /// Message [WpImageDescriptionCreatorParamsV1SetPrimariesRequest].
    WpImageDescriptionCreatorParamsV1SetPrimaries(
        WpImageDescriptionCreatorParamsV1SetPrimariesRequest,
    ),
    /// Message [WpImageDescriptionCreatorParamsV1SetLuminancesRequest].
    WpImageDescriptionCreatorParamsV1SetLuminances(
        WpImageDescriptionCreatorParamsV1SetLuminancesRequest,
    ),
    /// Message [WpImageDescriptionCreatorParamsV1SetMasteringDisplayPrimariesRequest].
    WpImageDescriptionCreatorParamsV1SetMasteringDisplayPrimaries(
        WpImageDescriptionCreatorParamsV1SetMasteringDisplayPrimariesRequest,
    ),
    /// Message [WpImageDescriptionCreatorParamsV1SetMasteringLuminanceRequest].
    WpImageDescriptionCreatorParamsV1SetMasteringLuminance(
        WpImageDescriptionCreatorParamsV1SetMasteringLuminanceRequest,
    ),
    /// Message [WpImageDescriptionCreatorParamsV1SetMaxCllRequest].
    WpImageDescriptionCreatorParamsV1SetMaxCll(WpImageDescriptionCreatorParamsV1SetMaxCllRequest),
    /// Message [WpImageDescriptionCreatorParamsV1SetMaxFallRequest].
    WpImageDescriptionCreatorParamsV1SetMaxFall(WpImageDescriptionCreatorParamsV1SetMaxFallRequest),
    /// Message [WpImageDescriptionV1DestroyRequest].
    WpImageDescriptionV1Destroy(WpImageDescriptionV1DestroyRequest),
    /// Message [WpImageDescriptionV1GetInformationRequest].
    WpImageDescriptionV1GetInformation(WpImageDescriptionV1GetInformationRequest),
    /// Message [WpImageDescriptionReferenceV1DestroyRequest].
    WpImageDescriptionReferenceV1Destroy(WpImageDescriptionReferenceV1DestroyRequest),
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
ObjectInteface::ColorManagement(ColorManagementObjectInteface::WpColorManagerV1{..})=>match opcode {
WpColorManagerV1DestroyRequest::OPCODE=>Ok(Self::WpColorManagerV1Destroy(WpColorManagerV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorManagerV1GetOutputRequest::OPCODE=>Ok(Self::WpColorManagerV1GetOutput(WpColorManagerV1GetOutputRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorManagerV1GetSurfaceRequest::OPCODE=>Ok(Self::WpColorManagerV1GetSurface(WpColorManagerV1GetSurfaceRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorManagerV1GetSurfaceFeedbackRequest::OPCODE=>Ok(Self::WpColorManagerV1GetSurfaceFeedback(WpColorManagerV1GetSurfaceFeedbackRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorManagerV1CreateIccCreatorRequest::OPCODE=>Ok(Self::WpColorManagerV1CreateIccCreator(WpColorManagerV1CreateIccCreatorRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorManagerV1CreateParametricCreatorRequest::OPCODE=>Ok(Self::WpColorManagerV1CreateParametricCreator(WpColorManagerV1CreateParametricCreatorRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorManagerV1CreateWindowsScrgbRequest::OPCODE=>Ok(Self::WpColorManagerV1CreateWindowsScrgb(WpColorManagerV1CreateWindowsScrgbRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorManagerV1GetImageDescriptionRequest::OPCODE=>Ok(Self::WpColorManagerV1GetImageDescription(WpColorManagerV1GetImageDescriptionRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorManagerV1CreateWindowsBt2100Request::OPCODE=>Ok(Self::WpColorManagerV1CreateWindowsBt2100(WpColorManagerV1CreateWindowsBt2100Request::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::ColorManagement(ColorManagementObjectInteface::WpColorManagementOutputV1{..})=>match opcode {
WpColorManagementOutputV1DestroyRequest::OPCODE=>Ok(Self::WpColorManagementOutputV1Destroy(WpColorManagementOutputV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorManagementOutputV1GetImageDescriptionRequest::OPCODE=>Ok(Self::WpColorManagementOutputV1GetImageDescription(WpColorManagementOutputV1GetImageDescriptionRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::ColorManagement(ColorManagementObjectInteface::WpColorManagementSurfaceV1{..})=>match opcode {
WpColorManagementSurfaceV1DestroyRequest::OPCODE=>Ok(Self::WpColorManagementSurfaceV1Destroy(WpColorManagementSurfaceV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorManagementSurfaceV1SetImageDescriptionRequest::OPCODE=>Ok(Self::WpColorManagementSurfaceV1SetImageDescription(WpColorManagementSurfaceV1SetImageDescriptionRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorManagementSurfaceV1UnsetImageDescriptionRequest::OPCODE=>Ok(Self::WpColorManagementSurfaceV1UnsetImageDescription(WpColorManagementSurfaceV1UnsetImageDescriptionRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::ColorManagement(ColorManagementObjectInteface::WpColorManagementSurfaceFeedbackV1{..})=>match opcode {
WpColorManagementSurfaceFeedbackV1DestroyRequest::OPCODE=>Ok(Self::WpColorManagementSurfaceFeedbackV1Destroy(WpColorManagementSurfaceFeedbackV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorManagementSurfaceFeedbackV1GetPreferredRequest::OPCODE=>Ok(Self::WpColorManagementSurfaceFeedbackV1GetPreferred(WpColorManagementSurfaceFeedbackV1GetPreferredRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorManagementSurfaceFeedbackV1GetPreferredParametricRequest::OPCODE=>Ok(Self::WpColorManagementSurfaceFeedbackV1GetPreferredParametric(WpColorManagementSurfaceFeedbackV1GetPreferredParametricRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::ColorManagement(ColorManagementObjectInteface::WpImageDescriptionCreatorIccV1{..})=>match opcode {
WpImageDescriptionCreatorIccV1CreateRequest::OPCODE=>Ok(Self::WpImageDescriptionCreatorIccV1Create(WpImageDescriptionCreatorIccV1CreateRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionCreatorIccV1SetIccFileRequest::OPCODE=>Ok(Self::WpImageDescriptionCreatorIccV1SetIccFile(WpImageDescriptionCreatorIccV1SetIccFileRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::ColorManagement(ColorManagementObjectInteface::WpImageDescriptionCreatorParamsV1{..})=>match opcode {
WpImageDescriptionCreatorParamsV1CreateRequest::OPCODE=>Ok(Self::WpImageDescriptionCreatorParamsV1Create(WpImageDescriptionCreatorParamsV1CreateRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionCreatorParamsV1SetTfNamedRequest::OPCODE=>Ok(Self::WpImageDescriptionCreatorParamsV1SetTfNamed(WpImageDescriptionCreatorParamsV1SetTfNamedRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionCreatorParamsV1SetTfPowerRequest::OPCODE=>Ok(Self::WpImageDescriptionCreatorParamsV1SetTfPower(WpImageDescriptionCreatorParamsV1SetTfPowerRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionCreatorParamsV1SetPrimariesNamedRequest::OPCODE=>Ok(Self::WpImageDescriptionCreatorParamsV1SetPrimariesNamed(WpImageDescriptionCreatorParamsV1SetPrimariesNamedRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionCreatorParamsV1SetPrimariesRequest::OPCODE=>Ok(Self::WpImageDescriptionCreatorParamsV1SetPrimaries(WpImageDescriptionCreatorParamsV1SetPrimariesRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionCreatorParamsV1SetLuminancesRequest::OPCODE=>Ok(Self::WpImageDescriptionCreatorParamsV1SetLuminances(WpImageDescriptionCreatorParamsV1SetLuminancesRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionCreatorParamsV1SetMasteringDisplayPrimariesRequest::OPCODE=>Ok(Self::WpImageDescriptionCreatorParamsV1SetMasteringDisplayPrimaries(WpImageDescriptionCreatorParamsV1SetMasteringDisplayPrimariesRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionCreatorParamsV1SetMasteringLuminanceRequest::OPCODE=>Ok(Self::WpImageDescriptionCreatorParamsV1SetMasteringLuminance(WpImageDescriptionCreatorParamsV1SetMasteringLuminanceRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionCreatorParamsV1SetMaxCllRequest::OPCODE=>Ok(Self::WpImageDescriptionCreatorParamsV1SetMaxCll(WpImageDescriptionCreatorParamsV1SetMaxCllRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionCreatorParamsV1SetMaxFallRequest::OPCODE=>Ok(Self::WpImageDescriptionCreatorParamsV1SetMaxFall(WpImageDescriptionCreatorParamsV1SetMaxFallRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::ColorManagement(ColorManagementObjectInteface::WpImageDescriptionV1{..})=>match opcode {
WpImageDescriptionV1DestroyRequest::OPCODE=>Ok(Self::WpImageDescriptionV1Destroy(WpImageDescriptionV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpImageDescriptionV1GetInformationRequest::OPCODE=>Ok(Self::WpImageDescriptionV1GetInformation(WpImageDescriptionV1GetInformationRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::ColorManagement(ColorManagementObjectInteface::WpImageDescriptionReferenceV1{..})=>match opcode {
WpImageDescriptionReferenceV1DestroyRequest::OPCODE=>Ok(Self::WpImageDescriptionReferenceV1Destroy(WpImageDescriptionReferenceV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
_=>Err(FdReaderError::InvalidProtocol),}
    }
}
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::WpColorManagerV1Destroy(message) => message.object_id(),
            Self::WpColorManagerV1GetOutput(message) => message.object_id(),
            Self::WpColorManagerV1GetSurface(message) => message.object_id(),
            Self::WpColorManagerV1GetSurfaceFeedback(message) => message.object_id(),
            Self::WpColorManagerV1CreateIccCreator(message) => message.object_id(),
            Self::WpColorManagerV1CreateParametricCreator(message) => message.object_id(),
            Self::WpColorManagerV1CreateWindowsScrgb(message) => message.object_id(),
            Self::WpColorManagerV1GetImageDescription(message) => message.object_id(),
            Self::WpColorManagerV1CreateWindowsBt2100(message) => message.object_id(),
            Self::WpColorManagementOutputV1Destroy(message) => message.object_id(),
            Self::WpColorManagementOutputV1GetImageDescription(message) => message.object_id(),
            Self::WpColorManagementSurfaceV1Destroy(message) => message.object_id(),
            Self::WpColorManagementSurfaceV1SetImageDescription(message) => message.object_id(),
            Self::WpColorManagementSurfaceV1UnsetImageDescription(message) => message.object_id(),
            Self::WpColorManagementSurfaceFeedbackV1Destroy(message) => message.object_id(),
            Self::WpColorManagementSurfaceFeedbackV1GetPreferred(message) => message.object_id(),
            Self::WpColorManagementSurfaceFeedbackV1GetPreferredParametric(message) => {
                message.object_id()
            }
            Self::WpImageDescriptionCreatorIccV1Create(message) => message.object_id(),
            Self::WpImageDescriptionCreatorIccV1SetIccFile(message) => message.object_id(),
            Self::WpImageDescriptionCreatorParamsV1Create(message) => message.object_id(),
            Self::WpImageDescriptionCreatorParamsV1SetTfNamed(message) => message.object_id(),
            Self::WpImageDescriptionCreatorParamsV1SetTfPower(message) => message.object_id(),
            Self::WpImageDescriptionCreatorParamsV1SetPrimariesNamed(message) => {
                message.object_id()
            }
            Self::WpImageDescriptionCreatorParamsV1SetPrimaries(message) => message.object_id(),
            Self::WpImageDescriptionCreatorParamsV1SetLuminances(message) => message.object_id(),
            Self::WpImageDescriptionCreatorParamsV1SetMasteringDisplayPrimaries(message) => {
                message.object_id()
            }
            Self::WpImageDescriptionCreatorParamsV1SetMasteringLuminance(message) => {
                message.object_id()
            }
            Self::WpImageDescriptionCreatorParamsV1SetMaxCll(message) => message.object_id(),
            Self::WpImageDescriptionCreatorParamsV1SetMaxFall(message) => message.object_id(),
            Self::WpImageDescriptionV1Destroy(message) => message.object_id(),
            Self::WpImageDescriptionV1GetInformation(message) => message.object_id(),
            Self::WpImageDescriptionReferenceV1Destroy(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WpColorManagerV1Destroy(message) => message.opcode(),
            Self::WpColorManagerV1GetOutput(message) => message.opcode(),
            Self::WpColorManagerV1GetSurface(message) => message.opcode(),
            Self::WpColorManagerV1GetSurfaceFeedback(message) => message.opcode(),
            Self::WpColorManagerV1CreateIccCreator(message) => message.opcode(),
            Self::WpColorManagerV1CreateParametricCreator(message) => message.opcode(),
            Self::WpColorManagerV1CreateWindowsScrgb(message) => message.opcode(),
            Self::WpColorManagerV1GetImageDescription(message) => message.opcode(),
            Self::WpColorManagerV1CreateWindowsBt2100(message) => message.opcode(),
            Self::WpColorManagementOutputV1Destroy(message) => message.opcode(),
            Self::WpColorManagementOutputV1GetImageDescription(message) => message.opcode(),
            Self::WpColorManagementSurfaceV1Destroy(message) => message.opcode(),
            Self::WpColorManagementSurfaceV1SetImageDescription(message) => message.opcode(),
            Self::WpColorManagementSurfaceV1UnsetImageDescription(message) => message.opcode(),
            Self::WpColorManagementSurfaceFeedbackV1Destroy(message) => message.opcode(),
            Self::WpColorManagementSurfaceFeedbackV1GetPreferred(message) => message.opcode(),
            Self::WpColorManagementSurfaceFeedbackV1GetPreferredParametric(message) => {
                message.opcode()
            }
            Self::WpImageDescriptionCreatorIccV1Create(message) => message.opcode(),
            Self::WpImageDescriptionCreatorIccV1SetIccFile(message) => message.opcode(),
            Self::WpImageDescriptionCreatorParamsV1Create(message) => message.opcode(),
            Self::WpImageDescriptionCreatorParamsV1SetTfNamed(message) => message.opcode(),
            Self::WpImageDescriptionCreatorParamsV1SetTfPower(message) => message.opcode(),
            Self::WpImageDescriptionCreatorParamsV1SetPrimariesNamed(message) => message.opcode(),
            Self::WpImageDescriptionCreatorParamsV1SetPrimaries(message) => message.opcode(),
            Self::WpImageDescriptionCreatorParamsV1SetLuminances(message) => message.opcode(),
            Self::WpImageDescriptionCreatorParamsV1SetMasteringDisplayPrimaries(message) => {
                message.opcode()
            }
            Self::WpImageDescriptionCreatorParamsV1SetMasteringLuminance(message) => {
                message.opcode()
            }
            Self::WpImageDescriptionCreatorParamsV1SetMaxCll(message) => message.opcode(),
            Self::WpImageDescriptionCreatorParamsV1SetMaxFall(message) => message.opcode(),
            Self::WpImageDescriptionV1Destroy(message) => message.opcode(),
            Self::WpImageDescriptionV1GetInformation(message) => message.opcode(),
            Self::WpImageDescriptionReferenceV1Destroy(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WpColorManagerV1Destroy(message) => message.payload_size(),
            Self::WpColorManagerV1GetOutput(message) => message.payload_size(),
            Self::WpColorManagerV1GetSurface(message) => message.payload_size(),
            Self::WpColorManagerV1GetSurfaceFeedback(message) => message.payload_size(),
            Self::WpColorManagerV1CreateIccCreator(message) => message.payload_size(),
            Self::WpColorManagerV1CreateParametricCreator(message) => message.payload_size(),
            Self::WpColorManagerV1CreateWindowsScrgb(message) => message.payload_size(),
            Self::WpColorManagerV1GetImageDescription(message) => message.payload_size(),
            Self::WpColorManagerV1CreateWindowsBt2100(message) => message.payload_size(),
            Self::WpColorManagementOutputV1Destroy(message) => message.payload_size(),
            Self::WpColorManagementOutputV1GetImageDescription(message) => message.payload_size(),
            Self::WpColorManagementSurfaceV1Destroy(message) => message.payload_size(),
            Self::WpColorManagementSurfaceV1SetImageDescription(message) => message.payload_size(),
            Self::WpColorManagementSurfaceV1UnsetImageDescription(message) => {
                message.payload_size()
            }
            Self::WpColorManagementSurfaceFeedbackV1Destroy(message) => message.payload_size(),
            Self::WpColorManagementSurfaceFeedbackV1GetPreferred(message) => message.payload_size(),
            Self::WpColorManagementSurfaceFeedbackV1GetPreferredParametric(message) => {
                message.payload_size()
            }
            Self::WpImageDescriptionCreatorIccV1Create(message) => message.payload_size(),
            Self::WpImageDescriptionCreatorIccV1SetIccFile(message) => message.payload_size(),
            Self::WpImageDescriptionCreatorParamsV1Create(message) => message.payload_size(),
            Self::WpImageDescriptionCreatorParamsV1SetTfNamed(message) => message.payload_size(),
            Self::WpImageDescriptionCreatorParamsV1SetTfPower(message) => message.payload_size(),
            Self::WpImageDescriptionCreatorParamsV1SetPrimariesNamed(message) => {
                message.payload_size()
            }
            Self::WpImageDescriptionCreatorParamsV1SetPrimaries(message) => message.payload_size(),
            Self::WpImageDescriptionCreatorParamsV1SetLuminances(message) => message.payload_size(),
            Self::WpImageDescriptionCreatorParamsV1SetMasteringDisplayPrimaries(message) => {
                message.payload_size()
            }
            Self::WpImageDescriptionCreatorParamsV1SetMasteringLuminance(message) => {
                message.payload_size()
            }
            Self::WpImageDescriptionCreatorParamsV1SetMaxCll(message) => message.payload_size(),
            Self::WpImageDescriptionCreatorParamsV1SetMaxFall(message) => message.payload_size(),
            Self::WpImageDescriptionV1Destroy(message) => message.payload_size(),
            Self::WpImageDescriptionV1GetInformation(message) => message.payload_size(),
            Self::WpImageDescriptionReferenceV1Destroy(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::WpColorManagerV1Destroy(message) => message.write(writer, state).await,
            Self::WpColorManagerV1GetOutput(message) => message.write(writer, state).await,
            Self::WpColorManagerV1GetSurface(message) => message.write(writer, state).await,
            Self::WpColorManagerV1GetSurfaceFeedback(message) => message.write(writer, state).await,
            Self::WpColorManagerV1CreateIccCreator(message) => message.write(writer, state).await,
            Self::WpColorManagerV1CreateParametricCreator(message) => {
                message.write(writer, state).await
            }
            Self::WpColorManagerV1CreateWindowsScrgb(message) => message.write(writer, state).await,
            Self::WpColorManagerV1GetImageDescription(message) => {
                message.write(writer, state).await
            }
            Self::WpColorManagerV1CreateWindowsBt2100(message) => {
                message.write(writer, state).await
            }
            Self::WpColorManagementOutputV1Destroy(message) => message.write(writer, state).await,
            Self::WpColorManagementOutputV1GetImageDescription(message) => {
                message.write(writer, state).await
            }
            Self::WpColorManagementSurfaceV1Destroy(message) => message.write(writer, state).await,
            Self::WpColorManagementSurfaceV1SetImageDescription(message) => {
                message.write(writer, state).await
            }
            Self::WpColorManagementSurfaceV1UnsetImageDescription(message) => {
                message.write(writer, state).await
            }
            Self::WpColorManagementSurfaceFeedbackV1Destroy(message) => {
                message.write(writer, state).await
            }
            Self::WpColorManagementSurfaceFeedbackV1GetPreferred(message) => {
                message.write(writer, state).await
            }
            Self::WpColorManagementSurfaceFeedbackV1GetPreferredParametric(message) => {
                message.write(writer, state).await
            }
            Self::WpImageDescriptionCreatorIccV1Create(message) => {
                message.write(writer, state).await
            }
            Self::WpImageDescriptionCreatorIccV1SetIccFile(message) => {
                message.write(writer, state).await
            }
            Self::WpImageDescriptionCreatorParamsV1Create(message) => {
                message.write(writer, state).await
            }
            Self::WpImageDescriptionCreatorParamsV1SetTfNamed(message) => {
                message.write(writer, state).await
            }
            Self::WpImageDescriptionCreatorParamsV1SetTfPower(message) => {
                message.write(writer, state).await
            }
            Self::WpImageDescriptionCreatorParamsV1SetPrimariesNamed(message) => {
                message.write(writer, state).await
            }
            Self::WpImageDescriptionCreatorParamsV1SetPrimaries(message) => {
                message.write(writer, state).await
            }
            Self::WpImageDescriptionCreatorParamsV1SetLuminances(message) => {
                message.write(writer, state).await
            }
            Self::WpImageDescriptionCreatorParamsV1SetMasteringDisplayPrimaries(message) => {
                message.write(writer, state).await
            }
            Self::WpImageDescriptionCreatorParamsV1SetMasteringLuminance(message) => {
                message.write(writer, state).await
            }
            Self::WpImageDescriptionCreatorParamsV1SetMaxCll(message) => {
                message.write(writer, state).await
            }
            Self::WpImageDescriptionCreatorParamsV1SetMaxFall(message) => {
                message.write(writer, state).await
            }
            Self::WpImageDescriptionV1Destroy(message) => message.write(writer, state).await,
            Self::WpImageDescriptionV1GetInformation(message) => message.write(writer, state).await,
            Self::WpImageDescriptionReferenceV1Destroy(message) => {
                message.write(writer, state).await
            }
        }
    }
}
