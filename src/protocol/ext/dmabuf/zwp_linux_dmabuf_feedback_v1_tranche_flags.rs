use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwp_linux_dmabuf_feedback_v1.tranche_flags`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="tranche_flags" bitfield="true">
///       <entry name="scanout" value="1" since="4">
///         <description summary="direct scan-out tranche">
///           The scanout flag is a hint that direct scan-out may be attempted by
///           the compositor on the target device if the client appropriately
///           allocates a buffer. How to allocate a buffer that can be scanned out
///           on the target device is implementation-defined.
///         </description>
///       </entry>
///       <entry name="sampling" value="2" since="6">
///         <description summary="sampling tranche">
///           The sampling flag describes that the compositor is able to efficiently
///           sample from buffers imported to the target device if the client
///           appropriately allocates a buffer. How to allocate a buffer that can be
///           efficiently sampled on the target device is implementation defined.
///         </description>
///       </entry>
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwpLinuxDmabufFeedbackV1TrancheFlags(pub u32);
impl ZwpLinuxDmabufFeedbackV1TrancheFlags {
    /// direct scan-out tranche; available since version 4.
    pub const SCANOUT: Self = Self(1);
    /// sampling tranche; available since version 6.
    pub const SAMPLING: Self = Self(2);
}
impl std::ops::BitOr for ZwpLinuxDmabufFeedbackV1TrancheFlags {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl std::ops::BitAnd for ZwpLinuxDmabufFeedbackV1TrancheFlags {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl CommandReader<Self> for ZwpLinuxDmabufFeedbackV1TrancheFlags {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwpLinuxDmabufFeedbackV1TrancheFlags {
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
