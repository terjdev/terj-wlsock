use crate::protocol::ext::color_management::ObjectInteface as ColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wp_color_management_surface_feedback_v1.preferred_changed2`.
///
/// Opcode 1; available since version 2.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="preferred_changed2" since="2">
///       <description summary="the preferred image description changed">
///         The preferred image description is the one which likely has the most
///         performance and/or quality benefits for the compositor if used by the
///         client for its wl_surface contents. This event is sent whenever the
///         compositor changes the wl_surface's preferred image description.
///
///         This event sends the identity of the new preferred state as the argument,
///         so clients who are aware of the image description already can reuse it.
///         Otherwise, if the client wants to know what the preferred image
///         description is, it shall use the get_preferred request.
///
///         The preferred image description is not automatically used for anything.
///         It is only a hint, and clients may set any valid image description with
///         set_image_description, but there might be performance and color accuracy
///         improvements by providing the wl_surface contents in the preferred
///         image description. Therefore clients that can, should render according
///         to the preferred image description
///       </description>
///
///       <arg name="identity_hi" type="uint" summary="high 32 bits of the 64-bit image description id number" />
///       <arg name="identity_lo" type="uint" summary="low 32 bits of the 64-bit image description id number" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WpColorManagementSurfaceFeedbackV1PreferredChanged2Event {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// high 32 bits of the 64-bit image description id number.
    pub identity_hi: u32,
    /// low 32 bits of the 64-bit image description id number.
    pub identity_lo: u32,
}
impl WpColorManagementSurfaceFeedbackV1PreferredChanged2Event {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 2;
}
impl CommandReader<Self> for WpColorManagementSurfaceFeedbackV1PreferredChanged2Event {
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
impl InterfaceReader for WpColorManagementSurfaceFeedbackV1PreferredChanged2Event {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::ColorManagement(
                ColorManagementObjectInteface::WpColorManagementSurfaceFeedbackV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let identity_hi = reader.next_u32().await?;
        let identity_lo = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            identity_hi,
            identity_lo,
        })
    }
}
impl CommandWriter for WpColorManagementSurfaceFeedbackV1PreferredChanged2Event {
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
            "wp_color_management_surface_feedback_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.identity_hi).await?;
        writer.next_u32(self.identity_lo).await?;
        Ok(())
    }
}
