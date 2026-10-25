use super::ObjectInteface as XxInputMethodObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xx_input_popup_surface_v2.start_configure`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="start_configure">
///       <description summary="configure the popup surface">
///         The start_configure event updates the popup geometry and marks the start of a configure sequence.
///
///         The anchor_* arguments represent the geometry of the anchor to which the popup was attached, relative to the upper left corner of the
///         popup's surface. Note that this makes anchor_x, anchor_y the reverse of the what they represent in xdg_popup.
///
///         A configure sequence is a set of one or more events configuring the state of the
///         input_popup_surface, starting with this event and ending with input_method.done. After the input_method.done event, the configure sequence is considered submitted.
///
///         State set by event in a configure sequence is conceptually double-buffered.
///         Every argument overwrites its previous value. The state change should get applied atomically with the input_method.done ending the sequence, and the value of serial should return to the undefined value.
///
///         Events on the input_popup_surface object received outside a configure sequence (while serial is undefined) must be ignored by the client.
///
///         A configure sequence shall be sent every time the compositor (re)positions the popup, or the shape of the anchor changes, for example after popup creation, or in response to text being typed and the text cursor moving.
///
///         The client may update the surface in response to input_method.done. Unless the popup is destroyed by the input_method.done, the client must reply with
///         an .ack_configure request with the serial sent in the start_configure event at
///         some point after the sequence ends and before committing the new surface.
///
///         If the client receives multiple configure sequences before it can respond
///         to one, it is free to discard all but the last event it received.
///
///
///       </description>
///       <arg name="width" type="uint" summary="popup width" />
///       <arg name="height" type="uint" summary="popup height" />
///       <arg name="anchor_x" type="int" summary="x position relative to anchor geometry" />
///       <arg name="anchor_y" type="int" summary="y position relative to anchor geometry" />
///       <arg name="anchor_width" type="uint" summary="width of the anchor area" />
///       <arg name="anchor_height" type="uint" summary="height of the anchor area" />
///       <arg name="serial" type="uint" summary="serial of the configure sequence" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XxInputPopupSurfaceV2StartConfigureEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// popup width.
    pub width: u32,
    /// popup height.
    pub height: u32,
    /// x position relative to anchor geometry.
    pub anchor_x: i32,
    /// y position relative to anchor geometry.
    pub anchor_y: i32,
    /// width of the anchor area.
    pub anchor_width: u32,
    /// height of the anchor area.
    pub anchor_height: u32,
    /// serial of the configure sequence.
    pub serial: u32,
}
impl XxInputPopupSurfaceV2StartConfigureEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxInputPopupSurfaceV2StartConfigureEvent {
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
impl InterfaceReader for XxInputPopupSurfaceV2StartConfigureEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxInputMethod(
                XxInputMethodObjectInteface::XxInputPopupSurfaceV2 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let width = reader.next_u32().await?;
        let height = reader.next_u32().await?;
        let anchor_x = reader.next_i32().await?;
        let anchor_y = reader.next_i32().await?;
        let anchor_width = reader.next_u32().await?;
        let anchor_height = reader.next_u32().await?;
        let serial = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            width,
            height,
            anchor_x,
            anchor_y,
            anchor_width,
            anchor_height,
            serial,
        })
    }
}
impl CommandWriter for XxInputPopupSurfaceV2StartConfigureEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[
            Some(4),
            Some(4),
            Some(4),
            Some(4),
            Some(4),
            Some(4),
            Some(4),
        ])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "xx_input_popup_surface_v2",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::XxInputMethod(
                XxInputMethodObjectInteface::XxInputPopupSurfaceV2 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.width).await?;
        writer.next_u32(self.height).await?;
        writer.next_i32(self.anchor_x).await?;
        writer.next_i32(self.anchor_y).await?;
        writer.next_u32(self.anchor_width).await?;
        writer.next_u32(self.anchor_height).await?;
        writer.next_u32(self.serial).await?;
        Ok(())
    }
}
