use super::ObjectInteface as XdgDecorationObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zxdg_decoration_manager_v1.get_toplevel_decoration`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="get_toplevel_decoration">
///       <description summary="create a new toplevel decoration object">
///         Create a new decoration object associated with the given toplevel.
///
///         For objects of version 1, creating an xdg_toplevel_decoration from an
///         xdg_toplevel which has a buffer attached or committed is a client
///         error, and any attempts by a client to attach or manipulate a buffer
///         prior to the first xdg_toplevel_decoration.configure event must also be
///         treated as errors.
///
///         For objects of version 2 or newer, creating an xdg_toplevel_decoration
///         from an xdg_toplevel which has a buffer attached or committed is
///         allowed. The initial decoration mode of the surface if a buffer is
///         already attached depends on whether a xdg_toplevel_decoration object
///         has been associated with the surface or not prior to this request.
///
///         If an xdg_toplevel_decoration was associated with the surface, then
///         destroyed without a surface commit, the previous decoration mode is
///         retained.
///
///         If no xdg_toplevel_decoration was associated with the surface prior to
///         this request, or if a surface commit has been performed after a previous
///         xdg_toplevel_decoration object associated with the surface was
///         destroyed, the decoration mode is assumed to be client-side.
///       </description>
///       <arg name="id" type="new_id" interface="zxdg_toplevel_decoration_v1" />
///       <arg name="toplevel" type="object" interface="xdg_toplevel" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZxdgDecorationManagerV1GetToplevelDecorationRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `id`.
    pub id: u32,
    /// Wire argument `toplevel`.
    pub toplevel: u32,
}
impl ZxdgDecorationManagerV1GetToplevelDecorationRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZxdgDecorationManagerV1GetToplevelDecorationRequest {
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
impl InterfaceReader for ZxdgDecorationManagerV1GetToplevelDecorationRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgDecoration(
                XdgDecorationObjectInteface::ZxdgDecorationManagerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = reader.next_u32().await?;
        let toplevel = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::XdgDecoration(
                XdgDecorationObjectInteface::ZxdgToplevelDecorationV1 {
                    version: interface.version().min(2),
                },
            );
            Self::reader_state(state)?
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            toplevel,
        })
    }
}
impl CommandWriter for ZxdgDecorationManagerV1GetToplevelDecorationRequest {
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
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "zxdg_decoration_manager_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::XdgDecoration(
                XdgDecorationObjectInteface::ZxdgDecorationManagerV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::XdgDecoration(
                XdgDecorationObjectInteface::ZxdgToplevelDecorationV1 {
                    version: interface.version().min(2),
                },
            );
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_u32(self.toplevel).await?;
        Ok(())
    }
}
