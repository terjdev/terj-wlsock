use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `ext_image_copy_capture_frame_v1.failure_reason`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="failure_reason">
///       <entry name="unknown" value="0">
///         <description summary="unknown runtime error">
///           An unspecified runtime error has occurred. The client may retry.
///         </description>
///       </entry>
///       <entry name="buffer_constraints" value="1">
///         <description summary="buffer constraints mismatch">
///           The buffer submitted by the client doesn't match the latest session
///           constraints. The client should re-allocate its buffers and retry.
///         </description>
///       </entry>
///       <entry name="stopped" value="2">
///         <description summary="session is no longer available">
///           The session has stopped. See ext_image_copy_capture_session_v1.stopped.
///         </description>
///       </entry>
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExtImageCopyCaptureFrameV1FailureReason(pub u32);
impl ExtImageCopyCaptureFrameV1FailureReason {
    /// unknown runtime error; available since version 1.
    pub const UNKNOWN: Self = Self(0);
    /// buffer constraints mismatch; available since version 1.
    pub const BUFFER_CONSTRAINTS: Self = Self(1);
    /// session is no longer available; available since version 1.
    pub const STOPPED: Self = Self(2);
}
impl CommandReader<Self> for ExtImageCopyCaptureFrameV1FailureReason {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ExtImageCopyCaptureFrameV1FailureReason {
    fn object_id(&self) -> u32 {
        0
    }
    fn opcode(&self) -> u16 {
        0
    }
    fn payload_size(&self) -> usize {
        4
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        writer.next_u32(self.0).await
    }
}
