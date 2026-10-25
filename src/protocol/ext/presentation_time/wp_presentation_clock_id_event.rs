use crate::protocol::ClientState;
use crate::protocol::ext::presentation_time::ObjectInteface as PresentationTimeObjectInteface;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wp_presentation.clock_id`.
///
/// Opcode 0; available since version 1.
/// Arguments follow XML wire order. Receiver metadata and payload framing are checked.
/// Compositor resources, object arguments, and presentation policy belong to the caller.
/// ```xml
/// <event name="clock_id">
///       <description summary="clock ID for timestamps">
///         This event tells the client in which clock domain the
///         compositor interprets the timestamps used by the presentation
///         extension. This clock is called the presentation clock.
///
///         The compositor sends this event when the client binds to the
///         presentation interface. The presentation clock does not change
///         during the lifetime of the client connection.
///
///         The clock identifier is platform dependent. On POSIX platforms, the
///         identifier value is one of the clockid_t values accepted by
///         clock_gettime(). clock_gettime() is defined by POSIX.1-2001.
///
///         Timestamps in this clock domain are expressed as tv_sec_hi,
///         tv_sec_lo, tv_nsec triples, each component being an unsigned
///         32-bit value. Whole seconds are in tv_sec which is a 64-bit
///         value combined from tv_sec_hi and tv_sec_lo, and the
///         additional fractional part in tv_nsec as nanoseconds. Hence,
///         for valid timestamps tv_nsec must be in [0, 999999999].
///
///         Note that clock_id applies only to the presentation clock,
///         and implies nothing about e.g. the timestamps used in the
///         Wayland core protocol input events.
///
///         Compositors should prefer a clock which does not jump and is
///         not slewed e.g. by NTP. The absolute value of the clock is
///         irrelevant. Precision of one millisecond or better is
///         recommended. Clients must be able to query the current clock
///         value directly, not by asking the compositor.
///       </description>
///       <arg name="clk_id" type="uint" summary="platform clock identifier" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WpPresentationClockIdEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `clk_id`: platform clock identifier.
    pub clk_id: u32,
}
impl WpPresentationClockIdEvent {
    /// Opcode in the event namespace of `wp_presentation`.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpPresentationClockIdEvent {
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

impl InterfaceReader for WpPresentationClockIdEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::PresentationTime(PresentationTimeObjectInteface::WpPresentation { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let clk_id = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, clk_id })
    }
}

impl CommandWriter for WpPresentationClockIdEvent {
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
        Self::writer_event_interface(state, self.object_id, "wp_presentation", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.clk_id).await?;
        Ok(())
    }
}
