use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_color_manager_v1.transfer_function`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="transfer_function">
///       <description summary="named transfer functions">
///         Named transfer functions used to represent well-known transfer
///         characteristics of displays.
///
///         A value of 0 is invalid and will never be present in the list of enums.
///
///         See appendix.md for the formulae.
///       </description>
///
///       <entry name="bt1886" value="1">
///         <description summary="BT.1886 display transfer characteristic">
///           Rec. ITU-R BT.1886 is the display transfer characteristic assumed by
///           - Rec. ITU-R BT.601-7 525 and 625
///           - Rec. ITU-R BT.709-6
///           - Rec. ITU-R BT.2020-2
///
///           This TF implies these default luminances from Rec. ITU-R BT.2035:
///           - primary color volume minimum: 0.01 cd/m²
///           - primary color volume maximum: 100 cd/m²
///           - reference white: 100 cd/m²
///         </description>
///       </entry>
///       <entry name="gamma22" value="2">
///         <description summary="Assumed display gamma 2.2 transfer function">
///           Transfer characteristics as defined by
///           - Rec. ITU-R BT.470-6 System M (historical)
///           - United States National Television System Committee 1953
///             Recommendation for transmission standards for color television
///           - United States Federal Communications Commission (2003) Title 47 Code
///             of Federal Regulations 73.682 (a) (20)
///           - Rec. ITU-R BT.1700-0 625 PAL and 625 SECAM
///           - IEC 61966-2-1 (reference display)
///         </description>
///       </entry>
///       <entry name="gamma28" value="3">
///         <description summary="Assumed display gamma 2.8 transfer function">
///           Transfer characteristics as defined by
///           - Rec. ITU-R BT.470-6 System B, G (historical)
///         </description>
///       </entry>
///       <entry name="st240" value="4">
///         <description summary="SMPTE ST 240 transfer function">
///           Transfer characteristics as defined by
///           - SMPTE ST 240 (1999)
///         </description>
///       </entry>
///       <entry name="ext_linear" value="5">
///         <description summary="extended linear transfer function">
///           Linear transfer function defined over all real numbers.
///           Normalised electrical values are equal the normalised optical values.
///         </description>
///       </entry>
///       <entry name="log_100" value="6">
///         <description summary="logarithmic 100:1 transfer function">
///           Logarithmic transfer characteristic (100:1 range).
///         </description>
///       </entry>
///       <entry name="log_316" value="7">
///         <description summary="logarithmic (100*Sqrt(10) : 1) transfer function">
///           Logarithmic transfer characteristic (100 * Sqrt(10) : 1 range).
///         </description>
///       </entry>
///       <entry name="xvycc" value="8">
///         <description summary="IEC 61966-2-4 transfer function">
///           Transfer characteristics as defined by
///           - IEC 61966-2-4
///         </description>
///       </entry>
///       <entry name="srgb" value="9" deprecated-since="2">
///         <description summary="Deprecated (ambiguous sRGB transfer function)">
///           Transfer characteristics as defined by
///           - IEC 61966-2-1 sRGB
///
///           As a rule of thumb, use gamma22 for video, motion picture and
///           computer graphics, or compound_power_2_4 for ICC calibrated print
///           workflows.
///         </description>
///       </entry>
///       <entry name="ext_srgb" value="10" deprecated-since="2">
///         <description summary="Deprecated (Extended sRGB piece-wise transfer function)">
///           Transfer characteristics as defined by
///           - IEC 61966-2-1 sYCC
///         </description>
///       </entry>
///       <entry name="st2084_pq" value="11">
///         <description summary="perceptual quantizer transfer function">
///           Transfer characteristics as defined by
///           - SMPTE ST 2084 (2014) for 10-, 12-, 14- and 16-bit systems
///           - Rec. ITU-R BT.2100-2 perceptual quantization (PQ) system
///
///           This TF implies these default luminances
///           - primary color volume minimum: 0.005 cd/m²
///           - primary color volume maximum: 10000 cd/m²
///           - reference white: 203 cd/m²
///
///           The difference between the primary color volume minimum and maximum
///           must be approximately 10000 cd/m² as that is the swing of the EOTF
///           defined by ST 2084 and BT.2100. The default value for the
///           reference white is a protocol addition: it is suggested by
///           Report ITU-R BT.2408-7 and is not part of ST 2084 or BT.2100.
///         </description>
///       </entry>
///       <entry name="st428" value="12">
///         <description summary="SMPTE ST 428 transfer function">
///           Transfer characteristics as defined by
///           - SMPTE ST 428-1 (2019)
///         </description>
///       </entry>
///       <entry name="hlg" value="13">
///         <description summary="hybrid log-gamma transfer function">
///           Transfer characteristics as defined by
///           - ARIB STD-B67 (2015)
///           - Rec. ITU-R BT.2100-2 hybrid log-gamma (HLG) system
///
///           This TF implies these default luminances
///           - primary color volume minimum: 0.005 cd/m²
///           - primary color volume maximum: 1000 cd/m²
///           - reference white: 203 cd/m²
///
///           HLG is a relative display-referred signal with a specified
///           non-linear mapping to the display peak luminance (the HLG OOTF).
///           All absolute luminance values used here for HLG assume a 1000 cd/m²
///           peak display.
///
///           The default value for the reference white is a protocol addition:
///           it is suggested by Report ITU-R BT.2408-7 and is not part of
///           ARIB STD-B67 or BT.2100.
///         </description>
///       </entry>
///       <entry name="compound_power_2_4" value="14" since="2">
///         <description summary="IEC 61966-2-1 encoding function">
///           Encoding characteristics as defined by IEC 61966-2-1, for displays
///           that invert the encoding function.
///         </description>
///       </entry>
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpColorManagerV1TransferFunction(pub u32);
impl WpColorManagerV1TransferFunction {
    /// BT.1886 display transfer characteristic; available since version 1.
    pub const BT1886: Self = Self(1);
    /// Assumed display gamma 2.2 transfer function; available since version 1.
    pub const GAMMA22: Self = Self(2);
    /// Assumed display gamma 2.8 transfer function; available since version 1.
    pub const GAMMA28: Self = Self(3);
    /// SMPTE ST 240 transfer function; available since version 1.
    pub const ST240: Self = Self(4);
    /// extended linear transfer function; available since version 1.
    pub const EXT_LINEAR: Self = Self(5);
    /// logarithmic 100:1 transfer function; available since version 1.
    pub const LOG_100: Self = Self(6);
    /// logarithmic (100*Sqrt(10) : 1) transfer function; available since version 1.
    pub const LOG_316: Self = Self(7);
    /// IEC 61966-2-4 transfer function; available since version 1.
    pub const XVYCC: Self = Self(8);
    /// Deprecated (ambiguous sRGB transfer function); available since version 1.
    pub const SRGB: Self = Self(9);
    /// Deprecated (Extended sRGB piece-wise transfer function); available since version 1.
    pub const EXT_SRGB: Self = Self(10);
    /// perceptual quantizer transfer function; available since version 1.
    pub const ST2084_PQ: Self = Self(11);
    /// SMPTE ST 428 transfer function; available since version 1.
    pub const ST428: Self = Self(12);
    /// hybrid log-gamma transfer function; available since version 1.
    pub const HLG: Self = Self(13);
    /// IEC 61966-2-1 encoding function; available since version 2.
    pub const COMPOUND_POWER_2_4: Self = Self(14);
}
impl CommandReader<Self> for WpColorManagerV1TransferFunction {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpColorManagerV1TransferFunction {
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
