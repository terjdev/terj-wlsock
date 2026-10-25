use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xdg_positioner.constraint_adjustment`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="constraint_adjustment" bitfield="true">
///       <description summary="constraint adjustments">
///     The constraint adjustment value define ways the compositor will adjust
///     the position of the surface, if the unadjusted position would result
///     in the surface being partly constrained.
///
///     Whether a surface is considered 'constrained' is left to the compositor
///     to determine. For example, the surface may be partly outside the
///     compositor's defined 'work area', thus necessitating the child surface's
///     position be adjusted until it is entirely inside the work area.
///
///     The adjustments can be combined, according to a defined precedence: 1)
///     Flip, 2) Slide, 3) Resize.
///       </description>
///       <entry name="none" value="0">
///     <description summary="don't move the child surface when constrained">
///       Don't alter the surface position even if it is constrained on some
///       axis, for example partially outside the edge of an output.
///     </description>
///       </entry>
///       <entry name="slide_x" value="1">
///     <description summary="move along the x axis until unconstrained">
///       Slide the surface along the x axis until it is no longer constrained.
///
///       First try to slide towards the direction of the gravity on the x axis
///       until either the edge in the opposite direction of the gravity is
///       unconstrained or the edge in the direction of the gravity is
///       constrained.
///
///       Then try to slide towards the opposite direction of the gravity on the
///       x axis until either the edge in the direction of the gravity is
///       unconstrained or the edge in the opposite direction of the gravity is
///       constrained.
///     </description>
///       </entry>
///       <entry name="slide_y" value="2">
///     <description summary="move along the y axis until unconstrained">
///       Slide the surface along the y axis until it is no longer constrained.
///
///       First try to slide towards the direction of the gravity on the y axis
///       until either the edge in the opposite direction of the gravity is
///       unconstrained or the edge in the direction of the gravity is
///       constrained.
///
///       Then try to slide towards the opposite direction of the gravity on the
///       y axis until either the edge in the direction of the gravity is
///       unconstrained or the edge in the opposite direction of the gravity is
///       constrained.
///     </description>
///       </entry>
///       <entry name="flip_x" value="4">
///     <description summary="invert the anchor and gravity on the x axis">
///       Invert the anchor and gravity on the x axis if the surface is
///       constrained on the x axis. For example, if the left edge of the
///       surface is constrained, the gravity is 'left' and the anchor is
///       'left', change the gravity to 'right' and the anchor to 'right'.
///
///       If the adjusted position also ends up being constrained, the resulting
///       position of the flip_x adjustment will be the one before the
///       adjustment.
///     </description>
///       </entry>
///       <entry name="flip_y" value="8">
///     <description summary="invert the anchor and gravity on the y axis">
///       Invert the anchor and gravity on the y axis if the surface is
///       constrained on the y axis. For example, if the bottom edge of the
///       surface is constrained, the gravity is 'bottom' and the anchor is
///       'bottom', change the gravity to 'top' and the anchor to 'top'.
///
///       The adjusted position is calculated given the original anchor
///       rectangle and offset, but with the new flipped anchor and gravity
///       values.
///
///       If the adjusted position also ends up being constrained, the resulting
///       position of the flip_y adjustment will be the one before the
///       adjustment.
///     </description>
///       </entry>
///       <entry name="resize_x" value="16">
///     <description summary="horizontally resize the surface">
///       Resize the surface horizontally so that it is completely
///       unconstrained.
///     </description>
///       </entry>
///       <entry name="resize_y" value="32">
///     <description summary="vertically resize the surface">
///       Resize the surface vertically so that it is completely unconstrained.
///     </description>
///       </entry>
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XdgPositionerConstraintAdjustment(pub u32);
impl XdgPositionerConstraintAdjustment {
    /// don't move the child surface when constrained; available since version 1.
    pub const NONE: Self = Self(0);
    /// move along the x axis until unconstrained; available since version 1.
    pub const SLIDE_X: Self = Self(1);
    /// move along the y axis until unconstrained; available since version 1.
    pub const SLIDE_Y: Self = Self(2);
    /// invert the anchor and gravity on the x axis; available since version 1.
    pub const FLIP_X: Self = Self(4);
    /// invert the anchor and gravity on the y axis; available since version 1.
    pub const FLIP_Y: Self = Self(8);
    /// horizontally resize the surface; available since version 1.
    pub const RESIZE_X: Self = Self(16);
    /// vertically resize the surface; available since version 1.
    pub const RESIZE_Y: Self = Self(32);
}
impl std::ops::BitOr for XdgPositionerConstraintAdjustment {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl std::ops::BitAnd for XdgPositionerConstraintAdjustment {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl CommandReader<Self> for XdgPositionerConstraintAdjustment {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XdgPositionerConstraintAdjustment {
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
