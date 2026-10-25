use super::ObjectInteface as XxFractionalScaleObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_fractional_scale_manager_v2.get_fractional_scale`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="get_fractional_scale">
///       <description summary="create an interface to enable fractional scaling">
///         Create an interface object for a wl_surface to communicate scale.
///         If the given wl_surface already has a xx_fractional_scale_v2 object
///         associated, the fractional_scale_exists protocol error is raised.
///       </description>
///       <arg name="id" type="new_id" interface="xx_fractional_scale_v2" summary="the new scale interface" />
///       <arg name="surface" type="object" interface="wl_surface" summary="the surface" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxFractionalScaleManagerV2GetFractionalScaleRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the new scale interface.
    pub id: u32,
    /// the surface.
    pub surface: u32,
}
impl XxFractionalScaleManagerV2GetFractionalScaleRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxFractionalScaleManagerV2GetFractionalScaleRequest {
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
impl InterfaceReader for XxFractionalScaleManagerV2GetFractionalScaleRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxFractionalScale(
                XxFractionalScaleObjectInteface::XxFractionalScaleManagerV2 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = reader.next_u32().await?;
        let surface = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::XxFractionalScale(
                XxFractionalScaleObjectInteface::XxFractionalScaleV2 {
                    version: interface.version().min(1),
                },
            );
            Self::reader_state(state)?
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            surface,
        })
    }
}
impl CommandWriter for XxFractionalScaleManagerV2GetFractionalScaleRequest {
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
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "xx_fractional_scale_manager_v2",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::XxFractionalScale(
                XxFractionalScaleObjectInteface::XxFractionalScaleManagerV2 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::XxFractionalScale(
                XxFractionalScaleObjectInteface::XxFractionalScaleV2 {
                    version: interface.version().min(1),
                },
            );
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_u32(self.surface).await?;
        Ok(())
    }
}
