use super::ObjectInteface as XxInputMethodObjectInteface;
use crate::protocol::ext::xx_text_input::XxTextInputV3SupportedFeatures;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xx_input_method_v1.announce_supported_features`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="announce_supported_features" since="3">
///       <description summary="announce extra supported features">
///         Notifies the input method what the currently active text input client is able to do.
///
///         This event should come within the same .done sequence as .activate. Otherwise, the input method may ignore it.
///
///         Values set with this event are double-buffered. They will get applied
///         on the next .done event.
///         They get reset to initial on the next committed deactivate event.
///
///         The initial value for features is none.
///       </description>
///       <arg name="features" type="uint" enum="xx_text_input_v3.supported_features" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XxInputMethodV1AnnounceSupportedFeaturesEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `features`.
    pub features: XxTextInputV3SupportedFeatures,
}
impl XxInputMethodV1AnnounceSupportedFeaturesEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 6;
    /// Minimum interface version.
    pub const SINCE: u32 = 3;
}
impl CommandReader<Self> for XxInputMethodV1AnnounceSupportedFeaturesEvent {
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
impl InterfaceReader for XxInputMethodV1AnnounceSupportedFeaturesEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxInputMethod(XxInputMethodObjectInteface::XxInputMethodV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let features = XxTextInputV3SupportedFeatures(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            features,
        })
    }
}
impl CommandWriter for XxInputMethodV1AnnounceSupportedFeaturesEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface =
            Self::writer_event_interface(state, self.object_id, "xx_input_method_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XxInputMethod(XxInputMethodObjectInteface::XxInputMethodV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.features.0).await?;
        Ok(())
    }
}
