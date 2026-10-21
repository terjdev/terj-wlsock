use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_touch.down`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="down">
///       <description summary="touch down event and beginning of a touch sequence">
///         A new touch point has appeared on the surface. This touch point is
///         assigned a unique ID. Future events from this touch point reference
///         this ID. The ID ceases to be valid after a touch up event and may be
///         reused in the future.
///       </description>
///       <arg name="serial" type="uint" summary="serial number of the touch down event" />
///       <arg name="time" type="uint" summary="timestamp with millisecond granularity" />
///       <arg name="surface" type="object" interface="wl_surface" summary="surface touched" />
///       <arg name="id" type="int" summary="the unique ID of this touch point" />
///       <arg name="x" type="fixed" summary="surface-local x coordinate" />
///       <arg name="y" type="fixed" summary="surface-local y coordinate" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlTouchDownEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `serial`: serial number of the touch down event.
    pub serial: u32,
    /// Argument `time`: timestamp with millisecond granularity.
    pub time: u32,
    /// Argument `surface`: surface touched.
    /// Object implementing interface wl_surface.
    pub surface: u32,
    /// Argument `id`: the unique ID of this touch point.
    pub id: i32,
    /// Argument `x`: surface-local x coordinate.
    pub x: f64,
    /// Argument `y`: surface-local y coordinate.
    pub y: f64,
}
impl WlTouchDownEvent {
    /// Opcode in the event namespace of interface wl_touch.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlTouchDownEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4), Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::event_interface(state, self.object_id, "wl_touch", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.x) {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.y) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        writer.next_u32(self.time).await?;
        writer.next_u32(self.surface).await?;
        writer.next_i32(self.id).await?;
        writer.next_fixed(self.x).await?;
        writer.next_fixed(self.y).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlTouchDownEvent {
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
            "wl_touch",
            Self::SINCE,
        )?;
        let serial = reader.next_u32().await?;
        let time = reader.next_u32().await?;
        let surface = reader.next_u32().await?;
        let id = reader.next_i32().await?;
        let x = reader.next_fixed().await?;
        let y = reader.next_fixed().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            serial,
            time,
            surface,
            id,
            x,
            y,
        })
    }
}
