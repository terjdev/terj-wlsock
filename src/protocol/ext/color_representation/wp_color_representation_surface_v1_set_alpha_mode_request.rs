use super::WpColorRepresentationSurfaceV1AlphaMode;
use crate::protocol::ext::color_representation::ObjectInteface as ColorRepresentationObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_color_representation_surface_v1.set_alpha_mode`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_alpha_mode">
///       <description summary="set the surface alpha mode">
///         If this protocol object is inert, the protocol error inert is raised.
///
///         Assuming an alpha channel exists, it is always linear. The alpha mode
///         determines whether and how the color channels include pre-multiplied
///         alpha. Using straight alpha might have performance benefits.
///
///         Only alpha modes advertised by the compositor are allowed to be used as
///         argument for this request. The "alpha_mode" protocol error is raised
///         otherwise.
///
///         Alpha mode is double buffered, see wl_surface.commit.
///       </description>
///       <arg name="alpha_mode" type="uint" enum="alpha_mode" summary="alpha mode" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpColorRepresentationSurfaceV1SetAlphaModeRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// alpha mode.
    pub alpha_mode: WpColorRepresentationSurfaceV1AlphaMode,
}
impl WpColorRepresentationSurfaceV1SetAlphaModeRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpColorRepresentationSurfaceV1SetAlphaModeRequest {
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
impl InterfaceReader for WpColorRepresentationSurfaceV1SetAlphaModeRequest {
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
impl CommandWriter for WpColorRepresentationSurfaceV1SetAlphaModeRequest {
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
        writer.next_u32(self.alpha_mode.0).await?;
        Ok(())
    }
}
