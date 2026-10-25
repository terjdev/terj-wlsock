use crate::protocol::ext::color_management::ObjectInteface as ColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wp_image_description_info_v1.target_primaries`.
///
/// Opcode 7; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="target_primaries">
///       <description summary="target primaries as chromaticity coordinates">
///         Provides the color primaries and white point of the target color volume
///         using CIE 1931 xy chromaticity coordinates. This is compatible with the
///         SMPTE ST 2086 definition of HDR static metadata for mastering displays.
///
///         While primary color volume is about how color is encoded, the target
///         color volume is the actually displayable color volume.
///
///         Each coordinate value is multiplied by 1 million to get the argument
///         value to carry precision of 6 decimals.
///       </description>
///
///       <arg name="r_x" type="int" summary="Red x * 1M" />
///       <arg name="r_y" type="int" summary="Red y * 1M" />
///       <arg name="g_x" type="int" summary="Green x * 1M" />
///       <arg name="g_y" type="int" summary="Green y * 1M" />
///       <arg name="b_x" type="int" summary="Blue x * 1M" />
///       <arg name="b_y" type="int" summary="Blue y * 1M" />
///       <arg name="w_x" type="int" summary="White x * 1M" />
///       <arg name="w_y" type="int" summary="White y * 1M" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WpImageDescriptionInfoV1TargetPrimariesEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Red x * 1M.
    pub r_x: i32,
    /// Red y * 1M.
    pub r_y: i32,
    /// Green x * 1M.
    pub g_x: i32,
    /// Green y * 1M.
    pub g_y: i32,
    /// Blue x * 1M.
    pub b_x: i32,
    /// Blue y * 1M.
    pub b_y: i32,
    /// White x * 1M.
    pub w_x: i32,
    /// White y * 1M.
    pub w_y: i32,
}
impl WpImageDescriptionInfoV1TargetPrimariesEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 7;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpImageDescriptionInfoV1TargetPrimariesEvent {
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
impl InterfaceReader for WpImageDescriptionInfoV1TargetPrimariesEvent {
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
                ColorManagementObjectInteface::WpImageDescriptionInfoV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let r_x = reader.next_i32().await?;
        let r_y = reader.next_i32().await?;
        let g_x = reader.next_i32().await?;
        let g_y = reader.next_i32().await?;
        let b_x = reader.next_i32().await?;
        let b_y = reader.next_i32().await?;
        let w_x = reader.next_i32().await?;
        let w_y = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            r_x,
            r_y,
            g_x,
            g_y,
            b_x,
            b_y,
            w_x,
            w_y,
        })
    }
}
impl CommandWriter for WpImageDescriptionInfoV1TargetPrimariesEvent {
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
            Some(4),
        ])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_event_interface(
            state,
            self.object_id,
            "wp_image_description_info_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.r_x).await?;
        writer.next_i32(self.r_y).await?;
        writer.next_i32(self.g_x).await?;
        writer.next_i32(self.g_y).await?;
        writer.next_i32(self.b_x).await?;
        writer.next_i32(self.b_y).await?;
        writer.next_i32(self.w_x).await?;
        writer.next_i32(self.w_y).await?;
        Ok(())
    }
}
