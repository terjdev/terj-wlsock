use super::WpColorRepresentationSurfaceV1ChromaLocation;
use crate::protocol::ext::color_representation::ObjectInteface as ColorRepresentationObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_color_representation_surface_v1.set_chroma_location`.
///
/// Opcode 3; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_chroma_location">
///       <description summary="set the chroma location">
///         If this protocol object is inert, the protocol error inert is raised.
///
///         Set the chroma location type which defines the position of downsampled
///         chroma samples, corresponding to Chroma420SampleLocType code points in
///         H.273.
///
///         An invalid chroma location enum value raises the "chroma_location"
///         protocol error.
///
///         A call to wl_surface.commit verifies that the pixel format and chroma
///         location type in the committed surface contents are compatible, if
///         contents exist. The "pixel_format" protocol error is raised otherwise.
///
///         For the definition of the supported chroma location types, see the
///         wp_color_representation_surface_v1::chroma_location enum.
///
///         The chroma location type is double-buffered, see wl_surface.commit.
///       </description>
///       <arg name="chroma_location" type="uint" enum="chroma_location" summary="chroma sample location" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpColorRepresentationSurfaceV1SetChromaLocationRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// chroma sample location.
    pub chroma_location: WpColorRepresentationSurfaceV1ChromaLocation,
}
impl WpColorRepresentationSurfaceV1SetChromaLocationRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpColorRepresentationSurfaceV1SetChromaLocationRequest {
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
impl InterfaceReader for WpColorRepresentationSurfaceV1SetChromaLocationRequest {
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
        let chroma_location =
            WpColorRepresentationSurfaceV1ChromaLocation(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            chroma_location,
        })
    }
}
impl CommandWriter for WpColorRepresentationSurfaceV1SetChromaLocationRequest {
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
        Self::writer_request_interface(
            state,
            self.object_id,
            "wp_color_representation_surface_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.chroma_location.0).await?;
        Ok(())
    }
}
