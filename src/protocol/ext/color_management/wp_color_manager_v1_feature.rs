use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_color_manager_v1.feature`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="feature">
///       <description summary="compositor supported features" />
///
///       <entry name="icc_v2_v4" value="0" summary="create_icc_creator request" />
///       <entry name="parametric" value="1" summary="create_parametric_creator request" />
///       <entry name="set_primaries" value="2" summary="parametric set_primaries request" />
///       <entry name="set_tf_power" value="3" summary="parametric set_tf_power request" />
///       <entry name="set_luminances" value="4" summary="parametric set_luminances request" />
///       <entry name="set_mastering_display_primaries" value="5">
///         <description summary="parametric set_mastering_display_primaries request">
///           The compositor supports set_mastering_display_primaries request with a
///           target color volume fully contained inside the primary color volume.
///         </description>
///       </entry>
///       <entry name="extended_target_volume" value="6">
///         <description summary="parametric target exceeds primary color volume">
///           The compositor additionally supports target color volumes that
///           extend outside of the primary color volume.
///
///           This can only be advertised if feature set_mastering_display_primaries
///           is supported as well.
///         </description>
///       </entry>
///       <entry name="windows_scrgb" value="7" summary="create_windows_scrgb request" />
///       <entry name="windows_bt2100" value="8" summary="create_windows_bt2100 request" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpColorManagerV1Feature(pub u32);
impl WpColorManagerV1Feature {
    /// create_icc_creator request; available since version 1.
    pub const ICC_V2_V4: Self = Self(0);
    /// create_parametric_creator request; available since version 1.
    pub const PARAMETRIC: Self = Self(1);
    /// parametric set_primaries request; available since version 1.
    pub const SET_PRIMARIES: Self = Self(2);
    /// parametric set_tf_power request; available since version 1.
    pub const SET_TF_POWER: Self = Self(3);
    /// parametric set_luminances request; available since version 1.
    pub const SET_LUMINANCES: Self = Self(4);
    /// parametric set_mastering_display_primaries request; available since version 1.
    pub const SET_MASTERING_DISPLAY_PRIMARIES: Self = Self(5);
    /// parametric target exceeds primary color volume; available since version 1.
    pub const EXTENDED_TARGET_VOLUME: Self = Self(6);
    /// create_windows_scrgb request; available since version 1.
    pub const WINDOWS_SCRGB: Self = Self(7);
    /// create_windows_bt2100 request; available since version 1.
    pub const WINDOWS_BT2100: Self = Self(8);
}
impl CommandReader<Self> for WpColorManagerV1Feature {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpColorManagerV1Feature {
    fn object_id(&self) -> u32 {
        0
    }
    fn opcode(&self) -> u16 {
        0
    }
    fn payload_size(&self) -> usize {
        4
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        writer.next_u32(self.0).await
    }
}
