use super::WpColorRepresentationSurfaceV1AlphaMode;
use crate::protocol::ext::color_representation::ObjectInteface as ColorRepresentationObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wp_color_representation_manager_v1.supported_alpha_mode`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="supported_alpha_mode">
///       <description summary="supported alpha modes">
///         When this object is created, it shall immediately send this event once
///         for each alpha mode the compositor supports.
///
///         For the definition of the supported values, see the
///         wp_color_representation_surface_v1::alpha_mode enum.
///       </description>
///       <arg name="alpha_mode" type="uint" enum="wp_color_representation_surface_v1.alpha_mode" summary="supported alpha mode" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WpColorRepresentationManagerV1SupportedAlphaModeEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// supported alpha mode.
    pub alpha_mode: WpColorRepresentationSurfaceV1AlphaMode,
}
impl WpColorRepresentationManagerV1SupportedAlphaModeEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpColorRepresentationManagerV1SupportedAlphaModeEvent {
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
impl InterfaceReader for WpColorRepresentationManagerV1SupportedAlphaModeEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::ColorRepresentation(
                ColorRepresentationObjectInteface::WpColorRepresentationManagerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let alpha_mode = WpColorRepresentationSurfaceV1AlphaMode(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            alpha_mode,
        })
    }
}
impl CommandWriter for WpColorRepresentationManagerV1SupportedAlphaModeEvent {
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
        Self::writer_event_interface(
            state,
            self.object_id,
            "wp_color_representation_manager_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.alpha_mode.0).await?;
        Ok(())
    }
}
