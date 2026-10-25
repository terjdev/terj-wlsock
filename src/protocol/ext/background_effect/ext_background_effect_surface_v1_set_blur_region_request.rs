use crate::protocol::ext::background_effect::ObjectInteface as BackgroundEffectObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `ext_background_effect_surface_v1.set_blur_region`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_blur_region">
///       <description summary="set blur region">
///         This request sets the region of the surface that will have its
///         background blurred.
///
///         The blur region is specified in the surface-local coordinates, and
///         clipped by the compositor to the surface size.
///
///         The initial value for the blur region is empty. Setting the pending
///         blur region has copy semantics, and the wl_region object can be
///         destroyed immediately. A NULL wl_region removes the effect.
///
///         The blur region is double-buffered state, and will be applied on
///         the next wl_surface.commit.
///
///         The blur algorithm is subject to compositor policies.
///
///         If the associated surface has been destroyed, the surface_destroyed
///         error will be raised.
///       </description>
///       <arg name="region" type="object" interface="wl_region" allow-null="true" summary="blur region of the surface" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ExtBackgroundEffectSurfaceV1SetBlurRegionRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// blur region of the surface.
    pub region: Option<u32>,
}
impl ExtBackgroundEffectSurfaceV1SetBlurRegionRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ExtBackgroundEffectSurfaceV1SetBlurRegionRequest {
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
impl InterfaceReader for ExtBackgroundEffectSurfaceV1SetBlurRegionRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::BackgroundEffect(
                BackgroundEffectObjectInteface::ExtBackgroundEffectSurfaceV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let region = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, region })
    }
}
impl CommandWriter for ExtBackgroundEffectSurfaceV1SetBlurRegionRequest {
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
            "ext_background_effect_surface_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.region.unwrap_or(0)).await?;
        Ok(())
    }
}
