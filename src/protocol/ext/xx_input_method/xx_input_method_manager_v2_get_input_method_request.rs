use super::ObjectInteface as XxInputMethodObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_input_method_manager_v2.get_input_method`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="get_input_method">
///       <description summary="request an input method object">
///         Request a new input xx_input_method_v1 object associated with a given
///         seat.
///       </description>
///       <arg name="seat" type="object" interface="wl_seat" />
///       <arg name="input_method" type="new_id" interface="xx_input_method_v1" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxInputMethodManagerV2GetInputMethodRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `seat`.
    pub seat: u32,
    /// Wire argument `input_method`.
    pub input_method: u32,
}
impl XxInputMethodManagerV2GetInputMethodRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxInputMethodManagerV2GetInputMethodRequest {
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
impl InterfaceReader for XxInputMethodManagerV2GetInputMethodRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxInputMethod(
                XxInputMethodObjectInteface::XxInputMethodManagerV2 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let seat = reader.next_u32().await?;
        let input_method = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::XxInputMethod(XxInputMethodObjectInteface::XxInputMethodV1 {
                    version: interface.version().min(4),
                });
            Self::reader_state(state)?
                .register_object(input_method, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            seat,
            input_method,
        })
    }
}
impl CommandWriter for XxInputMethodManagerV2GetInputMethodRequest {
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
            "xx_input_method_manager_v2",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::XxInputMethod(
                XxInputMethodObjectInteface::XxInputMethodManagerV2 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::XxInputMethod(XxInputMethodObjectInteface::XxInputMethodV1 {
                    version: interface.version().min(4),
                });
            Self::writer_state(state)?
                .register_object(self.input_method, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.seat).await?;
        writer.next_u32(self.input_method).await?;
        Ok(())
    }
}
