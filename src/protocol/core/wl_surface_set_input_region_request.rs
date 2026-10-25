use crate::protocol::ClientState;
use crate::protocol::core::ObjectInteface as CoreObjectInteface;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wl_surface.set_input_region`.
///
/// Opcode 5; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire framing and receiver metadata; object semantics belong to the compositor.
/// The caller handles compositor policy and message contents.
/// Client encoding checks the complete payload quota before writing arguments.
/// New object IDs are registered before payload I/O; client IDs remain reserved
/// after destruction until the server acknowledges them with delete_id.
/// ```xml
/// <request name="set_input_region">
///       <description summary="set input region">
///         This request sets the region of the surface that can receive
///         pointer and touch events.
///
///         Input events happening outside of this region will try the next
///         surface in the server surface stack. The compositor ignores the
///         parts of the input region that fall outside of the surface.
///
///         The input region is specified in surface-local coordinates.
///
///         Input region is double-buffered state, see wl_surface.commit.
///
///         wl_surface.set_input_region changes the pending input region.
///         wl_surface.commit copies the pending region to the current region.
///         Otherwise the pending and current regions are never changed,
///         except cursor and icon surfaces are special cases, see
///         wl_pointer.set_cursor and wl_data_device.start_drag.
///
///         The initial value for an input region is infinite. That means the
///         whole surface will accept input. Setting the pending input region
///         has copy semantics, and the wl_region object can be destroyed
///         immediately. A NULL wl_region causes the input region to be set
///         to infinite.
///       </description>
///       <arg name="region" type="object" interface="wl_region" allow-null="true" summary="input region of the surface" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlSurfaceSetInputRegionRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `region`: input region of the surface.
    /// The protocol permits NULL.
    /// Object implementing interface wl_region.
    pub region: Option<u32>,
}
impl WlSurfaceSetInputRegionRequest {
    /// Opcode in the request namespace of interface wl_surface.
    pub const OPCODE: u16 = 5;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlSurfaceSetInputRegionRequest {
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

impl InterfaceReader for WlSurfaceSetInputRegionRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Core(CoreObjectInteface::WlSurface { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let region = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, region })
    }
}

impl CommandWriter for WlSurfaceSetInputRegionRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(state, self.object_id, "wl_surface", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.region.unwrap_or(0)).await?;
        Ok(())
    }
}
