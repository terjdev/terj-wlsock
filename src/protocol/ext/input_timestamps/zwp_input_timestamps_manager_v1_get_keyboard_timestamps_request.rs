use super::ObjectInteface as InputTimestampsObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_input_timestamps_manager_v1.get_keyboard_timestamps`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="get_keyboard_timestamps">
///       <description summary="subscribe to high-resolution keyboard timestamp events">
///         Creates a new input timestamps object that represents a subscription
///         to high-resolution timestamp events for all wl_keyboard events that
///         carry a timestamp.
///
///         If the associated wl_keyboard object is invalidated, either through
///         client action (e.g. release) or server-side changes, the input
///         timestamps object becomes inert and the client should destroy it
///         by calling zwp_input_timestamps_v1.destroy.
///       </description>
///       <arg name="id" type="new_id" interface="zwp_input_timestamps_v1" />
///       <arg name="keyboard" type="object" interface="wl_keyboard" summary="the wl_keyboard object for which to get timestamp events" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpInputTimestampsManagerV1GetKeyboardTimestampsRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `id`.
    pub id: u32,
    /// the wl_keyboard object for which to get timestamp events.
    pub keyboard: u32,
}
impl ZwpInputTimestampsManagerV1GetKeyboardTimestampsRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpInputTimestampsManagerV1GetKeyboardTimestampsRequest {
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
impl InterfaceReader for ZwpInputTimestampsManagerV1GetKeyboardTimestampsRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::InputTimestamps(
                InputTimestampsObjectInteface::ZwpInputTimestampsManagerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = reader.next_u32().await?;
        let keyboard = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::InputTimestamps(
                InputTimestampsObjectInteface::ZwpInputTimestampsV1 {
                    version: interface.version().min(1),
                },
            );
            Self::reader_state(state)?
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            keyboard,
        })
    }
}
impl CommandWriter for ZwpInputTimestampsManagerV1GetKeyboardTimestampsRequest {
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
            "zwp_input_timestamps_manager_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::InputTimestamps(
                InputTimestampsObjectInteface::ZwpInputTimestampsManagerV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::InputTimestamps(
                InputTimestampsObjectInteface::ZwpInputTimestampsV1 {
                    version: interface.version().min(1),
                },
            );
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_u32(self.keyboard).await?;
        Ok(())
    }
}
