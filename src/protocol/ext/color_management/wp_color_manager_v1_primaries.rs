use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_color_manager_v1.primaries`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="primaries">
///       <description summary="named color primaries">
///         Named color primaries used to encode well-known sets of primaries.
///
///         A value of 0 is invalid and will never be present in the list of enums.
///       </description>
///
///       <entry name="srgb" value="1">
///         <description summary="Color primaries for the sRGB color space as defined by the BT.709 standard">
///           Color primaries as defined by
///           - Rec. ITU-R BT.709-6
///           - Rec. ITU-R BT.1361-0 conventional colour gamut system and extended
///             colour gamut system (historical)
///           - IEC 61966-2-1 sRGB or sYCC
///           - IEC 61966-2-4
///           - Society of Motion Picture and Television Engineers (SMPTE) RP 177
///             (1993) Annex B
///         </description>
///       </entry>
///       <entry name="pal_m" value="2">
///         <description summary="Color primaries for PAL-M as defined by the BT.470 standard">
///           Color primaries as defined by
///           - Rec. ITU-R BT.470-6 System M (historical)
///           - United States National Television System Committee 1953
///             Recommendation for transmission standards for color television
///           - United States Federal Communications Commission (2003) Title 47 Code
///             of Federal Regulations 73.682 (a)(20)
///         </description>
///       </entry>
///       <entry name="pal" value="3">
///         <description summary="Color primaries for PAL as defined by the BT.601 standard">
///           Color primaries as defined by
///           - Rec. ITU-R BT.470-6 System B, G (historical)
///           - Rec. ITU-R BT.601-7 625
///           - Rec. ITU-R BT.1358-0 625 (historical)
///           - Rec. ITU-R BT.1700-0 625 PAL and 625 SECAM
///         </description>
///       </entry>
///       <entry name="ntsc" value="4">
///         <description summary="Color primaries for NTSC as defined by the BT.601 standard">
///           Color primaries as defined by
///           - Rec. ITU-R BT.601-7 525
///           - Rec. ITU-R BT.1358-1 525 or 625 (historical)
///           - Rec. ITU-R BT.1700-0 NTSC
///           - SMPTE 170M (2004)
///           - SMPTE 240M (1999) (historical)
///         </description>
///       </entry>
///       <entry name="generic_film" value="5">
///         <description summary="Generic film with colour filters using Illuminant C">
///           Color primaries as defined by Recommendation ITU-T H.273
///           "Coding-independent code points for video signal type identification"
///           for "generic film".
///         </description>
///       </entry>
///       <entry name="bt2020" value="6">
///         <description summary="Color primaries as defined by the BT.2020 and BT.2100 standard">
///           Color primaries as defined by
///           - Rec. ITU-R BT.2020-2
///           - Rec. ITU-R BT.2100-0
///         </description>
///       </entry>
///       <entry name="cie1931_xyz" value="7">
///         <description summary="Color primaries of the full CIE 1931 XYZ color space">
///           Color primaries as defined as the maximum of the CIE 1931 XYZ color
///           space by
///           - SMPTE ST 428-1
///           - (CIE 1931 XYZ as in ISO 11664-1)
///         </description>
///       </entry>
///       <entry name="dci_p3" value="8">
///         <description summary="Color primaries of the DCI P3 color space as defined by the SMPTE RP 431 standard">
///           Color primaries as defined by Digital Cinema System and published in
///           SMPTE RP 431-2 (2011).
///         </description>
///       </entry>
///       <entry name="display_p3" value="9">
///         <description summary="Color primaries of Display P3 variant of the DCI-P3 color space as defined by the SMPTE EG 432 standard">
///           Color primaries as defined by Digital Cinema System and published in
///           SMPTE EG 432-1 (2010).
///         </description>
///       </entry>
///       <entry name="adobe_rgb" value="10">
///         <description summary="Color primaries of the Adobe RGB color space as defined by the ISO 12640 standard">
///           Color primaries as defined by Adobe as "Adobe RGB" and later published
///           by ISO 12640-4 (2011).
///         </description>
///       </entry>
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpColorManagerV1Primaries(pub u32);
impl WpColorManagerV1Primaries {
    /// Color primaries for the sRGB color space as defined by the BT.709 standard; available since version 1.
    pub const SRGB: Self = Self(1);
    /// Color primaries for PAL-M as defined by the BT.470 standard; available since version 1.
    pub const PAL_M: Self = Self(2);
    /// Color primaries for PAL as defined by the BT.601 standard; available since version 1.
    pub const PAL: Self = Self(3);
    /// Color primaries for NTSC as defined by the BT.601 standard; available since version 1.
    pub const NTSC: Self = Self(4);
    /// Generic film with colour filters using Illuminant C; available since version 1.
    pub const GENERIC_FILM: Self = Self(5);
    /// Color primaries as defined by the BT.2020 and BT.2100 standard; available since version 1.
    pub const BT2020: Self = Self(6);
    /// Color primaries of the full CIE 1931 XYZ color space; available since version 1.
    pub const CIE1931_XYZ: Self = Self(7);
    /// Color primaries of the DCI P3 color space as defined by the SMPTE RP 431 standard; available since version 1.
    pub const DCI_P3: Self = Self(8);
    /// Color primaries of Display P3 variant of the DCI-P3 color space as defined by the SMPTE EG 432 standard; available since version 1.
    pub const DISPLAY_P3: Self = Self(9);
    /// Color primaries of the Adobe RGB color space as defined by the ISO 12640 standard; available since version 1.
    pub const ADOBE_RGB: Self = Self(10);
}
impl CommandReader<Self> for WpColorManagerV1Primaries {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpColorManagerV1Primaries {
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
