use super::WpColorRepresentationSurfaceV1Coefficients;
use super::WpColorRepresentationSurfaceV1Range;
use crate::protocol::ext::color_representation::ObjectInteface as ColorRepresentationObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wp_color_representation_manager_v1.supported_coefficients_and_ranges`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="supported_coefficients_and_ranges">
///       <description summary="supported matrix coefficients and ranges">
///         When this object is created, it shall immediately send this event once
///         for each matrix coefficient and color range combination the compositor
///         supports.
///
///         For the definition of the supported values, see the
///         wp_color_representation_surface_v1::coefficients and
///         wp_color_representation_surface_v1::range enums.
///       </description>
///       <arg name="coefficients" type="uint" enum="wp_color_representation_surface_v1.coefficients" summary="supported matrix coefficients" />
///       <arg name="range" type="uint" enum="wp_color_representation_surface_v1.range" summary="full range flag" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WpColorRepresentationManagerV1SupportedCoefficientsAndRangesEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// supported matrix coefficients.
    pub coefficients: WpColorRepresentationSurfaceV1Coefficients,
    /// full range flag.
    pub range: WpColorRepresentationSurfaceV1Range,
}
impl WpColorRepresentationManagerV1SupportedCoefficientsAndRangesEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpColorRepresentationManagerV1SupportedCoefficientsAndRangesEvent {
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
impl InterfaceReader for WpColorRepresentationManagerV1SupportedCoefficientsAndRangesEvent {
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
        let coefficients = WpColorRepresentationSurfaceV1Coefficients(reader.next_u32().await?);
        let range = WpColorRepresentationSurfaceV1Range(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            coefficients,
            range,
        })
    }
}
impl CommandWriter for WpColorRepresentationManagerV1SupportedCoefficientsAndRangesEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4)])
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
        writer.next_u32(self.coefficients.0).await?;
        writer.next_u32(self.range.0).await?;
        Ok(())
    }
}
