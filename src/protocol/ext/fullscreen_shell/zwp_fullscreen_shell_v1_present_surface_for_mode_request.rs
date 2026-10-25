use super::ObjectInteface as FullscreenShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_fullscreen_shell_v1.present_surface_for_mode`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="present_surface_for_mode">
///       <description summary="present surface for display at a particular mode">
///     Presents a surface on the given output for a particular mode.
///
///     If the current size of the output differs from that of the surface,
///     the compositor will attempt to change the size of the output to
///     match the surface.  The result of the mode-switch operation will be
///     returned via the provided wl_fullscreen_shell_mode_feedback object.
///
///     If the current output mode matches the one requested or if the
///     compositor successfully switches the mode to match the surface,
///     then the mode_successful event will be sent and the output will
///     contain the contents of the given surface.  If the compositor
///     cannot match the output size to the surface size, the mode_failed
///     will be sent and the output will contain the contents of the
///     previously presented surface (if any).  If another surface is
///     presented on the given output before either of these has a chance
///     to happen, the present_cancelled event will be sent.
///
///     Due to race conditions and other issues unknown to the client, no
///     mode-switch operation is guaranteed to succeed.  However, if the
///     mode is one advertised by wl_output.mode or if the compositor
///     advertises the ARBITRARY_MODES capability, then the client should
///     expect that the mode-switch operation will usually succeed.
///
///     If the size of the presented surface changes, the resulting output
///     is undefined.  The compositor may attempt to change the output mode
///     to compensate.  However, there is no guarantee that a suitable mode
///     will be found and the client has no way to be notified of success
///     or failure.
///
///     The framerate parameter specifies the desired framerate for the
///     output in mHz.  The compositor is free to ignore this parameter.  A
///     value of 0 indicates that the client has no preference.
///
///     If the value of wl_output.scale differs from wl_surface.buffer_scale,
///     then the compositor may choose a mode that matches either the buffer
///     size or the surface size.  In either case, the surface will fill the
///     output.
///
///     This request gives the surface the role of a fullscreen shell surface.
///     If the surface already has another role, it raises a role protocol
///     error.
///       </description>
///       <arg name="surface" type="object" interface="wl_surface" />
///       <arg name="output" type="object" interface="wl_output" />
///       <arg name="framerate" type="int" />
///       <arg name="feedback" type="new_id" interface="zwp_fullscreen_shell_mode_feedback_v1" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpFullscreenShellV1PresentSurfaceForModeRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `surface`.
    pub surface: u32,
    /// Wire argument `output`.
    pub output: u32,
    /// Wire argument `framerate`.
    pub framerate: i32,
    /// Wire argument `feedback`.
    pub feedback: u32,
}
impl ZwpFullscreenShellV1PresentSurfaceForModeRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpFullscreenShellV1PresentSurfaceForModeRequest {
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
impl InterfaceReader for ZwpFullscreenShellV1PresentSurfaceForModeRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::FullscreenShell(
                FullscreenShellObjectInteface::ZwpFullscreenShellV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let surface = reader.next_u32().await?;
        let output = reader.next_u32().await?;
        let framerate = reader.next_i32().await?;
        let feedback = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::FullscreenShell(
                FullscreenShellObjectInteface::ZwpFullscreenShellModeFeedbackV1 {
                    version: interface.version().min(1),
                },
            );
            Self::reader_state(state)?
                .register_object(feedback, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            surface,
            output,
            framerate,
            feedback,
        })
    }
}
impl CommandWriter for ZwpFullscreenShellV1PresentSurfaceForModeRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "zwp_fullscreen_shell_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::FullscreenShell(
                FullscreenShellObjectInteface::ZwpFullscreenShellV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::FullscreenShell(
                FullscreenShellObjectInteface::ZwpFullscreenShellModeFeedbackV1 {
                    version: interface.version().min(1),
                },
            );
            Self::writer_state(state)?
                .register_object(self.feedback, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.surface).await?;
        writer.next_u32(self.output).await?;
        writer.next_i32(self.framerate).await?;
        writer.next_u32(self.feedback).await?;
        Ok(())
    }
}
