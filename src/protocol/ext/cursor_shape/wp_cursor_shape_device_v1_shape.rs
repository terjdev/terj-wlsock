use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_cursor_shape_device_v1.shape`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="shape">
///       <description summary="cursor shapes">
///         This enum describes cursor shapes.
///
///         The names are taken from the CSS W3C specification:
///         https://w3c.github.io/csswg-drafts/css-ui/#cursor
///         with a few additions.
///
///         Note that there are some groups of cursor shapes that are related:
///         The first group is drag-and-drop cursors which are used to indicate
///         the selected action during dnd operations. The second group is resize
///         cursors which are used to indicate resizing and moving possibilities
///         on window borders. It is recommended that the shapes in these groups
///         should use visually compatible images and metaphors.
///       </description>
///       <entry name="default" value="1" summary="default cursor" />
///       <entry name="context_menu" value="2" summary="a context menu is available for the object under the cursor" />
///       <entry name="help" value="3" summary="help is available for the object under the cursor" />
///       <entry name="pointer" value="4" summary="pointer that indicates a link or another interactive element" />
///       <entry name="progress" value="5" summary="progress indicator" />
///       <entry name="wait" value="6" summary="program is busy, user should wait" />
///       <entry name="cell" value="7" summary="a cell or set of cells may be selected" />
///       <entry name="crosshair" value="8" summary="simple crosshair" />
///       <entry name="text" value="9" summary="text may be selected" />
///       <entry name="vertical_text" value="10" summary="vertical text may be selected" />
///       <entry name="alias" value="11" summary="drag-and-drop: alias of/shortcut to something is to be created" />
///       <entry name="copy" value="12" summary="drag-and-drop: something is to be copied" />
///       <entry name="move" value="13" summary="drag-and-drop: something is to be moved" />
///       <entry name="no_drop" value="14" summary="drag-and-drop: the dragged item cannot be dropped at the current cursor location" />
///       <entry name="not_allowed" value="15" summary="drag-and-drop: the requested action will not be carried out" />
///       <entry name="grab" value="16" summary="drag-and-drop: something can be grabbed" />
///       <entry name="grabbing" value="17" summary="drag-and-drop: something is being grabbed" />
///       <entry name="e_resize" value="18" summary="resizing: the east border is to be moved" />
///       <entry name="n_resize" value="19" summary="resizing: the north border is to be moved" />
///       <entry name="ne_resize" value="20" summary="resizing: the north-east corner is to be moved" />
///       <entry name="nw_resize" value="21" summary="resizing: the north-west corner is to be moved" />
///       <entry name="s_resize" value="22" summary="resizing: the south border is to be moved" />
///       <entry name="se_resize" value="23" summary="resizing: the south-east corner is to be moved" />
///       <entry name="sw_resize" value="24" summary="resizing: the south-west corner is to be moved" />
///       <entry name="w_resize" value="25" summary="resizing: the west border is to be moved" />
///       <entry name="ew_resize" value="26" summary="resizing: the east and west borders are to be moved" />
///       <entry name="ns_resize" value="27" summary="resizing: the north and south borders are to be moved" />
///       <entry name="nesw_resize" value="28" summary="resizing: the north-east and south-west corners are to be moved" />
///       <entry name="nwse_resize" value="29" summary="resizing: the north-west and south-east corners are to be moved" />
///       <entry name="col_resize" value="30" summary="resizing: that the item/column can be resized horizontally" />
///       <entry name="row_resize" value="31" summary="resizing: that the item/row can be resized vertically" />
///       <entry name="all_scroll" value="32" summary="something can be scrolled in any direction" />
///       <entry name="zoom_in" value="33" summary="something can be zoomed in" />
///       <entry name="zoom_out" value="34" summary="something can be zoomed out" />
///       <entry name="dnd_ask" value="35" summary="drag-and-drop: the user will select which action will be carried out (non-css value)" since="2" />
///       <entry name="all_resize" value="36" summary="resizing: something can be moved or resized in any direction (non-css value)" since="2" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpCursorShapeDeviceV1Shape(pub u32);
impl WpCursorShapeDeviceV1Shape {
    /// default cursor; available since version 1.
    pub const DEFAULT: Self = Self(1);
    /// a context menu is available for the object under the cursor; available since version 1.
    pub const CONTEXT_MENU: Self = Self(2);
    /// help is available for the object under the cursor; available since version 1.
    pub const HELP: Self = Self(3);
    /// pointer that indicates a link or another interactive element; available since version 1.
    pub const POINTER: Self = Self(4);
    /// progress indicator; available since version 1.
    pub const PROGRESS: Self = Self(5);
    /// program is busy, user should wait; available since version 1.
    pub const WAIT: Self = Self(6);
    /// a cell or set of cells may be selected; available since version 1.
    pub const CELL: Self = Self(7);
    /// simple crosshair; available since version 1.
    pub const CROSSHAIR: Self = Self(8);
    /// text may be selected; available since version 1.
    pub const TEXT: Self = Self(9);
    /// vertical text may be selected; available since version 1.
    pub const VERTICAL_TEXT: Self = Self(10);
    /// drag-and-drop: alias of/shortcut to something is to be created; available since version 1.
    pub const ALIAS: Self = Self(11);
    /// drag-and-drop: something is to be copied; available since version 1.
    pub const COPY: Self = Self(12);
    /// drag-and-drop: something is to be moved; available since version 1.
    pub const MOVE: Self = Self(13);
    /// drag-and-drop: the dragged item cannot be dropped at the current cursor location; available since version 1.
    pub const NO_DROP: Self = Self(14);
    /// drag-and-drop: the requested action will not be carried out; available since version 1.
    pub const NOT_ALLOWED: Self = Self(15);
    /// drag-and-drop: something can be grabbed; available since version 1.
    pub const GRAB: Self = Self(16);
    /// drag-and-drop: something is being grabbed; available since version 1.
    pub const GRABBING: Self = Self(17);
    /// resizing: the east border is to be moved; available since version 1.
    pub const E_RESIZE: Self = Self(18);
    /// resizing: the north border is to be moved; available since version 1.
    pub const N_RESIZE: Self = Self(19);
    /// resizing: the north-east corner is to be moved; available since version 1.
    pub const NE_RESIZE: Self = Self(20);
    /// resizing: the north-west corner is to be moved; available since version 1.
    pub const NW_RESIZE: Self = Self(21);
    /// resizing: the south border is to be moved; available since version 1.
    pub const S_RESIZE: Self = Self(22);
    /// resizing: the south-east corner is to be moved; available since version 1.
    pub const SE_RESIZE: Self = Self(23);
    /// resizing: the south-west corner is to be moved; available since version 1.
    pub const SW_RESIZE: Self = Self(24);
    /// resizing: the west border is to be moved; available since version 1.
    pub const W_RESIZE: Self = Self(25);
    /// resizing: the east and west borders are to be moved; available since version 1.
    pub const EW_RESIZE: Self = Self(26);
    /// resizing: the north and south borders are to be moved; available since version 1.
    pub const NS_RESIZE: Self = Self(27);
    /// resizing: the north-east and south-west corners are to be moved; available since version 1.
    pub const NESW_RESIZE: Self = Self(28);
    /// resizing: the north-west and south-east corners are to be moved; available since version 1.
    pub const NWSE_RESIZE: Self = Self(29);
    /// resizing: that the item/column can be resized horizontally; available since version 1.
    pub const COL_RESIZE: Self = Self(30);
    /// resizing: that the item/row can be resized vertically; available since version 1.
    pub const ROW_RESIZE: Self = Self(31);
    /// something can be scrolled in any direction; available since version 1.
    pub const ALL_SCROLL: Self = Self(32);
    /// something can be zoomed in; available since version 1.
    pub const ZOOM_IN: Self = Self(33);
    /// something can be zoomed out; available since version 1.
    pub const ZOOM_OUT: Self = Self(34);
    /// drag-and-drop: the user will select which action will be carried out (non-css value); available since version 2.
    pub const DND_ASK: Self = Self(35);
    /// resizing: something can be moved or resized in any direction (non-css value); available since version 2.
    pub const ALL_RESIZE: Self = Self(36);
}
impl CommandReader<Self> for WpCursorShapeDeviceV1Shape {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpCursorShapeDeviceV1Shape {
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
