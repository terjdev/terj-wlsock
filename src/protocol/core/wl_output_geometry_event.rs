use super::{WlOutputSubpixel, WlOutputTransform};
use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_output.geometry`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="geometry">
///       <description summary="properties of the output">
///         The geometry event describes geometric properties of the output.
///         The event is sent when binding to the output object and whenever
///         any of the properties change.
///
///         The physical size can be set to zero if it doesn't make sense for this
///         output (e.g. for projectors or virtual outputs).
///
///         The geometry event will be followed by a done event (starting from
///         version 2).
///
///         Clients should use wl_surface.preferred_buffer_transform instead of the
///         transform advertised by this event to find the preferred buffer
///         transform to use for a surface.
///
///         Note: wl_output only advertises partial information about the output
///         position and identification. Some compositors, for instance those not
///         implementing a desktop-style output layout or those exposing virtual
///         outputs, might fake this information. Instead of using x and y, clients
///         should use xdg_output.logical_position. Instead of using make and model,
///         clients should use name and description.
///       </description>
///       <arg name="x" type="int" summary="x position within the global compositor space" />
///       <arg name="y" type="int" summary="y position within the global compositor space" />
///       <arg name="physical_width" type="int" summary="width in millimeters of the output" />
///       <arg name="physical_height" type="int" summary="height in millimeters of the output" />
///       <arg name="subpixel" type="int" enum="subpixel" summary="subpixel orientation of the output" />
///       <arg name="make" type="string" summary="textual description of the manufacturer" />
///       <arg name="model" type="string" summary="textual description of the model" />
///       <arg name="transform" type="int" enum="transform" summary="additional transformation applied to buffer contents during presentation" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlOutputGeometryEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `x`: x position within the global compositor space.
    pub x: i32,
    /// Argument `y`: y position within the global compositor space.
    pub y: i32,
    /// Argument `physical_width`: width in millimeters of the output.
    pub physical_width: i32,
    /// Argument `physical_height`: height in millimeters of the output.
    pub physical_height: i32,
    /// Argument `subpixel`: subpixel orientation of the output.
    pub subpixel: WlOutputSubpixel,
    /// Argument `make`: textual description of the manufacturer.
    pub make: String,
    /// Argument `model`: textual description of the model.
    pub model: String,
    /// Argument `transform`: additional transformation applied to buffer contents during presentation.
    pub transform: WlOutputTransform,
}
impl WlOutputGeometryEvent {
    /// Opcode in the event namespace of interface wl_output.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlOutputGeometryEvent {
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
            FdWriter::encoded_string_size(Some(&self.make)),
            FdWriter::encoded_string_size(Some(&self.model)),
            Some(4),
        ])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::event_interface(state, self.object_id, "wl_output", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.make) {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.model) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.x).await?;
        writer.next_i32(self.y).await?;
        writer.next_i32(self.physical_width).await?;
        writer.next_i32(self.physical_height).await?;
        writer.next_u32(self.subpixel.0).await?;
        writer.next_string(&self.make).await?;
        writer.next_string(&self.model).await?;
        writer.next_u32(self.transform.0).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlOutputGeometryEvent {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Self::event_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_output",
            Self::SINCE,
        )?;
        let x = reader.next_i32().await?;
        let y = reader.next_i32().await?;
        let physical_width = reader.next_i32().await?;
        let physical_height = reader.next_i32().await?;
        let subpixel = WlOutputSubpixel::read(object_id, opcode, reader, state).await?;
        let make = reader.next_string().await?;
        let model = reader.next_string().await?;
        let transform = WlOutputTransform::read(object_id, opcode, reader, state).await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            x,
            y,
            physical_width,
            physical_height,
            subpixel,
            make,
            model,
            transform,
        })
    }
}
