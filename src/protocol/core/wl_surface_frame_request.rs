use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_surface.frame`.
///
/// Opcode 3; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="frame">
///       <description summary="request a frame throttling hint">
///         Request a notification when it is a good time to start drawing a new
///         frame, by creating a frame callback. This is useful for throttling
///         redrawing operations, and driving animations.
///
///         When a client is animating on a wl_surface, it can use the 'frame'
///         request to get notified when it is a good time to draw and commit the
///         next frame of animation. If the client commits an update earlier than
///         that, it is likely that some updates will not make it to the display,
///         and the client is wasting resources by drawing too often.
///
///         The frame request will take effect on the next wl_surface.commit.
///         The notification will only be posted for one frame unless
///         requested again. For a wl_surface, the notifications are posted in
///         the order the frame requests were committed.
///
///         The server must send the notifications so that a client
///         will not send excessive updates, while still allowing
///         the highest possible update rate for clients that wait for the reply
///         before drawing again. The server should give some time for the client
///         to draw and commit after sending the frame callback events to let it
///         hit the next output refresh.
///
///         A server should avoid signaling the frame callbacks if the
///         surface is not visible in any way, e.g. the surface is off-screen,
///         or completely obscured by other opaque surfaces.
///
///         The object returned by this request will be destroyed by the
///         compositor after the callback is fired and as such the client must not
///         attempt to use it after that point.
///
///         The callback_data passed in the callback is the current time, in
///         milliseconds, with an undefined base.
///       </description>
///       <arg name="callback" type="new_id" interface="wl_callback" summary="callback object for the frame request" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlSurfaceFrameRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `callback`: callback object for the frame request.
    /// Object implementing interface wl_callback.
    pub callback: u32,
}
impl WlSurfaceFrameRequest {
    /// Opcode in the request namespace of interface wl_surface.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlSurfaceFrameRequest {
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
