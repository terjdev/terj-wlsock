use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_color_representation_surface_v1.coefficients`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="coefficients">
///       <description summary="named coefficients">
///         Named matrix coefficients used to encode well-known sets of
///         coefficients. H.273 is the authority, when it comes to the exact values
///         of coefficients and authoritative specifications, where an equivalent
///         code point exists.
///
///         A value of 0 is invalid and will never be present in the list of enums.
///
///         Descriptions do list the specifications for convenience.
///       </description>
///       <entry name="identity" value="1">
///         <description summary="The identity matrix">
///           Coefficients as defined by
///           - IEC 61966-2-1 sRGB
///           - SMPTE ST 428-1 (2019)
///
///           Equivalent to H.273 MatrixCoefficients code point 0.
///           Compatible with pixel formats of the RGB family.
///         </description>
///       </entry>
///       <entry name="bt709" value="2">
///         <description summary="BT.709 matrix coefficients">
///           Coefficients as defined by
///           - Rec. ITU-R BT.709-6
///           - Rec. ITU-R BT.1361-0 conventional colour gamut system (historical)
///           - Rec. ITU-R BT.1361-0 conventional colour gamut system and extended
///             colour gamut system (historical)
///           - IEC 61966-2-4 xvYCC709
///           - SMPTE RP 177 (1993) Annex B
///
///           Equivalent to H.273 MatrixCoefficients code point 1.
///           Compatible with pixel formats of the YCbCr family.
///         </description>
///       </entry>
///       <entry name="fcc" value="3">
///         <description summary="FCC matrix coefficients">
///           Coefficients as defined by
///           - United States Federal Communications Commission (2003) Title 47
///             Code of Federal Regulations 73.682 (a) (20)
///
///           Equivalent to H.273 MatrixCoefficients code point 4.
///           Compatible with pixel formats of the YCbCr family.
///         </description>
///       </entry>
///       <entry name="bt601" value="4">
///         <description summary="BT.601-7 matrix coefficients">
///           Coefficients as defined by
///           - Rec. ITU-R BT.470-6 System B, G (historical)
///           - Rec. ITU-R BT.601-7 625
///           - Rec. ITU-R BT.601-7 525
///           - Rec. ITU-R BT.1358-0 625 (historical)
///           - Rec. ITU-R BT.1358-1 525 or 625 (historical)
///           - Rec. ITU-R BT.1700-0 625 PAL and 625 SECAM
///           - Rec. ITU-R BT.1700-0 NTSC
///           - IEC 61966-2-1 sYCC
///           - IEC 61966-2-4 xvYCC601
///           - SMPTE ST 170 (2004)
///
///           Equivalent to H.273 MatrixCoefficients code point 5, 6.
///           Compatible with pixel formats of the YCbCr family.
///         </description>
///       </entry>
///       <entry name="smpte240" value="5">
///         <description summary="SMPTE ST 240 matrix coefficients">
///           Coefficients as defined by
///           - SMPTE ST 240 (1999)
///
///           Equivalent to H.273 MatrixCoefficients code point 7.
///           Compatible with pixel formats of the YCbCr family.
///         </description>
///       </entry>
///       <entry name="bt2020" value="6">
///         <description summary="BT.2020 and BT.2100 YCbCr matrix coefficients">
///           Coefficients as defined by
///           - Rec. ITU-R BT.2020-2 (non-constant luminance)
///           - Rec. ITU-R BT.2100-2 Y′CbCr
///
///           Equivalent to H.273 MatrixCoefficients code point 9.
///           Compatible with pixel formats of the YCbCr family.
///         </description>
///       </entry>
///       <entry name="bt2020_cl" value="7">
///         <description summary="BT.2020 matrix coefficients for constant luminance">
///           Coefficients as defined by
///           - Rec. ITU-R BT.2020-2 (constant luminance)
///
///           Equivalent to H.273 MatrixCoefficients code point 10.
///           Compatible with pixel formats of the YCbCr family.
///         </description>
///       </entry>
///       <entry name="ictcp" value="8">
///         <description summary="BT.2100 ICtCp matrix coefficients">
///           Coefficients as defined by
///           - Rec. ITU-R BT.2100-2 ICTCP
///
///           Equivalent to H.273 MatrixCoefficients code point 14.
///           Compatible with pixel formats of the YCbCr family.
///         </description>
///       </entry>
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpColorRepresentationSurfaceV1Coefficients(pub u32);
impl WpColorRepresentationSurfaceV1Coefficients {
    /// The identity matrix; available since version 1.
    pub const IDENTITY: Self = Self(1);
    /// BT.709 matrix coefficients; available since version 1.
    pub const BT709: Self = Self(2);
    /// FCC matrix coefficients; available since version 1.
    pub const FCC: Self = Self(3);
    /// BT.601-7 matrix coefficients; available since version 1.
    pub const BT601: Self = Self(4);
    /// SMPTE ST 240 matrix coefficients; available since version 1.
    pub const SMPTE240: Self = Self(5);
    /// BT.2020 and BT.2100 YCbCr matrix coefficients; available since version 1.
    pub const BT2020: Self = Self(6);
    /// BT.2020 matrix coefficients for constant luminance; available since version 1.
    pub const BT2020_CL: Self = Self(7);
    /// BT.2100 ICtCp matrix coefficients; available since version 1.
    pub const ICTCP: Self = Self(8);
}
impl CommandReader<Self> for WpColorRepresentationSurfaceV1Coefficients {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpColorRepresentationSurfaceV1Coefficients {
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
