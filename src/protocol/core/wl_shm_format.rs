use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_shm.format`.
///
/// The numeric representation preserves unknown values without losing information.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="format">
///       <description summary="pixel formats">
///         This describes the memory layout of an individual pixel.
///
///         All renderers should support argb8888 and xrgb8888 but any other
///         formats are optional and may not be supported by the particular
///         renderer in use.
///
///         The drm format codes match the macros defined in drm_fourcc.h, except
///         argb8888 and xrgb8888. The formats actually supported by the compositor
///         will be reported by the format event. See drm_fourcc.h for more detailed
///         format descriptions.
///
///         For all wl_shm formats and unless specified in another protocol
///         extension, pre-multiplied alpha is used for pixel values.
///       </description>
///       
///       <entry name="argb8888" value="0" summary="32-bit ARGB format, [31:0] A:R:G:B 8:8:8:8 little endian" />
///       <entry name="xrgb8888" value="1" summary="32-bit RGB format, [31:0] x:R:G:B 8:8:8:8 little endian" />
///       <entry name="c8" value="0x20203843" summary="8-bit color index format, [7:0] C" />
///       <entry name="rgb332" value="0x38424752" summary="8-bit RGB format, [7:0] R:G:B 3:3:2" />
///       <entry name="bgr233" value="0x38524742" summary="8-bit BGR format, [7:0] B:G:R 2:3:3" />
///       <entry name="xrgb4444" value="0x32315258" summary="16-bit xRGB format, [15:0] x:R:G:B 4:4:4:4 little endian" />
///       <entry name="xbgr4444" value="0x32314258" summary="16-bit xBGR format, [15:0] x:B:G:R 4:4:4:4 little endian" />
///       <entry name="rgbx4444" value="0x32315852" summary="16-bit RGBx format, [15:0] R:G:B:x 4:4:4:4 little endian" />
///       <entry name="bgrx4444" value="0x32315842" summary="16-bit BGRx format, [15:0] B:G:R:x 4:4:4:4 little endian" />
///       <entry name="argb4444" value="0x32315241" summary="16-bit ARGB format, [15:0] A:R:G:B 4:4:4:4 little endian" />
///       <entry name="abgr4444" value="0x32314241" summary="16-bit ABGR format, [15:0] A:B:G:R 4:4:4:4 little endian" />
///       <entry name="rgba4444" value="0x32314152" summary="16-bit RGBA format, [15:0] R:G:B:A 4:4:4:4 little endian" />
///       <entry name="bgra4444" value="0x32314142" summary="16-bit BGRA format, [15:0] B:G:R:A 4:4:4:4 little endian" />
///       <entry name="xrgb1555" value="0x35315258" summary="16-bit xRGB format, [15:0] x:R:G:B 1:5:5:5 little endian" />
///       <entry name="xbgr1555" value="0x35314258" summary="16-bit xBGR 1555 format, [15:0] x:B:G:R 1:5:5:5 little endian" />
///       <entry name="rgbx5551" value="0x35315852" summary="16-bit RGBx 5551 format, [15:0] R:G:B:x 5:5:5:1 little endian" />
///       <entry name="bgrx5551" value="0x35315842" summary="16-bit BGRx 5551 format, [15:0] B:G:R:x 5:5:5:1 little endian" />
///       <entry name="argb1555" value="0x35315241" summary="16-bit ARGB 1555 format, [15:0] A:R:G:B 1:5:5:5 little endian" />
///       <entry name="abgr1555" value="0x35314241" summary="16-bit ABGR 1555 format, [15:0] A:B:G:R 1:5:5:5 little endian" />
///       <entry name="rgba5551" value="0x35314152" summary="16-bit RGBA 5551 format, [15:0] R:G:B:A 5:5:5:1 little endian" />
///       <entry name="bgra5551" value="0x35314142" summary="16-bit BGRA 5551 format, [15:0] B:G:R:A 5:5:5:1 little endian" />
///       <entry name="rgb565" value="0x36314752" summary="16-bit RGB 565 format, [15:0] R:G:B 5:6:5 little endian" />
///       <entry name="bgr565" value="0x36314742" summary="16-bit BGR 565 format, [15:0] B:G:R 5:6:5 little endian" />
///       <entry name="rgb888" value="0x34324752" summary="24-bit RGB format, [23:0] R:G:B little endian" />
///       <entry name="bgr888" value="0x34324742" summary="24-bit BGR format, [23:0] B:G:R little endian" />
///       <entry name="xbgr8888" value="0x34324258" summary="32-bit xBGR format, [31:0] x:B:G:R 8:8:8:8 little endian" />
///       <entry name="rgbx8888" value="0x34325852" summary="32-bit RGBx format, [31:0] R:G:B:x 8:8:8:8 little endian" />
///       <entry name="bgrx8888" value="0x34325842" summary="32-bit BGRx format, [31:0] B:G:R:x 8:8:8:8 little endian" />
///       <entry name="abgr8888" value="0x34324241" summary="32-bit ABGR format, [31:0] A:B:G:R 8:8:8:8 little endian" />
///       <entry name="rgba8888" value="0x34324152" summary="32-bit RGBA format, [31:0] R:G:B:A 8:8:8:8 little endian" />
///       <entry name="bgra8888" value="0x34324142" summary="32-bit BGRA format, [31:0] B:G:R:A 8:8:8:8 little endian" />
///       <entry name="xrgb2101010" value="0x30335258" summary="32-bit xRGB format, [31:0] x:R:G:B 2:10:10:10 little endian" />
///       <entry name="xbgr2101010" value="0x30334258" summary="32-bit xBGR format, [31:0] x:B:G:R 2:10:10:10 little endian" />
///       <entry name="rgbx1010102" value="0x30335852" summary="32-bit RGBx format, [31:0] R:G:B:x 10:10:10:2 little endian" />
///       <entry name="bgrx1010102" value="0x30335842" summary="32-bit BGRx format, [31:0] B:G:R:x 10:10:10:2 little endian" />
///       <entry name="argb2101010" value="0x30335241" summary="32-bit ARGB format, [31:0] A:R:G:B 2:10:10:10 little endian" />
///       <entry name="abgr2101010" value="0x30334241" summary="32-bit ABGR format, [31:0] A:B:G:R 2:10:10:10 little endian" />
///       <entry name="rgba1010102" value="0x30334152" summary="32-bit RGBA format, [31:0] R:G:B:A 10:10:10:2 little endian" />
///       <entry name="bgra1010102" value="0x30334142" summary="32-bit BGRA format, [31:0] B:G:R:A 10:10:10:2 little endian" />
///       <entry name="yuyv" value="0x56595559" summary="packed YCbCr format, [31:0] Cr0:Y1:Cb0:Y0 8:8:8:8 little endian" />
///       <entry name="yvyu" value="0x55595659" summary="packed YCbCr format, [31:0] Cb0:Y1:Cr0:Y0 8:8:8:8 little endian" />
///       <entry name="uyvy" value="0x59565955" summary="packed YCbCr format, [31:0] Y1:Cr0:Y0:Cb0 8:8:8:8 little endian" />
///       <entry name="vyuy" value="0x59555956" summary="packed YCbCr format, [31:0] Y1:Cb0:Y0:Cr0 8:8:8:8 little endian" />
///       <entry name="ayuv" value="0x56555941" summary="packed AYCbCr format, [31:0] A:Y:Cb:Cr 8:8:8:8 little endian" />
///       <entry name="nv12" value="0x3231564e" summary="2 plane YCbCr Cr:Cb format, 2x2 subsampled Cr:Cb plane" />
///       <entry name="nv21" value="0x3132564e" summary="2 plane YCbCr Cb:Cr format, 2x2 subsampled Cb:Cr plane" />
///       <entry name="nv16" value="0x3631564e" summary="2 plane YCbCr Cr:Cb format, 2x1 subsampled Cr:Cb plane" />
///       <entry name="nv61" value="0x3136564e" summary="2 plane YCbCr Cb:Cr format, 2x1 subsampled Cb:Cr plane" />
///       <entry name="yuv410" value="0x39565559" summary="3 plane YCbCr format, 4x4 subsampled Cb (1) and Cr (2) planes" />
///       <entry name="yvu410" value="0x39555659" summary="3 plane YCbCr format, 4x4 subsampled Cr (1) and Cb (2) planes" />
///       <entry name="yuv411" value="0x31315559" summary="3 plane YCbCr format, 4x1 subsampled Cb (1) and Cr (2) planes" />
///       <entry name="yvu411" value="0x31315659" summary="3 plane YCbCr format, 4x1 subsampled Cr (1) and Cb (2) planes" />
///       <entry name="yuv420" value="0x32315559" summary="3 plane YCbCr format, 2x2 subsampled Cb (1) and Cr (2) planes" />
///       <entry name="yvu420" value="0x32315659" summary="3 plane YCbCr format, 2x2 subsampled Cr (1) and Cb (2) planes" />
///       <entry name="yuv422" value="0x36315559" summary="3 plane YCbCr format, 2x1 subsampled Cb (1) and Cr (2) planes" />
///       <entry name="yvu422" value="0x36315659" summary="3 plane YCbCr format, 2x1 subsampled Cr (1) and Cb (2) planes" />
///       <entry name="yuv444" value="0x34325559" summary="3 plane YCbCr format, non-subsampled Cb (1) and Cr (2) planes" />
///       <entry name="yvu444" value="0x34325659" summary="3 plane YCbCr format, non-subsampled Cr (1) and Cb (2) planes" />
///       <entry name="r8" value="0x20203852" summary="[7:0] R" />
///       <entry name="r16" value="0x20363152" summary="[15:0] R little endian" />
///       <entry name="rg88" value="0x38384752" summary="[15:0] R:G 8:8 little endian" />
///       <entry name="gr88" value="0x38385247" summary="[15:0] G:R 8:8 little endian" />
///       <entry name="rg1616" value="0x32334752" summary="[31:0] R:G 16:16 little endian" />
///       <entry name="gr1616" value="0x32335247" summary="[31:0] G:R 16:16 little endian" />
///       <entry name="xrgb16161616f" value="0x48345258" summary="[63:0] x:R:G:B 16:16:16:16 little endian" />
///       <entry name="xbgr16161616f" value="0x48344258" summary="[63:0] x:B:G:R 16:16:16:16 little endian" />
///       <entry name="argb16161616f" value="0x48345241" summary="[63:0] A:R:G:B 16:16:16:16 little endian" />
///       <entry name="abgr16161616f" value="0x48344241" summary="[63:0] A:B:G:R 16:16:16:16 little endian" />
///       <entry name="xyuv8888" value="0x56555958" summary="[31:0] X:Y:Cb:Cr 8:8:8:8 little endian" />
///       <entry name="vuy888" value="0x34325556" summary="[23:0] Cr:Cb:Y 8:8:8 little endian" />
///       <entry name="vuy101010" value="0x30335556" summary="Y followed by U then V, 10:10:10. Non-linear modifier only" />
///       <entry name="y210" value="0x30313259" summary="[63:0] Cr0:0:Y1:0:Cb0:0:Y0:0 10:6:10:6:10:6:10:6 little endian per 2 Y pixels" />
///       <entry name="y212" value="0x32313259" summary="[63:0] Cr0:0:Y1:0:Cb0:0:Y0:0 12:4:12:4:12:4:12:4 little endian per 2 Y pixels" />
///       <entry name="y216" value="0x36313259" summary="[63:0] Cr0:Y1:Cb0:Y0 16:16:16:16 little endian per 2 Y pixels" />
///       <entry name="y410" value="0x30313459" summary="[31:0] A:Cr:Y:Cb 2:10:10:10 little endian" />
///       <entry name="y412" value="0x32313459" summary="[63:0] A:0:Cr:0:Y:0:Cb:0 12:4:12:4:12:4:12:4 little endian" />
///       <entry name="y416" value="0x36313459" summary="[63:0] A:Cr:Y:Cb 16:16:16:16 little endian" />
///       <entry name="xvyu2101010" value="0x30335658" summary="[31:0] X:Cr:Y:Cb 2:10:10:10 little endian" />
///       <entry name="xvyu12_16161616" value="0x36335658" summary="[63:0] X:0:Cr:0:Y:0:Cb:0 12:4:12:4:12:4:12:4 little endian" />
///       <entry name="xvyu16161616" value="0x38345658" summary="[63:0] X:Cr:Y:Cb 16:16:16:16 little endian" />
///       <entry name="y0l0" value="0x304c3059" summary="[63:0]   A3:A2:Y3:0:Cr0:0:Y2:0:A1:A0:Y1:0:Cb0:0:Y0:0  1:1:8:2:8:2:8:2:1:1:8:2:8:2:8:2 little endian" />
///       <entry name="x0l0" value="0x304c3058" summary="[63:0]   X3:X2:Y3:0:Cr0:0:Y2:0:X1:X0:Y1:0:Cb0:0:Y0:0  1:1:8:2:8:2:8:2:1:1:8:2:8:2:8:2 little endian" />
///       <entry name="y0l2" value="0x324c3059" summary="[63:0]   A3:A2:Y3:Cr0:Y2:A1:A0:Y1:Cb0:Y0  1:1:10:10:10:1:1:10:10:10 little endian" />
///       <entry name="x0l2" value="0x324c3058" summary="[63:0]   X3:X2:Y3:Cr0:Y2:X1:X0:Y1:Cb0:Y0  1:1:10:10:10:1:1:10:10:10 little endian" />
///       <entry name="yuv420_8bit" value="0x38305559" />
///       <entry name="yuv420_10bit" value="0x30315559" />
///       <entry name="xrgb8888_a8" value="0x38415258" />
///       <entry name="xbgr8888_a8" value="0x38414258" />
///       <entry name="rgbx8888_a8" value="0x38415852" />
///       <entry name="bgrx8888_a8" value="0x38415842" />
///       <entry name="rgb888_a8" value="0x38413852" />
///       <entry name="bgr888_a8" value="0x38413842" />
///       <entry name="rgb565_a8" value="0x38413552" />
///       <entry name="bgr565_a8" value="0x38413542" />
///       <entry name="nv24" value="0x3432564e" summary="non-subsampled Cr:Cb plane" />
///       <entry name="nv42" value="0x3234564e" summary="non-subsampled Cb:Cr plane" />
///       <entry name="p210" value="0x30313250" summary="2x1 subsampled Cr:Cb plane, 10 bit per channel" />
///       <entry name="p010" value="0x30313050" summary="2x2 subsampled Cr:Cb plane 10 bits per channel" />
///       <entry name="p012" value="0x32313050" summary="2x2 subsampled Cr:Cb plane 12 bits per channel" />
///       <entry name="p016" value="0x36313050" summary="2x2 subsampled Cr:Cb plane 16 bits per channel" />
///       <entry name="axbxgxrx106106106106" value="0x30314241" summary="[63:0] A:x:B:x:G:x:R:x 10:6:10:6:10:6:10:6 little endian" />
///       <entry name="nv15" value="0x3531564e" summary="2x2 subsampled Cr:Cb plane" />
///       <entry name="q410" value="0x30313451" />
///       <entry name="q401" value="0x31303451" />
///       <entry name="xrgb16161616" value="0x38345258" summary="[63:0] x:R:G:B 16:16:16:16 little endian" />
///       <entry name="xbgr16161616" value="0x38344258" summary="[63:0] x:B:G:R 16:16:16:16 little endian" />
///       <entry name="argb16161616" value="0x38345241" summary="[63:0] A:R:G:B 16:16:16:16 little endian" />
///       <entry name="abgr16161616" value="0x38344241" summary="[63:0] A:B:G:R 16:16:16:16 little endian" />
///       <entry name="c1" value="0x20203143" summary="[7:0] C0:C1:C2:C3:C4:C5:C6:C7 1:1:1:1:1:1:1:1 eight pixels/byte" />
///       <entry name="c2" value="0x20203243" summary="[7:0] C0:C1:C2:C3 2:2:2:2 four pixels/byte" />
///       <entry name="c4" value="0x20203443" summary="[7:0] C0:C1 4:4 two pixels/byte" />
///       <entry name="d1" value="0x20203144" summary="[7:0] D0:D1:D2:D3:D4:D5:D6:D7 1:1:1:1:1:1:1:1 eight pixels/byte" />
///       <entry name="d2" value="0x20203244" summary="[7:0] D0:D1:D2:D3 2:2:2:2 four pixels/byte" />
///       <entry name="d4" value="0x20203444" summary="[7:0] D0:D1 4:4 two pixels/byte" />
///       <entry name="d8" value="0x20203844" summary="[7:0] D" />
///       <entry name="r1" value="0x20203152" summary="[7:0] R0:R1:R2:R3:R4:R5:R6:R7 1:1:1:1:1:1:1:1 eight pixels/byte" />
///       <entry name="r2" value="0x20203252" summary="[7:0] R0:R1:R2:R3 2:2:2:2 four pixels/byte" />
///       <entry name="r4" value="0x20203452" summary="[7:0] R0:R1 4:4 two pixels/byte" />
///       <entry name="r10" value="0x20303152" summary="[15:0] x:R 6:10 little endian" />
///       <entry name="r12" value="0x20323152" summary="[15:0] x:R 4:12 little endian" />
///       <entry name="avuy8888" value="0x59555641" summary="[31:0] A:Cr:Cb:Y 8:8:8:8 little endian" />
///       <entry name="xvuy8888" value="0x59555658" summary="[31:0] X:Cr:Cb:Y 8:8:8:8 little endian" />
///       <entry name="p030" value="0x30333050" summary="2x2 subsampled Cr:Cb plane 10 bits per channel packed" />
///       <entry name="rgb161616" value="0x38344752" summary="[47:0] R:G:B 16:16:16 little endian" />
///       <entry name="bgr161616" value="0x38344742" summary="[47:0] B:G:R 16:16:16 little endian" />
///       <entry name="r16f" value="0x48202052" summary="[15:0] R 16 little endian" />
///       <entry name="gr1616f" value="0x48205247" summary="[31:0] G:R 16:16 little endian" />
///       <entry name="bgr161616f" value="0x48524742" summary="[47:0] B:G:R 16:16:16 little endian" />
///       <entry name="r32f" value="0x46202052" summary="[31:0] R 32 little endian" />
///       <entry name="gr3232f" value="0x46205247" summary="[63:0] G:R 32:32 little endian" />
///       <entry name="bgr323232f" value="0x46524742" summary="[95:0] B:G:R 32:32:32 little endian" />
///       <entry name="abgr32323232f" value="0x46384241" summary="[127:0] A:B:G:R 32:32:32:32 little endian" />
///       <entry name="nv20" value="0x3032564e" summary="2x1 subsampled Cr:Cb plane" />
///       <entry name="nv30" value="0x3033564e" summary="non-subsampled Cr:Cb plane" />
///       <entry name="s010" value="0x30313053" summary="2x2 subsampled Cb (1) and Cr (2) planes 10 bits per channel" />
///       <entry name="s210" value="0x30313253" summary="2x1 subsampled Cb (1) and Cr (2) planes 10 bits per channel" />
///       <entry name="s410" value="0x30313453" summary="non-subsampled Cb (1) and Cr (2) planes 10 bits per channel" />
///       <entry name="s012" value="0x32313053" summary="2x2 subsampled Cb (1) and Cr (2) planes 12 bits per channel" />
///       <entry name="s212" value="0x32313253" summary="2x1 subsampled Cb (1) and Cr (2) planes 12 bits per channel" />
///       <entry name="s412" value="0x32313453" summary="non-subsampled Cb (1) and Cr (2) planes 12 bits per channel" />
///       <entry name="s016" value="0x36313053" summary="2x2 subsampled Cb (1) and Cr (2) planes 16 bits per channel" />
///       <entry name="s216" value="0x36313253" summary="2x1 subsampled Cb (1) and Cr (2) planes 16 bits per channel" />
///       <entry name="s416" value="0x36313453" summary="non-subsampled Cb (1) and Cr (2) planes 16 bits per channel" />
///       <entry name="xvuy2101010" value="0x30335958" summary="[31:0] x:Cr:Cb:Y 2:10:10:10 little endian" />
///       <entry name="p230" value="0x30333250" summary="2x1 subsampled Cr:Cb plane 10 bits per channel packed" />
///       <entry name="t430" value="0x30333454" />
///       <entry name="y8" value="0x59455247" summary="8-bit Y-only" />
///       <entry name="xyyy2101010" value="0x34415059" summary="[31:0] x:Y2:Y1:Y0 2:10:10:10 little endian" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlShmFormat(pub u32);
impl WlShmFormat {
    /// 32-bit ARGB format, \[31:0\] A:R:G:B 8:8:8:8 little endian; XML since=1.
    /// ```xml
    /// <entry name="argb8888" value="0" summary="32-bit ARGB format, [31:0] A:R:G:B 8:8:8:8 little endian" />
    /// ```
    pub const ARGB8888: Self = Self(0);
    /// 32-bit RGB format, \[31:0\] x:R:G:B 8:8:8:8 little endian; XML since=1.
    /// ```xml
    /// <entry name="xrgb8888" value="1" summary="32-bit RGB format, [31:0] x:R:G:B 8:8:8:8 little endian" />
    /// ```
    pub const XRGB8888: Self = Self(1);
    /// 8-bit color index format, \[7:0\] C; XML since=1.
    /// ```xml
    /// <entry name="c8" value="0x20203843" summary="8-bit color index format, [7:0] C" />
    /// ```
    pub const C8: Self = Self(0x20203843);
    /// 8-bit RGB format, \[7:0\] R:G:B 3:3:2; XML since=1.
    /// ```xml
    /// <entry name="rgb332" value="0x38424752" summary="8-bit RGB format, [7:0] R:G:B 3:3:2" />
    /// ```
    pub const RGB332: Self = Self(0x38424752);
    /// 8-bit BGR format, \[7:0\] B:G:R 2:3:3; XML since=1.
    /// ```xml
    /// <entry name="bgr233" value="0x38524742" summary="8-bit BGR format, [7:0] B:G:R 2:3:3" />
    /// ```
    pub const BGR233: Self = Self(0x38524742);
    /// 16-bit xRGB format, \[15:0\] x:R:G:B 4:4:4:4 little endian; XML since=1.
    /// ```xml
    /// <entry name="xrgb4444" value="0x32315258" summary="16-bit xRGB format, [15:0] x:R:G:B 4:4:4:4 little endian" />
    /// ```
    pub const XRGB4444: Self = Self(0x32315258);
    /// 16-bit xBGR format, \[15:0\] x:B:G:R 4:4:4:4 little endian; XML since=1.
    /// ```xml
    /// <entry name="xbgr4444" value="0x32314258" summary="16-bit xBGR format, [15:0] x:B:G:R 4:4:4:4 little endian" />
    /// ```
    pub const XBGR4444: Self = Self(0x32314258);
    /// 16-bit RGBx format, \[15:0\] R:G:B:x 4:4:4:4 little endian; XML since=1.
    /// ```xml
    /// <entry name="rgbx4444" value="0x32315852" summary="16-bit RGBx format, [15:0] R:G:B:x 4:4:4:4 little endian" />
    /// ```
    pub const RGBX4444: Self = Self(0x32315852);
    /// 16-bit BGRx format, \[15:0\] B:G:R:x 4:4:4:4 little endian; XML since=1.
    /// ```xml
    /// <entry name="bgrx4444" value="0x32315842" summary="16-bit BGRx format, [15:0] B:G:R:x 4:4:4:4 little endian" />
    /// ```
    pub const BGRX4444: Self = Self(0x32315842);
    /// 16-bit ARGB format, \[15:0\] A:R:G:B 4:4:4:4 little endian; XML since=1.
    /// ```xml
    /// <entry name="argb4444" value="0x32315241" summary="16-bit ARGB format, [15:0] A:R:G:B 4:4:4:4 little endian" />
    /// ```
    pub const ARGB4444: Self = Self(0x32315241);
    /// 16-bit ABGR format, \[15:0\] A:B:G:R 4:4:4:4 little endian; XML since=1.
    /// ```xml
    /// <entry name="abgr4444" value="0x32314241" summary="16-bit ABGR format, [15:0] A:B:G:R 4:4:4:4 little endian" />
    /// ```
    pub const ABGR4444: Self = Self(0x32314241);
    /// 16-bit RGBA format, \[15:0\] R:G:B:A 4:4:4:4 little endian; XML since=1.
    /// ```xml
    /// <entry name="rgba4444" value="0x32314152" summary="16-bit RGBA format, [15:0] R:G:B:A 4:4:4:4 little endian" />
    /// ```
    pub const RGBA4444: Self = Self(0x32314152);
    /// 16-bit BGRA format, \[15:0\] B:G:R:A 4:4:4:4 little endian; XML since=1.
    /// ```xml
    /// <entry name="bgra4444" value="0x32314142" summary="16-bit BGRA format, [15:0] B:G:R:A 4:4:4:4 little endian" />
    /// ```
    pub const BGRA4444: Self = Self(0x32314142);
    /// 16-bit xRGB format, \[15:0\] x:R:G:B 1:5:5:5 little endian; XML since=1.
    /// ```xml
    /// <entry name="xrgb1555" value="0x35315258" summary="16-bit xRGB format, [15:0] x:R:G:B 1:5:5:5 little endian" />
    /// ```
    pub const XRGB1555: Self = Self(0x35315258);
    /// 16-bit xBGR 1555 format, \[15:0\] x:B:G:R 1:5:5:5 little endian; XML since=1.
    /// ```xml
    /// <entry name="xbgr1555" value="0x35314258" summary="16-bit xBGR 1555 format, [15:0] x:B:G:R 1:5:5:5 little endian" />
    /// ```
    pub const XBGR1555: Self = Self(0x35314258);
    /// 16-bit RGBx 5551 format, \[15:0\] R:G:B:x 5:5:5:1 little endian; XML since=1.
    /// ```xml
    /// <entry name="rgbx5551" value="0x35315852" summary="16-bit RGBx 5551 format, [15:0] R:G:B:x 5:5:5:1 little endian" />
    /// ```
    pub const RGBX5551: Self = Self(0x35315852);
    /// 16-bit BGRx 5551 format, \[15:0\] B:G:R:x 5:5:5:1 little endian; XML since=1.
    /// ```xml
    /// <entry name="bgrx5551" value="0x35315842" summary="16-bit BGRx 5551 format, [15:0] B:G:R:x 5:5:5:1 little endian" />
    /// ```
    pub const BGRX5551: Self = Self(0x35315842);
    /// 16-bit ARGB 1555 format, \[15:0\] A:R:G:B 1:5:5:5 little endian; XML since=1.
    /// ```xml
    /// <entry name="argb1555" value="0x35315241" summary="16-bit ARGB 1555 format, [15:0] A:R:G:B 1:5:5:5 little endian" />
    /// ```
    pub const ARGB1555: Self = Self(0x35315241);
    /// 16-bit ABGR 1555 format, \[15:0\] A:B:G:R 1:5:5:5 little endian; XML since=1.
    /// ```xml
    /// <entry name="abgr1555" value="0x35314241" summary="16-bit ABGR 1555 format, [15:0] A:B:G:R 1:5:5:5 little endian" />
    /// ```
    pub const ABGR1555: Self = Self(0x35314241);
    /// 16-bit RGBA 5551 format, \[15:0\] R:G:B:A 5:5:5:1 little endian; XML since=1.
    /// ```xml
    /// <entry name="rgba5551" value="0x35314152" summary="16-bit RGBA 5551 format, [15:0] R:G:B:A 5:5:5:1 little endian" />
    /// ```
    pub const RGBA5551: Self = Self(0x35314152);
    /// 16-bit BGRA 5551 format, \[15:0\] B:G:R:A 5:5:5:1 little endian; XML since=1.
    /// ```xml
    /// <entry name="bgra5551" value="0x35314142" summary="16-bit BGRA 5551 format, [15:0] B:G:R:A 5:5:5:1 little endian" />
    /// ```
    pub const BGRA5551: Self = Self(0x35314142);
    /// 16-bit RGB 565 format, \[15:0\] R:G:B 5:6:5 little endian; XML since=1.
    /// ```xml
    /// <entry name="rgb565" value="0x36314752" summary="16-bit RGB 565 format, [15:0] R:G:B 5:6:5 little endian" />
    /// ```
    pub const RGB565: Self = Self(0x36314752);
    /// 16-bit BGR 565 format, \[15:0\] B:G:R 5:6:5 little endian; XML since=1.
    /// ```xml
    /// <entry name="bgr565" value="0x36314742" summary="16-bit BGR 565 format, [15:0] B:G:R 5:6:5 little endian" />
    /// ```
    pub const BGR565: Self = Self(0x36314742);
    /// 24-bit RGB format, \[23:0\] R:G:B little endian; XML since=1.
    /// ```xml
    /// <entry name="rgb888" value="0x34324752" summary="24-bit RGB format, [23:0] R:G:B little endian" />
    /// ```
    pub const RGB888: Self = Self(0x34324752);
    /// 24-bit BGR format, \[23:0\] B:G:R little endian; XML since=1.
    /// ```xml
    /// <entry name="bgr888" value="0x34324742" summary="24-bit BGR format, [23:0] B:G:R little endian" />
    /// ```
    pub const BGR888: Self = Self(0x34324742);
    /// 32-bit xBGR format, \[31:0\] x:B:G:R 8:8:8:8 little endian; XML since=1.
    /// ```xml
    /// <entry name="xbgr8888" value="0x34324258" summary="32-bit xBGR format, [31:0] x:B:G:R 8:8:8:8 little endian" />
    /// ```
    pub const XBGR8888: Self = Self(0x34324258);
    /// 32-bit RGBx format, \[31:0\] R:G:B:x 8:8:8:8 little endian; XML since=1.
    /// ```xml
    /// <entry name="rgbx8888" value="0x34325852" summary="32-bit RGBx format, [31:0] R:G:B:x 8:8:8:8 little endian" />
    /// ```
    pub const RGBX8888: Self = Self(0x34325852);
    /// 32-bit BGRx format, \[31:0\] B:G:R:x 8:8:8:8 little endian; XML since=1.
    /// ```xml
    /// <entry name="bgrx8888" value="0x34325842" summary="32-bit BGRx format, [31:0] B:G:R:x 8:8:8:8 little endian" />
    /// ```
    pub const BGRX8888: Self = Self(0x34325842);
    /// 32-bit ABGR format, \[31:0\] A:B:G:R 8:8:8:8 little endian; XML since=1.
    /// ```xml
    /// <entry name="abgr8888" value="0x34324241" summary="32-bit ABGR format, [31:0] A:B:G:R 8:8:8:8 little endian" />
    /// ```
    pub const ABGR8888: Self = Self(0x34324241);
    /// 32-bit RGBA format, \[31:0\] R:G:B:A 8:8:8:8 little endian; XML since=1.
    /// ```xml
    /// <entry name="rgba8888" value="0x34324152" summary="32-bit RGBA format, [31:0] R:G:B:A 8:8:8:8 little endian" />
    /// ```
    pub const RGBA8888: Self = Self(0x34324152);
    /// 32-bit BGRA format, \[31:0\] B:G:R:A 8:8:8:8 little endian; XML since=1.
    /// ```xml
    /// <entry name="bgra8888" value="0x34324142" summary="32-bit BGRA format, [31:0] B:G:R:A 8:8:8:8 little endian" />
    /// ```
    pub const BGRA8888: Self = Self(0x34324142);
    /// 32-bit xRGB format, \[31:0\] x:R:G:B 2:10:10:10 little endian; XML since=1.
    /// ```xml
    /// <entry name="xrgb2101010" value="0x30335258" summary="32-bit xRGB format, [31:0] x:R:G:B 2:10:10:10 little endian" />
    /// ```
    pub const XRGB2101010: Self = Self(0x30335258);
    /// 32-bit xBGR format, \[31:0\] x:B:G:R 2:10:10:10 little endian; XML since=1.
    /// ```xml
    /// <entry name="xbgr2101010" value="0x30334258" summary="32-bit xBGR format, [31:0] x:B:G:R 2:10:10:10 little endian" />
    /// ```
    pub const XBGR2101010: Self = Self(0x30334258);
    /// 32-bit RGBx format, \[31:0\] R:G:B:x 10:10:10:2 little endian; XML since=1.
    /// ```xml
    /// <entry name="rgbx1010102" value="0x30335852" summary="32-bit RGBx format, [31:0] R:G:B:x 10:10:10:2 little endian" />
    /// ```
    pub const RGBX1010102: Self = Self(0x30335852);
    /// 32-bit BGRx format, \[31:0\] B:G:R:x 10:10:10:2 little endian; XML since=1.
    /// ```xml
    /// <entry name="bgrx1010102" value="0x30335842" summary="32-bit BGRx format, [31:0] B:G:R:x 10:10:10:2 little endian" />
    /// ```
    pub const BGRX1010102: Self = Self(0x30335842);
    /// 32-bit ARGB format, \[31:0\] A:R:G:B 2:10:10:10 little endian; XML since=1.
    /// ```xml
    /// <entry name="argb2101010" value="0x30335241" summary="32-bit ARGB format, [31:0] A:R:G:B 2:10:10:10 little endian" />
    /// ```
    pub const ARGB2101010: Self = Self(0x30335241);
    /// 32-bit ABGR format, \[31:0\] A:B:G:R 2:10:10:10 little endian; XML since=1.
    /// ```xml
    /// <entry name="abgr2101010" value="0x30334241" summary="32-bit ABGR format, [31:0] A:B:G:R 2:10:10:10 little endian" />
    /// ```
    pub const ABGR2101010: Self = Self(0x30334241);
    /// 32-bit RGBA format, \[31:0\] R:G:B:A 10:10:10:2 little endian; XML since=1.
    /// ```xml
    /// <entry name="rgba1010102" value="0x30334152" summary="32-bit RGBA format, [31:0] R:G:B:A 10:10:10:2 little endian" />
    /// ```
    pub const RGBA1010102: Self = Self(0x30334152);
    /// 32-bit BGRA format, \[31:0\] B:G:R:A 10:10:10:2 little endian; XML since=1.
    /// ```xml
    /// <entry name="bgra1010102" value="0x30334142" summary="32-bit BGRA format, [31:0] B:G:R:A 10:10:10:2 little endian" />
    /// ```
    pub const BGRA1010102: Self = Self(0x30334142);
    /// packed YCbCr format, \[31:0\] Cr0:Y1:Cb0:Y0 8:8:8:8 little endian; XML since=1.
    /// ```xml
    /// <entry name="yuyv" value="0x56595559" summary="packed YCbCr format, [31:0] Cr0:Y1:Cb0:Y0 8:8:8:8 little endian" />
    /// ```
    pub const YUYV: Self = Self(0x56595559);
    /// packed YCbCr format, \[31:0\] Cb0:Y1:Cr0:Y0 8:8:8:8 little endian; XML since=1.
    /// ```xml
    /// <entry name="yvyu" value="0x55595659" summary="packed YCbCr format, [31:0] Cb0:Y1:Cr0:Y0 8:8:8:8 little endian" />
    /// ```
    pub const YVYU: Self = Self(0x55595659);
    /// packed YCbCr format, \[31:0\] Y1:Cr0:Y0:Cb0 8:8:8:8 little endian; XML since=1.
    /// ```xml
    /// <entry name="uyvy" value="0x59565955" summary="packed YCbCr format, [31:0] Y1:Cr0:Y0:Cb0 8:8:8:8 little endian" />
    /// ```
    pub const UYVY: Self = Self(0x59565955);
    /// packed YCbCr format, \[31:0\] Y1:Cb0:Y0:Cr0 8:8:8:8 little endian; XML since=1.
    /// ```xml
    /// <entry name="vyuy" value="0x59555956" summary="packed YCbCr format, [31:0] Y1:Cb0:Y0:Cr0 8:8:8:8 little endian" />
    /// ```
    pub const VYUY: Self = Self(0x59555956);
    /// packed AYCbCr format, \[31:0\] A:Y:Cb:Cr 8:8:8:8 little endian; XML since=1.
    /// ```xml
    /// <entry name="ayuv" value="0x56555941" summary="packed AYCbCr format, [31:0] A:Y:Cb:Cr 8:8:8:8 little endian" />
    /// ```
    pub const AYUV: Self = Self(0x56555941);
    /// 2 plane YCbCr Cr:Cb format, 2x2 subsampled Cr:Cb plane; XML since=1.
    /// ```xml
    /// <entry name="nv12" value="0x3231564e" summary="2 plane YCbCr Cr:Cb format, 2x2 subsampled Cr:Cb plane" />
    /// ```
    pub const NV12: Self = Self(0x3231564e);
    /// 2 plane YCbCr Cb:Cr format, 2x2 subsampled Cb:Cr plane; XML since=1.
    /// ```xml
    /// <entry name="nv21" value="0x3132564e" summary="2 plane YCbCr Cb:Cr format, 2x2 subsampled Cb:Cr plane" />
    /// ```
    pub const NV21: Self = Self(0x3132564e);
    /// 2 plane YCbCr Cr:Cb format, 2x1 subsampled Cr:Cb plane; XML since=1.
    /// ```xml
    /// <entry name="nv16" value="0x3631564e" summary="2 plane YCbCr Cr:Cb format, 2x1 subsampled Cr:Cb plane" />
    /// ```
    pub const NV16: Self = Self(0x3631564e);
    /// 2 plane YCbCr Cb:Cr format, 2x1 subsampled Cb:Cr plane; XML since=1.
    /// ```xml
    /// <entry name="nv61" value="0x3136564e" summary="2 plane YCbCr Cb:Cr format, 2x1 subsampled Cb:Cr plane" />
    /// ```
    pub const NV61: Self = Self(0x3136564e);
    /// 3 plane YCbCr format, 4x4 subsampled Cb (1) and Cr (2) planes; XML since=1.
    /// ```xml
    /// <entry name="yuv410" value="0x39565559" summary="3 plane YCbCr format, 4x4 subsampled Cb (1) and Cr (2) planes" />
    /// ```
    pub const YUV410: Self = Self(0x39565559);
    /// 3 plane YCbCr format, 4x4 subsampled Cr (1) and Cb (2) planes; XML since=1.
    /// ```xml
    /// <entry name="yvu410" value="0x39555659" summary="3 plane YCbCr format, 4x4 subsampled Cr (1) and Cb (2) planes" />
    /// ```
    pub const YVU410: Self = Self(0x39555659);
    /// 3 plane YCbCr format, 4x1 subsampled Cb (1) and Cr (2) planes; XML since=1.
    /// ```xml
    /// <entry name="yuv411" value="0x31315559" summary="3 plane YCbCr format, 4x1 subsampled Cb (1) and Cr (2) planes" />
    /// ```
    pub const YUV411: Self = Self(0x31315559);
    /// 3 plane YCbCr format, 4x1 subsampled Cr (1) and Cb (2) planes; XML since=1.
    /// ```xml
    /// <entry name="yvu411" value="0x31315659" summary="3 plane YCbCr format, 4x1 subsampled Cr (1) and Cb (2) planes" />
    /// ```
    pub const YVU411: Self = Self(0x31315659);
    /// 3 plane YCbCr format, 2x2 subsampled Cb (1) and Cr (2) planes; XML since=1.
    /// ```xml
    /// <entry name="yuv420" value="0x32315559" summary="3 plane YCbCr format, 2x2 subsampled Cb (1) and Cr (2) planes" />
    /// ```
    pub const YUV420: Self = Self(0x32315559);
    /// 3 plane YCbCr format, 2x2 subsampled Cr (1) and Cb (2) planes; XML since=1.
    /// ```xml
    /// <entry name="yvu420" value="0x32315659" summary="3 plane YCbCr format, 2x2 subsampled Cr (1) and Cb (2) planes" />
    /// ```
    pub const YVU420: Self = Self(0x32315659);
    /// 3 plane YCbCr format, 2x1 subsampled Cb (1) and Cr (2) planes; XML since=1.
    /// ```xml
    /// <entry name="yuv422" value="0x36315559" summary="3 plane YCbCr format, 2x1 subsampled Cb (1) and Cr (2) planes" />
    /// ```
    pub const YUV422: Self = Self(0x36315559);
    /// 3 plane YCbCr format, 2x1 subsampled Cr (1) and Cb (2) planes; XML since=1.
    /// ```xml
    /// <entry name="yvu422" value="0x36315659" summary="3 plane YCbCr format, 2x1 subsampled Cr (1) and Cb (2) planes" />
    /// ```
    pub const YVU422: Self = Self(0x36315659);
    /// 3 plane YCbCr format, non-subsampled Cb (1) and Cr (2) planes; XML since=1.
    /// ```xml
    /// <entry name="yuv444" value="0x34325559" summary="3 plane YCbCr format, non-subsampled Cb (1) and Cr (2) planes" />
    /// ```
    pub const YUV444: Self = Self(0x34325559);
    /// 3 plane YCbCr format, non-subsampled Cr (1) and Cb (2) planes; XML since=1.
    /// ```xml
    /// <entry name="yvu444" value="0x34325659" summary="3 plane YCbCr format, non-subsampled Cr (1) and Cb (2) planes" />
    /// ```
    pub const YVU444: Self = Self(0x34325659);
    /// \[7:0\] R; XML since=1.
    /// ```xml
    /// <entry name="r8" value="0x20203852" summary="[7:0] R" />
    /// ```
    pub const R8: Self = Self(0x20203852);
    /// \[15:0\] R little endian; XML since=1.
    /// ```xml
    /// <entry name="r16" value="0x20363152" summary="[15:0] R little endian" />
    /// ```
    pub const R16: Self = Self(0x20363152);
    /// \[15:0\] R:G 8:8 little endian; XML since=1.
    /// ```xml
    /// <entry name="rg88" value="0x38384752" summary="[15:0] R:G 8:8 little endian" />
    /// ```
    pub const RG88: Self = Self(0x38384752);
    /// \[15:0\] G:R 8:8 little endian; XML since=1.
    /// ```xml
    /// <entry name="gr88" value="0x38385247" summary="[15:0] G:R 8:8 little endian" />
    /// ```
    pub const GR88: Self = Self(0x38385247);
    /// \[31:0\] R:G 16:16 little endian; XML since=1.
    /// ```xml
    /// <entry name="rg1616" value="0x32334752" summary="[31:0] R:G 16:16 little endian" />
    /// ```
    pub const RG1616: Self = Self(0x32334752);
    /// \[31:0\] G:R 16:16 little endian; XML since=1.
    /// ```xml
    /// <entry name="gr1616" value="0x32335247" summary="[31:0] G:R 16:16 little endian" />
    /// ```
    pub const GR1616: Self = Self(0x32335247);
    /// \[63:0\] x:R:G:B 16:16:16:16 little endian; XML since=1.
    /// ```xml
    /// <entry name="xrgb16161616f" value="0x48345258" summary="[63:0] x:R:G:B 16:16:16:16 little endian" />
    /// ```
    pub const XRGB16161616F: Self = Self(0x48345258);
    /// \[63:0\] x:B:G:R 16:16:16:16 little endian; XML since=1.
    /// ```xml
    /// <entry name="xbgr16161616f" value="0x48344258" summary="[63:0] x:B:G:R 16:16:16:16 little endian" />
    /// ```
    pub const XBGR16161616F: Self = Self(0x48344258);
    /// \[63:0\] A:R:G:B 16:16:16:16 little endian; XML since=1.
    /// ```xml
    /// <entry name="argb16161616f" value="0x48345241" summary="[63:0] A:R:G:B 16:16:16:16 little endian" />
    /// ```
    pub const ARGB16161616F: Self = Self(0x48345241);
    /// \[63:0\] A:B:G:R 16:16:16:16 little endian; XML since=1.
    /// ```xml
    /// <entry name="abgr16161616f" value="0x48344241" summary="[63:0] A:B:G:R 16:16:16:16 little endian" />
    /// ```
    pub const ABGR16161616F: Self = Self(0x48344241);
    /// \[31:0\] X:Y:Cb:Cr 8:8:8:8 little endian; XML since=1.
    /// ```xml
    /// <entry name="xyuv8888" value="0x56555958" summary="[31:0] X:Y:Cb:Cr 8:8:8:8 little endian" />
    /// ```
    pub const XYUV8888: Self = Self(0x56555958);
    /// \[23:0\] Cr:Cb:Y 8:8:8 little endian; XML since=1.
    /// ```xml
    /// <entry name="vuy888" value="0x34325556" summary="[23:0] Cr:Cb:Y 8:8:8 little endian" />
    /// ```
    pub const VUY888: Self = Self(0x34325556);
    /// Y followed by U then V, 10:10:10. Non-linear modifier only; XML since=1.
    /// ```xml
    /// <entry name="vuy101010" value="0x30335556" summary="Y followed by U then V, 10:10:10. Non-linear modifier only" />
    /// ```
    pub const VUY101010: Self = Self(0x30335556);
    /// \[63:0\] Cr0:0:Y1:0:Cb0:0:Y0:0 10:6:10:6:10:6:10:6 little endian per 2 Y pixels; XML since=1.
    /// ```xml
    /// <entry name="y210" value="0x30313259" summary="[63:0] Cr0:0:Y1:0:Cb0:0:Y0:0 10:6:10:6:10:6:10:6 little endian per 2 Y pixels" />
    /// ```
    pub const Y210: Self = Self(0x30313259);
    /// \[63:0\] Cr0:0:Y1:0:Cb0:0:Y0:0 12:4:12:4:12:4:12:4 little endian per 2 Y pixels; XML since=1.
    /// ```xml
    /// <entry name="y212" value="0x32313259" summary="[63:0] Cr0:0:Y1:0:Cb0:0:Y0:0 12:4:12:4:12:4:12:4 little endian per 2 Y pixels" />
    /// ```
    pub const Y212: Self = Self(0x32313259);
    /// \[63:0\] Cr0:Y1:Cb0:Y0 16:16:16:16 little endian per 2 Y pixels; XML since=1.
    /// ```xml
    /// <entry name="y216" value="0x36313259" summary="[63:0] Cr0:Y1:Cb0:Y0 16:16:16:16 little endian per 2 Y pixels" />
    /// ```
    pub const Y216: Self = Self(0x36313259);
    /// \[31:0\] A:Cr:Y:Cb 2:10:10:10 little endian; XML since=1.
    /// ```xml
    /// <entry name="y410" value="0x30313459" summary="[31:0] A:Cr:Y:Cb 2:10:10:10 little endian" />
    /// ```
    pub const Y410: Self = Self(0x30313459);
    /// \[63:0\] A:0:Cr:0:Y:0:Cb:0 12:4:12:4:12:4:12:4 little endian; XML since=1.
    /// ```xml
    /// <entry name="y412" value="0x32313459" summary="[63:0] A:0:Cr:0:Y:0:Cb:0 12:4:12:4:12:4:12:4 little endian" />
    /// ```
    pub const Y412: Self = Self(0x32313459);
    /// \[63:0\] A:Cr:Y:Cb 16:16:16:16 little endian; XML since=1.
    /// ```xml
    /// <entry name="y416" value="0x36313459" summary="[63:0] A:Cr:Y:Cb 16:16:16:16 little endian" />
    /// ```
    pub const Y416: Self = Self(0x36313459);
    /// \[31:0\] X:Cr:Y:Cb 2:10:10:10 little endian; XML since=1.
    /// ```xml
    /// <entry name="xvyu2101010" value="0x30335658" summary="[31:0] X:Cr:Y:Cb 2:10:10:10 little endian" />
    /// ```
    pub const XVYU2101010: Self = Self(0x30335658);
    /// \[63:0\] X:0:Cr:0:Y:0:Cb:0 12:4:12:4:12:4:12:4 little endian; XML since=1.
    /// ```xml
    /// <entry name="xvyu12_16161616" value="0x36335658" summary="[63:0] X:0:Cr:0:Y:0:Cb:0 12:4:12:4:12:4:12:4 little endian" />
    /// ```
    pub const XVYU12_16161616: Self = Self(0x36335658);
    /// \[63:0\] X:Cr:Y:Cb 16:16:16:16 little endian; XML since=1.
    /// ```xml
    /// <entry name="xvyu16161616" value="0x38345658" summary="[63:0] X:Cr:Y:Cb 16:16:16:16 little endian" />
    /// ```
    pub const XVYU16161616: Self = Self(0x38345658);
    /// \[63:0\]   A3:A2:Y3:0:Cr0:0:Y2:0:A1:A0:Y1:0:Cb0:0:Y0:0  1:1:8:2:8:2:8:2:1:1:8:2:8:2:8:2 little endian; XML since=1.
    /// ```xml
    /// <entry name="y0l0" value="0x304c3059" summary="[63:0]   A3:A2:Y3:0:Cr0:0:Y2:0:A1:A0:Y1:0:Cb0:0:Y0:0  1:1:8:2:8:2:8:2:1:1:8:2:8:2:8:2 little endian" />
    /// ```
    pub const Y0L0: Self = Self(0x304c3059);
    /// \[63:0\]   X3:X2:Y3:0:Cr0:0:Y2:0:X1:X0:Y1:0:Cb0:0:Y0:0  1:1:8:2:8:2:8:2:1:1:8:2:8:2:8:2 little endian; XML since=1.
    /// ```xml
    /// <entry name="x0l0" value="0x304c3058" summary="[63:0]   X3:X2:Y3:0:Cr0:0:Y2:0:X1:X0:Y1:0:Cb0:0:Y0:0  1:1:8:2:8:2:8:2:1:1:8:2:8:2:8:2 little endian" />
    /// ```
    pub const X0L0: Self = Self(0x304c3058);
    /// \[63:0\]   A3:A2:Y3:Cr0:Y2:A1:A0:Y1:Cb0:Y0  1:1:10:10:10:1:1:10:10:10 little endian; XML since=1.
    /// ```xml
    /// <entry name="y0l2" value="0x324c3059" summary="[63:0]   A3:A2:Y3:Cr0:Y2:A1:A0:Y1:Cb0:Y0  1:1:10:10:10:1:1:10:10:10 little endian" />
    /// ```
    pub const Y0L2: Self = Self(0x324c3059);
    /// \[63:0\]   X3:X2:Y3:Cr0:Y2:X1:X0:Y1:Cb0:Y0  1:1:10:10:10:1:1:10:10:10 little endian; XML since=1.
    /// ```xml
    /// <entry name="x0l2" value="0x324c3058" summary="[63:0]   X3:X2:Y3:Cr0:Y2:X1:X0:Y1:Cb0:Y0  1:1:10:10:10:1:1:10:10:10 little endian" />
    /// ```
    pub const X0L2: Self = Self(0x324c3058);
    /// yuv420_8bit; XML since=1.
    /// ```xml
    /// <entry name="yuv420_8bit" value="0x38305559" />
    /// ```
    pub const YUV420_8BIT: Self = Self(0x38305559);
    /// yuv420_10bit; XML since=1.
    /// ```xml
    /// <entry name="yuv420_10bit" value="0x30315559" />
    /// ```
    pub const YUV420_10BIT: Self = Self(0x30315559);
    /// xrgb8888_a8; XML since=1.
    /// ```xml
    /// <entry name="xrgb8888_a8" value="0x38415258" />
    /// ```
    pub const XRGB8888_A8: Self = Self(0x38415258);
    /// xbgr8888_a8; XML since=1.
    /// ```xml
    /// <entry name="xbgr8888_a8" value="0x38414258" />
    /// ```
    pub const XBGR8888_A8: Self = Self(0x38414258);
    /// rgbx8888_a8; XML since=1.
    /// ```xml
    /// <entry name="rgbx8888_a8" value="0x38415852" />
    /// ```
    pub const RGBX8888_A8: Self = Self(0x38415852);
    /// bgrx8888_a8; XML since=1.
    /// ```xml
    /// <entry name="bgrx8888_a8" value="0x38415842" />
    /// ```
    pub const BGRX8888_A8: Self = Self(0x38415842);
    /// rgb888_a8; XML since=1.
    /// ```xml
    /// <entry name="rgb888_a8" value="0x38413852" />
    /// ```
    pub const RGB888_A8: Self = Self(0x38413852);
    /// bgr888_a8; XML since=1.
    /// ```xml
    /// <entry name="bgr888_a8" value="0x38413842" />
    /// ```
    pub const BGR888_A8: Self = Self(0x38413842);
    /// rgb565_a8; XML since=1.
    /// ```xml
    /// <entry name="rgb565_a8" value="0x38413552" />
    /// ```
    pub const RGB565_A8: Self = Self(0x38413552);
    /// bgr565_a8; XML since=1.
    /// ```xml
    /// <entry name="bgr565_a8" value="0x38413542" />
    /// ```
    pub const BGR565_A8: Self = Self(0x38413542);
    /// non-subsampled Cr:Cb plane; XML since=1.
    /// ```xml
    /// <entry name="nv24" value="0x3432564e" summary="non-subsampled Cr:Cb plane" />
    /// ```
    pub const NV24: Self = Self(0x3432564e);
    /// non-subsampled Cb:Cr plane; XML since=1.
    /// ```xml
    /// <entry name="nv42" value="0x3234564e" summary="non-subsampled Cb:Cr plane" />
    /// ```
    pub const NV42: Self = Self(0x3234564e);
    /// 2x1 subsampled Cr:Cb plane, 10 bit per channel; XML since=1.
    /// ```xml
    /// <entry name="p210" value="0x30313250" summary="2x1 subsampled Cr:Cb plane, 10 bit per channel" />
    /// ```
    pub const P210: Self = Self(0x30313250);
    /// 2x2 subsampled Cr:Cb plane 10 bits per channel; XML since=1.
    /// ```xml
    /// <entry name="p010" value="0x30313050" summary="2x2 subsampled Cr:Cb plane 10 bits per channel" />
    /// ```
    pub const P010: Self = Self(0x30313050);
    /// 2x2 subsampled Cr:Cb plane 12 bits per channel; XML since=1.
    /// ```xml
    /// <entry name="p012" value="0x32313050" summary="2x2 subsampled Cr:Cb plane 12 bits per channel" />
    /// ```
    pub const P012: Self = Self(0x32313050);
    /// 2x2 subsampled Cr:Cb plane 16 bits per channel; XML since=1.
    /// ```xml
    /// <entry name="p016" value="0x36313050" summary="2x2 subsampled Cr:Cb plane 16 bits per channel" />
    /// ```
    pub const P016: Self = Self(0x36313050);
    /// \[63:0\] A:x:B:x:G:x:R:x 10:6:10:6:10:6:10:6 little endian; XML since=1.
    /// ```xml
    /// <entry name="axbxgxrx106106106106" value="0x30314241" summary="[63:0] A:x:B:x:G:x:R:x 10:6:10:6:10:6:10:6 little endian" />
    /// ```
    pub const AXBXGXRX106106106106: Self = Self(0x30314241);
    /// 2x2 subsampled Cr:Cb plane; XML since=1.
    /// ```xml
    /// <entry name="nv15" value="0x3531564e" summary="2x2 subsampled Cr:Cb plane" />
    /// ```
    pub const NV15: Self = Self(0x3531564e);
    /// q410; XML since=1.
    /// ```xml
    /// <entry name="q410" value="0x30313451" />
    /// ```
    pub const Q410: Self = Self(0x30313451);
    /// q401; XML since=1.
    /// ```xml
    /// <entry name="q401" value="0x31303451" />
    /// ```
    pub const Q401: Self = Self(0x31303451);
    /// \[63:0\] x:R:G:B 16:16:16:16 little endian; XML since=1.
    /// ```xml
    /// <entry name="xrgb16161616" value="0x38345258" summary="[63:0] x:R:G:B 16:16:16:16 little endian" />
    /// ```
    pub const XRGB16161616: Self = Self(0x38345258);
    /// \[63:0\] x:B:G:R 16:16:16:16 little endian; XML since=1.
    /// ```xml
    /// <entry name="xbgr16161616" value="0x38344258" summary="[63:0] x:B:G:R 16:16:16:16 little endian" />
    /// ```
    pub const XBGR16161616: Self = Self(0x38344258);
    /// \[63:0\] A:R:G:B 16:16:16:16 little endian; XML since=1.
    /// ```xml
    /// <entry name="argb16161616" value="0x38345241" summary="[63:0] A:R:G:B 16:16:16:16 little endian" />
    /// ```
    pub const ARGB16161616: Self = Self(0x38345241);
    /// \[63:0\] A:B:G:R 16:16:16:16 little endian; XML since=1.
    /// ```xml
    /// <entry name="abgr16161616" value="0x38344241" summary="[63:0] A:B:G:R 16:16:16:16 little endian" />
    /// ```
    pub const ABGR16161616: Self = Self(0x38344241);
    /// \[7:0\] C0:C1:C2:C3:C4:C5:C6:C7 1:1:1:1:1:1:1:1 eight pixels/byte; XML since=1.
    /// ```xml
    /// <entry name="c1" value="0x20203143" summary="[7:0] C0:C1:C2:C3:C4:C5:C6:C7 1:1:1:1:1:1:1:1 eight pixels/byte" />
    /// ```
    pub const C1: Self = Self(0x20203143);
    /// \[7:0\] C0:C1:C2:C3 2:2:2:2 four pixels/byte; XML since=1.
    /// ```xml
    /// <entry name="c2" value="0x20203243" summary="[7:0] C0:C1:C2:C3 2:2:2:2 four pixels/byte" />
    /// ```
    pub const C2: Self = Self(0x20203243);
    /// \[7:0\] C0:C1 4:4 two pixels/byte; XML since=1.
    /// ```xml
    /// <entry name="c4" value="0x20203443" summary="[7:0] C0:C1 4:4 two pixels/byte" />
    /// ```
    pub const C4: Self = Self(0x20203443);
    /// \[7:0\] D0:D1:D2:D3:D4:D5:D6:D7 1:1:1:1:1:1:1:1 eight pixels/byte; XML since=1.
    /// ```xml
    /// <entry name="d1" value="0x20203144" summary="[7:0] D0:D1:D2:D3:D4:D5:D6:D7 1:1:1:1:1:1:1:1 eight pixels/byte" />
    /// ```
    pub const D1: Self = Self(0x20203144);
    /// \[7:0\] D0:D1:D2:D3 2:2:2:2 four pixels/byte; XML since=1.
    /// ```xml
    /// <entry name="d2" value="0x20203244" summary="[7:0] D0:D1:D2:D3 2:2:2:2 four pixels/byte" />
    /// ```
    pub const D2: Self = Self(0x20203244);
    /// \[7:0\] D0:D1 4:4 two pixels/byte; XML since=1.
    /// ```xml
    /// <entry name="d4" value="0x20203444" summary="[7:0] D0:D1 4:4 two pixels/byte" />
    /// ```
    pub const D4: Self = Self(0x20203444);
    /// \[7:0\] D; XML since=1.
    /// ```xml
    /// <entry name="d8" value="0x20203844" summary="[7:0] D" />
    /// ```
    pub const D8: Self = Self(0x20203844);
    /// \[7:0\] R0:R1:R2:R3:R4:R5:R6:R7 1:1:1:1:1:1:1:1 eight pixels/byte; XML since=1.
    /// ```xml
    /// <entry name="r1" value="0x20203152" summary="[7:0] R0:R1:R2:R3:R4:R5:R6:R7 1:1:1:1:1:1:1:1 eight pixels/byte" />
    /// ```
    pub const R1: Self = Self(0x20203152);
    /// \[7:0\] R0:R1:R2:R3 2:2:2:2 four pixels/byte; XML since=1.
    /// ```xml
    /// <entry name="r2" value="0x20203252" summary="[7:0] R0:R1:R2:R3 2:2:2:2 four pixels/byte" />
    /// ```
    pub const R2: Self = Self(0x20203252);
    /// \[7:0\] R0:R1 4:4 two pixels/byte; XML since=1.
    /// ```xml
    /// <entry name="r4" value="0x20203452" summary="[7:0] R0:R1 4:4 two pixels/byte" />
    /// ```
    pub const R4: Self = Self(0x20203452);
    /// \[15:0\] x:R 6:10 little endian; XML since=1.
    /// ```xml
    /// <entry name="r10" value="0x20303152" summary="[15:0] x:R 6:10 little endian" />
    /// ```
    pub const R10: Self = Self(0x20303152);
    /// \[15:0\] x:R 4:12 little endian; XML since=1.
    /// ```xml
    /// <entry name="r12" value="0x20323152" summary="[15:0] x:R 4:12 little endian" />
    /// ```
    pub const R12: Self = Self(0x20323152);
    /// \[31:0\] A:Cr:Cb:Y 8:8:8:8 little endian; XML since=1.
    /// ```xml
    /// <entry name="avuy8888" value="0x59555641" summary="[31:0] A:Cr:Cb:Y 8:8:8:8 little endian" />
    /// ```
    pub const AVUY8888: Self = Self(0x59555641);
    /// \[31:0\] X:Cr:Cb:Y 8:8:8:8 little endian; XML since=1.
    /// ```xml
    /// <entry name="xvuy8888" value="0x59555658" summary="[31:0] X:Cr:Cb:Y 8:8:8:8 little endian" />
    /// ```
    pub const XVUY8888: Self = Self(0x59555658);
    /// 2x2 subsampled Cr:Cb plane 10 bits per channel packed; XML since=1.
    /// ```xml
    /// <entry name="p030" value="0x30333050" summary="2x2 subsampled Cr:Cb plane 10 bits per channel packed" />
    /// ```
    pub const P030: Self = Self(0x30333050);
    /// \[47:0\] R:G:B 16:16:16 little endian; XML since=1.
    /// ```xml
    /// <entry name="rgb161616" value="0x38344752" summary="[47:0] R:G:B 16:16:16 little endian" />
    /// ```
    pub const RGB161616: Self = Self(0x38344752);
    /// \[47:0\] B:G:R 16:16:16 little endian; XML since=1.
    /// ```xml
    /// <entry name="bgr161616" value="0x38344742" summary="[47:0] B:G:R 16:16:16 little endian" />
    /// ```
    pub const BGR161616: Self = Self(0x38344742);
    /// \[15:0\] R 16 little endian; XML since=1.
    /// ```xml
    /// <entry name="r16f" value="0x48202052" summary="[15:0] R 16 little endian" />
    /// ```
    pub const R16F: Self = Self(0x48202052);
    /// \[31:0\] G:R 16:16 little endian; XML since=1.
    /// ```xml
    /// <entry name="gr1616f" value="0x48205247" summary="[31:0] G:R 16:16 little endian" />
    /// ```
    pub const GR1616F: Self = Self(0x48205247);
    /// \[47:0\] B:G:R 16:16:16 little endian; XML since=1.
    /// ```xml
    /// <entry name="bgr161616f" value="0x48524742" summary="[47:0] B:G:R 16:16:16 little endian" />
    /// ```
    pub const BGR161616F: Self = Self(0x48524742);
    /// \[31:0\] R 32 little endian; XML since=1.
    /// ```xml
    /// <entry name="r32f" value="0x46202052" summary="[31:0] R 32 little endian" />
    /// ```
    pub const R32F: Self = Self(0x46202052);
    /// \[63:0\] G:R 32:32 little endian; XML since=1.
    /// ```xml
    /// <entry name="gr3232f" value="0x46205247" summary="[63:0] G:R 32:32 little endian" />
    /// ```
    pub const GR3232F: Self = Self(0x46205247);
    /// \[95:0\] B:G:R 32:32:32 little endian; XML since=1.
    /// ```xml
    /// <entry name="bgr323232f" value="0x46524742" summary="[95:0] B:G:R 32:32:32 little endian" />
    /// ```
    pub const BGR323232F: Self = Self(0x46524742);
    /// \[127:0\] A:B:G:R 32:32:32:32 little endian; XML since=1.
    /// ```xml
    /// <entry name="abgr32323232f" value="0x46384241" summary="[127:0] A:B:G:R 32:32:32:32 little endian" />
    /// ```
    pub const ABGR32323232F: Self = Self(0x46384241);
    /// 2x1 subsampled Cr:Cb plane; XML since=1.
    /// ```xml
    /// <entry name="nv20" value="0x3032564e" summary="2x1 subsampled Cr:Cb plane" />
    /// ```
    pub const NV20: Self = Self(0x3032564e);
    /// non-subsampled Cr:Cb plane; XML since=1.
    /// ```xml
    /// <entry name="nv30" value="0x3033564e" summary="non-subsampled Cr:Cb plane" />
    /// ```
    pub const NV30: Self = Self(0x3033564e);
    /// 2x2 subsampled Cb (1) and Cr (2) planes 10 bits per channel; XML since=1.
    /// ```xml
    /// <entry name="s010" value="0x30313053" summary="2x2 subsampled Cb (1) and Cr (2) planes 10 bits per channel" />
    /// ```
    pub const S010: Self = Self(0x30313053);
    /// 2x1 subsampled Cb (1) and Cr (2) planes 10 bits per channel; XML since=1.
    /// ```xml
    /// <entry name="s210" value="0x30313253" summary="2x1 subsampled Cb (1) and Cr (2) planes 10 bits per channel" />
    /// ```
    pub const S210: Self = Self(0x30313253);
    /// non-subsampled Cb (1) and Cr (2) planes 10 bits per channel; XML since=1.
    /// ```xml
    /// <entry name="s410" value="0x30313453" summary="non-subsampled Cb (1) and Cr (2) planes 10 bits per channel" />
    /// ```
    pub const S410: Self = Self(0x30313453);
    /// 2x2 subsampled Cb (1) and Cr (2) planes 12 bits per channel; XML since=1.
    /// ```xml
    /// <entry name="s012" value="0x32313053" summary="2x2 subsampled Cb (1) and Cr (2) planes 12 bits per channel" />
    /// ```
    pub const S012: Self = Self(0x32313053);
    /// 2x1 subsampled Cb (1) and Cr (2) planes 12 bits per channel; XML since=1.
    /// ```xml
    /// <entry name="s212" value="0x32313253" summary="2x1 subsampled Cb (1) and Cr (2) planes 12 bits per channel" />
    /// ```
    pub const S212: Self = Self(0x32313253);
    /// non-subsampled Cb (1) and Cr (2) planes 12 bits per channel; XML since=1.
    /// ```xml
    /// <entry name="s412" value="0x32313453" summary="non-subsampled Cb (1) and Cr (2) planes 12 bits per channel" />
    /// ```
    pub const S412: Self = Self(0x32313453);
    /// 2x2 subsampled Cb (1) and Cr (2) planes 16 bits per channel; XML since=1.
    /// ```xml
    /// <entry name="s016" value="0x36313053" summary="2x2 subsampled Cb (1) and Cr (2) planes 16 bits per channel" />
    /// ```
    pub const S016: Self = Self(0x36313053);
    /// 2x1 subsampled Cb (1) and Cr (2) planes 16 bits per channel; XML since=1.
    /// ```xml
    /// <entry name="s216" value="0x36313253" summary="2x1 subsampled Cb (1) and Cr (2) planes 16 bits per channel" />
    /// ```
    pub const S216: Self = Self(0x36313253);
    /// non-subsampled Cb (1) and Cr (2) planes 16 bits per channel; XML since=1.
    /// ```xml
    /// <entry name="s416" value="0x36313453" summary="non-subsampled Cb (1) and Cr (2) planes 16 bits per channel" />
    /// ```
    pub const S416: Self = Self(0x36313453);
    /// \[31:0\] x:Cr:Cb:Y 2:10:10:10 little endian; XML since=1.
    /// ```xml
    /// <entry name="xvuy2101010" value="0x30335958" summary="[31:0] x:Cr:Cb:Y 2:10:10:10 little endian" />
    /// ```
    pub const XVUY2101010: Self = Self(0x30335958);
    /// 2x1 subsampled Cr:Cb plane 10 bits per channel packed; XML since=1.
    /// ```xml
    /// <entry name="p230" value="0x30333250" summary="2x1 subsampled Cr:Cb plane 10 bits per channel packed" />
    /// ```
    pub const P230: Self = Self(0x30333250);
    /// t430; XML since=1.
    /// ```xml
    /// <entry name="t430" value="0x30333454" />
    /// ```
    pub const T430: Self = Self(0x30333454);
    /// 8-bit Y-only; XML since=1.
    /// ```xml
    /// <entry name="y8" value="0x59455247" summary="8-bit Y-only" />
    /// ```
    pub const Y8: Self = Self(0x59455247);
    /// \[31:0\] x:Y2:Y1:Y0 2:10:10:10 little endian; XML since=1.
    /// ```xml
    /// <entry name="xyyy2101010" value="0x34415059" summary="[31:0] x:Y2:Y1:Y0 2:10:10:10 little endian" />
    /// ```
    pub const XYYY2101010: Self = Self(0x34415059);
}
impl CommandReader<Self> for WlShmFormat {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlShmFormat {
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
