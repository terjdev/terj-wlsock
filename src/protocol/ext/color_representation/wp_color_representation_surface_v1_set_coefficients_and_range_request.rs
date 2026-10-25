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

/// Client request `wp_color_representation_surface_v1.set_coefficients_and_range`.
///
/// Opcode 2; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_coefficients_and_range">
///       <description summary="set the matrix coefficients and range">
///         If this protocol object is inert, the protocol error inert is raised.
///
///         Set the matrix coefficients and video range which defines the formula
///         and the related constants used to derive red, green and blue signals.
///         Usually coefficients correspond to MatrixCoefficients code points in
///         H.273.
///
///         Only combinations advertised by the compositor are allowed to be used as
///         argument for this request. The "coefficients" protocol error is raised
///         otherwise.
///
///         A call to wl_surface.commit verifies that the pixel format and the
///         coefficients-range combination in the committed surface contents are
///         compatible, if contents exist. The "pixel_format" protocol error is
///         raised otherwise.
///
///         A pixel format is compatible with the coefficients-range combination if
///         the related equations and conventions as defined in H.273 can produce
///         the color channels (RGB or YCbCr) of the pixel format.
///
///         For the definition of the supported combination, see the
///         wp_color_representation_surface_v1::coefficients and
///         wp_color_representation_surface_v1::range enums.
///
///         The coefficients-range combination is double-buffered, see
///         wl_surface.commit.
///       </description>
///       <arg name="coefficients" type="uint" enum="coefficients" summary="matrix coefficients" />
///       <arg name="range" type="uint" enum="range" summary="range" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpColorRepresentationSurfaceV1SetCoefficientsAndRangeRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// matrix coefficients.
    pub coefficients: WpColorRepresentationSurfaceV1Coefficients,
    /// range.
    pub range: WpColorRepresentationSurfaceV1Range,
}
impl WpColorRepresentationSurfaceV1SetCoefficientsAndRangeRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpColorRepresentationSurfaceV1SetCoefficientsAndRangeRequest {
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
impl InterfaceReader for WpColorRepresentationSurfaceV1SetCoefficientsAndRangeRequest {
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
                ColorRepresentationObjectInteface::WpColorRepresentationSurfaceV1 { .. }
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
impl CommandWriter for WpColorRepresentationSurfaceV1SetCoefficientsAndRangeRequest {
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
        Self::writer_request_interface(
            state,
            self.object_id,
            "wp_color_representation_surface_v1",
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
