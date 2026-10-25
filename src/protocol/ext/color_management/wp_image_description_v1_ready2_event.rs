use crate::protocol::ext::color_management::ObjectInteface as ColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wp_image_description_v1.ready2`.
///
/// Opcode 2; available since version 2.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="ready2" since="2">
///       <description summary="the object is ready to be used">
///         Once this event has been sent, the wp_image_description_v1 object is
///         deemed "ready". Ready objects can be used to send requests and can be
///         used through other interfaces.
///
///         Every ready wp_image_description_v1 protocol object refers to an
///         underlying image description record in the compositor. Multiple protocol
///         objects may end up referring to the same record. Clients may identify
///         these "copies" by comparing their id numbers: if the numbers from two
///         protocol objects are identical, the protocol objects refer to the same
///         image description record. Two different image description records
///         cannot have the same id number simultaneously. The id number does not
///         change during the lifetime of the image description record.
///
///         Image description id number is not a protocol object id. Zero is
///         reserved as an invalid id number. It shall not be possible for a client
///         to refer to an image description by its id number in protocol. The id
///         numbers might not be portable between Wayland connections. A compositor
///         shall not send an invalid id number.
///
///         Compositors must not recycle image description id numbers.
///
///         This identity allows clients to de-duplicate image description records
///         and avoid get_information request if they already have the image
///         description information.
///       </description>
///
///       <arg name="identity_hi" type="uint" summary="high 32 bits of the 64-bit image description id number" />
///       <arg name="identity_lo" type="uint" summary="low 32 bits of the 64-bit image description id number" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WpImageDescriptionV1Ready2Event {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// high 32 bits of the 64-bit image description id number.
    pub identity_hi: u32,
    /// low 32 bits of the 64-bit image description id number.
    pub identity_lo: u32,
}
impl WpImageDescriptionV1Ready2Event {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 2;
}
impl CommandReader<Self> for WpImageDescriptionV1Ready2Event {
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
impl InterfaceReader for WpImageDescriptionV1Ready2Event {
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
                ColorManagementObjectInteface::WpImageDescriptionV1 { .. }
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
impl CommandWriter for WpImageDescriptionV1Ready2Event {
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
            "wp_image_description_v1",
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
