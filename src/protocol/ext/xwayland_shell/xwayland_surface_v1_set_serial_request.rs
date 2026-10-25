use crate::protocol::ext::xwayland_shell::ObjectInteface as XwaylandShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xwayland_surface_v1.set_serial`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_serial">
///       <description summary="associates a Xwayland window to a wl_surface">
///         Associates an Xwayland window to a wl_surface.
///         The association state is double-buffered, see wl_surface.commit.
///
///         The `serial_lo` and `serial_hi` parameters specify a non-zero
///         monotonic serial number which is entirely unique and provided by the
///         Xwayland server equal to the serial value provided by a client message
///         with a message type of the `WL_SURFACE_SERIAL` atom on the X11 window
///         for this surface to be associated to.
///
///         The serial value in the `WL_SURFACE_SERIAL` client message is specified
///         as having the lo-bits specified in `l[0]` and the hi-bits specified
///         in `l[1]`.
///
///         If the serial value provided by `serial_lo` and `serial_hi` is not
///         valid, the `invalid_serial` protocol error will be raised.
///
///         An X11 window may be associated with multiple surfaces throughout its
///         lifespan. (eg. unmapping and remapping a window).
///
///         For each wl_surface, this state must not be committed more than once,
///         otherwise the `already_associated` protocol error will be raised.
///       </description>
///       <arg name="serial_lo" type="uint" summary="The lower 32-bits of the serial number associated with the X11 window" />
///       <arg name="serial_hi" type="uint" summary="The upper 32-bits of the serial number associated with the X11 window" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XwaylandSurfaceV1SetSerialRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// The lower 32-bits of the serial number associated with the X11 window.
    pub serial_lo: u32,
    /// The upper 32-bits of the serial number associated with the X11 window.
    pub serial_hi: u32,
}
impl XwaylandSurfaceV1SetSerialRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XwaylandSurfaceV1SetSerialRequest {
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
impl InterfaceReader for XwaylandSurfaceV1SetSerialRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XwaylandShell(XwaylandShellObjectInteface::XwaylandSurfaceV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let serial_lo = reader.next_u32().await?;
        let serial_hi = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            serial_lo,
            serial_hi,
        })
    }
}
impl CommandWriter for XwaylandSurfaceV1SetSerialRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(state, self.object_id, "xwayland_surface_v1", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial_lo).await?;
        writer.next_u32(self.serial_hi).await?;
        Ok(())
    }
}
