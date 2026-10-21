use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_surface.get_release`.
///
/// Opcode 11; available since version 7. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="get_release" since="7">
///       <description summary="get a release callback">
///         Create a callback for the release of the buffer attached by the client
///         with wl_surface.attach.
///
///         The compositor will release the buffer when it has finished its usage of
///         the underlying storage for the relevant commit. Once the client receives
///         this event, and assuming the associated buffer is not pending release
///         from other wl_surface.commit requests, the client can safely re-use the
///         buffer.
///
///         Release callbacks are double-buffered state, and will be associated
///         with the pending buffer at wl_surface.commit time.
///
///         The callback_data passed in the wl_callback.done event is unused and
///         is always zero.
///
///         Sending this request without attaching a non-null buffer in the same
///         content update is a protocol error. The compositor will send the
///         no_buffer error in this case.
///       </description>
///       <arg name="callback" type="new_id" interface="wl_callback" summary="callback object for the release" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlSurfaceGetReleaseRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `callback`: callback object for the release.
    /// Object implementing interface wl_callback.
    pub callback: u32,
}
impl WlSurfaceGetReleaseRequest {
    /// Opcode in the request namespace of interface wl_surface.
    pub const OPCODE: u16 = 11;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 7;
}
impl CommandReader<Self> for WlSurfaceGetReleaseRequest {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        let receiver_interface = Self::request_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_surface",
            Self::SINCE,
        )?;
        let callback = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let mut state = Self::read_state(state)?;
            let new_interface =
                ObjectInteface::from_name("wl_callback", receiver_interface.version().min(1))
                    .ok_or(FdReaderError::InvalidProtocol)?;
            state
                .register_object(callback, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            callback,
        })
    }
}
