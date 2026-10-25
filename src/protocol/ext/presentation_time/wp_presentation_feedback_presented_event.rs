use super::WpPresentationFeedbackKind;
use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wp_presentation_feedback.presented`.
///
/// Opcode 1; available since version 1.
/// Arguments follow XML wire order. Receiver metadata and payload framing are checked.
/// Compositor resources, object arguments, and presentation policy belong to the caller.
/// Ends the receiver lifetime; client IDs remain reserved until `wl_display.delete_id`.
/// ```xml
/// <event name="presented" type="destructor">
///       <description summary="the content update was displayed">
///         The associated content update was displayed to the user at the
///         indicated time (tv_sec_hi/lo, tv_nsec). For the interpretation of
///         the timestamp, see presentation.clock_id event.
///
///         The timestamp corresponds to the time when the content update
///         turned into light the first time on the surface's main output.
///         Compositors may approximate this from the framebuffer flip
///         completion events from the system, and the latency of the
///         physical display path if known.
///
///         This event is preceded by all related sync_output events
///         telling which output's refresh cycle the feedback corresponds
///         to, i.e. the main output for the surface. Compositors are
///         recommended to choose the output containing the largest part
///         of the wl_surface, or keeping the output they previously
///         chose. Having a stable presentation output association helps
///         clients predict future output refreshes (vblank).
///
///         The 'refresh' argument gives the compositor's prediction of how
///         many nanoseconds after tv_sec, tv_nsec the very next output
///         refresh may occur. This is to further aid clients in
///         predicting future refreshes, i.e., estimating the timestamps
///         targeting the next few vblanks. If such prediction cannot
///         usefully be done, the argument is zero.
///
///         For version 2 and later, if the output does not have a constant
///         refresh rate, explicit video mode switches excluded, then the
///         refresh argument must be either an appropriate rate picked by the
///         compositor (e.g. fastest rate), or 0 if no such rate exists.
///         For version 1, if the output does not have a constant refresh rate,
///         the refresh argument must be zero.
///
///         If the 'fixed_rate' flag is set, then the refresh argument is
///         the output's constant refresh rate. If the 'variable_rate' flag
///         is set, then the refresh argument is the output's fastest refresh
///         rate.
///
///         The 64-bit value combined from seq_hi and seq_lo is the value
///         of the output's vertical retrace counter when the content
///         update was first scanned out to the display. This value must
///         be compatible with the definition of MSC in
///         GLX_OML_sync_control specification. Note, that if the display
///         path has a non-zero latency, the time instant specified by
///         this counter may differ from the timestamp's.
///
///         If the output does not have a concept of vertical retrace or a
///         refresh cycle, or the output device is self-refreshing without
///         a way to query the refresh count, then the arguments seq_hi
///         and seq_lo must be zero.
///       </description>
///       <arg name="tv_sec_hi" type="uint" summary="high 32 bits of the seconds part of the presentation timestamp" />
///       <arg name="tv_sec_lo" type="uint" summary="low 32 bits of the seconds part of the presentation timestamp" />
///       <arg name="tv_nsec" type="uint" summary="nanoseconds part of the presentation timestamp" />
///       <arg name="refresh" type="uint" summary="nanoseconds till next refresh" />
///       <arg name="seq_hi" type="uint" summary="high 32 bits of refresh counter" />
///       <arg name="seq_lo" type="uint" summary="low 32 bits of refresh counter" />
///       <arg name="flags" type="uint" enum="kind" summary="combination of 'kind' values" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WpPresentationFeedbackPresentedEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `tv_sec_hi`: high 32 bits of the seconds part of the presentation timestamp.
    pub tv_sec_hi: u32,
    /// Argument `tv_sec_lo`: low 32 bits of the seconds part of the presentation timestamp.
    pub tv_sec_lo: u32,
    /// Argument `tv_nsec`: nanoseconds part of the presentation timestamp.
    pub tv_nsec: u32,
    /// Argument `refresh`: nanoseconds till next refresh.
    pub refresh: u32,
    /// Argument `seq_hi`: high 32 bits of refresh counter.
    pub seq_hi: u32,
    /// Argument `seq_lo`: low 32 bits of refresh counter.
    pub seq_lo: u32,
    /// Argument `flags`: combination of 'kind' values.
    pub flags: WpPresentationFeedbackKind,
}
impl WpPresentationFeedbackPresentedEvent {
    /// Opcode in the event namespace of `wp_presentation_feedback`.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpPresentationFeedbackPresentedEvent {
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

impl InterfaceReader for WpPresentationFeedbackPresentedEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(interface, ObjectInteface::WpPresentationFeedback { .. })
            || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let tv_sec_hi = reader.next_u32().await?;
        let tv_sec_lo = reader.next_u32().await?;
        let tv_nsec = reader.next_u32().await?;
        let refresh = reader.next_u32().await?;
        let seq_hi = reader.next_u32().await?;
        let seq_lo = reader.next_u32().await?;
        let flags = WpPresentationFeedbackKind(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Self::reader_state(state)?.release_server_id(object_id);
        Ok(Self {
            object_id,
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
            refresh,
            seq_hi,
            seq_lo,
            flags,
        })
    }
}

impl CommandWriter for WpPresentationFeedbackPresentedEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[
            Some(4),
            Some(4),
            Some(4),
            Some(4),
            Some(4),
            Some(4),
            Some(4),
        ])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_event_interface(
            state,
            self.object_id,
            "wp_presentation_feedback",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.tv_sec_hi).await?;
        writer.next_u32(self.tv_sec_lo).await?;
        writer.next_u32(self.tv_nsec).await?;
        writer.next_u32(self.refresh).await?;
        writer.next_u32(self.seq_hi).await?;
        writer.next_u32(self.seq_lo).await?;
        writer.next_u32(self.flags.0).await?;
        Self::writer_state(state)?.release_server_id(self.object_id);
        Ok(())
    }
}
