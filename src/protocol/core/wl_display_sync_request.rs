use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_display.sync`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="sync">
///       <description summary="asynchronous roundtrip">
///         The sync request asks the server to emit the 'done' event
///         on the returned wl_callback object.  Since requests are
///         handled in-order and events are delivered in-order, this can
///         be used as a barrier to ensure all previous requests and the
///         resulting events have been handled.
///
///         The object returned by this request will be destroyed by the
///         compositor after the callback is fired and as such the client must not
///         attempt to use it after that point.
///
///         The callback_data passed in the callback is undefined and should be ignored.
///       </description>
///       <arg name="callback" type="new_id" interface="wl_callback" summary="callback object for the sync request" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlDisplaySyncRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `callback`: callback object for the sync request.
    /// Object implementing interface wl_callback.
    pub callback: u32,
}
impl WlDisplaySyncRequest {
    /// Opcode in the request namespace of interface wl_display.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlDisplaySyncRequest {
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
            "wl_display",
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
