use crate::protocol::ext::alpha_modifier::ObjectInteface as AlphaModifierObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_alpha_modifier_surface_v1.set_multiplier`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_multiplier">
///       <description summary="specify the alpha multiplier">
///         Sets the alpha multiplier for the surface. The alpha multiplier is
///         double-buffered state, see wl_surface.commit for details.
///
///         This factor is applied in the compositor's blending space, as an
///         additional step after the processing of per-pixel alpha values for the
///         wl_surface. The exact meaning of the factor is thus undefined, unless
///         the blending space is specified in a different extension.
///
///         This multiplier is applied even if the buffer attached to the
///         wl_surface doesn't have an alpha channel; in that case an alpha value
///         of one is used instead.
///
///         Zero means completely transparent, UINT32_MAX means completely opaque.
///       </description>
///       <arg name="factor" type="uint" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpAlphaModifierSurfaceV1SetMultiplierRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `factor`.
    pub factor: u32,
}
impl WpAlphaModifierSurfaceV1SetMultiplierRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpAlphaModifierSurfaceV1SetMultiplierRequest {
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
impl InterfaceReader for WpAlphaModifierSurfaceV1SetMultiplierRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::AlphaModifier(
                AlphaModifierObjectInteface::WpAlphaModifierSurfaceV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let factor = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, factor })
    }
}
impl CommandWriter for WpAlphaModifierSurfaceV1SetMultiplierRequest {
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
            "wp_alpha_modifier_surface_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.factor).await?;
        Ok(())
    }
}
