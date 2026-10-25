use crate::protocol::ClientState;
use crate::protocol::ext::presentation_time::ObjectInteface as PresentationTimeObjectInteface;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_presentation.feedback`.
///
/// Opcode 1; available since version 1.
/// Arguments follow XML wire order. Receiver metadata and payload framing are checked.
/// Compositor resources, object arguments, and presentation policy belong to the caller.
/// The feedback ID inherits the receiver version and is registered before payload writes.
/// A failed or cancelled write after registration requires closing the connection.
/// ```xml
/// <request name="feedback">
///       <description summary="request presentation feedback information">
///         Request presentation feedback for the current content submission
///         on the given surface. This creates a new presentation_feedback
///         object, which will deliver the feedback information once. If
///         multiple presentation_feedback objects are created for the same
///         submission, they will all deliver the same information.
///
///         For details on what information is returned, see the
///         presentation_feedback interface.
///       </description>
///       <arg name="surface" type="object" interface="wl_surface" summary="target surface" />
///       <arg name="callback" type="new_id" interface="wp_presentation_feedback" summary="new feedback object" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpPresentationFeedbackRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `surface`: target surface.
    /// Object implementing `wl_surface`.
    pub surface: u32,
    /// Argument `callback`: new feedback object.
    /// Object implementing `wp_presentation_feedback`.
    pub callback: u32,
}
impl WpPresentationFeedbackRequest {
    /// Opcode in the request namespace of `wp_presentation`.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpPresentationFeedbackRequest {
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

impl InterfaceReader for WpPresentationFeedbackRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::PresentationTime(PresentationTimeObjectInteface::WpPresentation { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let surface = reader.next_u32().await?;
        let callback = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let mut state = Self::reader_state(state)?;
            let new_interface =
                ObjectInteface::from_name("wp_presentation_feedback", interface.version())
                    .ok_or(FdReaderError::InvalidProtocol)?;
            state
                .register_object(callback, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            surface,
            callback,
        })
    }
}

impl CommandWriter for WpPresentationFeedbackRequest {
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
        let receiver_interface =
            Self::writer_request_interface(state, self.object_id, "wp_presentation", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let mut state = Self::writer_state(state)?;
            let new_interface =
                ObjectInteface::from_name("wp_presentation_feedback", receiver_interface.version())
                    .ok_or(FdWriterError::InvalidProtocol)?;
            state
                .register_object(self.callback, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.surface).await?;
        writer.next_u32(self.callback).await?;
        Ok(())
    }
}
