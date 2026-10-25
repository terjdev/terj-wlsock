use crate::protocol::ext::tablet::ObjectInteface as TabletObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_tablet_pad_strip_v2.set_feedback`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_feedback">
///       <description summary="set compositor feedback">
///     Requests the compositor to use the provided feedback string
///     associated with this strip. This request should be issued immediately
///     after a zwp_tablet_pad_group_v2.mode_switch event from the corresponding
///     group is received, or whenever the strip is mapped to a different
///     action. See zwp_tablet_pad_group_v2.mode_switch for more details.
///
///     Clients are encouraged to provide context-aware descriptions for
///     the actions associated with the strip, and compositors may use this
///     information to offer visual feedback about the button layout
///     (eg. on-screen displays).
///
///     The provided string 'description' is a UTF-8 encoded string to be
///     associated with this ring, and is considered user-visible; general
///     internationalization rules apply.
///
///     The serial argument will be that of the last
///     zwp_tablet_pad_group_v2.mode_switch event received for the group of this
///     strip. Requests providing other serials than the most recent one will be
///     ignored.
///       </description>
///       <arg name="description" type="string" summary="strip description" />
///       <arg name="serial" type="uint" summary="serial of the mode switch event" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpTabletPadStripV2SetFeedbackRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// strip description.
    pub description: String,
    /// serial of the mode switch event.
    pub serial: u32,
}
impl ZwpTabletPadStripV2SetFeedbackRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTabletPadStripV2SetFeedbackRequest {
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
impl InterfaceReader for ZwpTabletPadStripV2SetFeedbackRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletPadStripV2 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let description = reader.next_string().await?;
        let serial = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            description,
            serial,
        })
    }
}
impl CommandWriter for ZwpTabletPadStripV2SetFeedbackRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[
            FdWriter::encoded_string_size(Some(&self.description)),
            Some(4),
        ])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(
            state,
            self.object_id,
            "zwp_tablet_pad_strip_v2",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.description) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.description).await?;
        writer.next_u32(self.serial).await?;
        Ok(())
    }
}
