use crate::protocol::ClientState;
use crate::protocol::InterfaceReader;
use crate::protocol::{CommandReader, CommandReaderExt, ObjectInteface};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wl_display.sync`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire framing and receiver metadata; object semantics belong to the compositor.
/// The caller handles compositor policy and message contents.
/// Client encoding checks the complete payload quota before writing arguments.
/// New object IDs are registered before payload I/O; client IDs remain reserved
/// after destruction until the server acknowledges them with delete_id.
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
        let interface = Self::reader_state(state)?
            .object_interface(object_id)
            .ok_or(FdReaderError::InvalidProtocol)?;
        Self::read_with_interface(object_id, opcode, interface, reader, state).await
    }
}

impl InterfaceReader for WlDisplaySyncRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(interface, ObjectInteface::WlDisplay { .. })
            || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let receiver_interface = interface;
        let callback = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let mut state = Self::reader_state(state)?;
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

impl CommandWriter for WlDisplaySyncRequest {
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
        let receiver_interface =
            Self::writer_request_interface(state, self.object_id, "wl_display", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let mut state = Self::writer_state(state)?;
            let new_interface =
                ObjectInteface::from_name("wl_callback", receiver_interface.version().min(1))
                    .ok_or(FdWriterError::InvalidProtocol)?;
            state
                .register_object(self.callback, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.callback).await?;
        Ok(())
    }
}
