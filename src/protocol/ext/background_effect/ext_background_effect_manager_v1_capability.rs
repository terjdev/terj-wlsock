use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `ext_background_effect_manager_v1.capability`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="capability" bitfield="true">
///       <entry name="blur" value="1" summary="the compositor supports applying blur" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExtBackgroundEffectManagerV1Capability(pub u32);
impl ExtBackgroundEffectManagerV1Capability {
    /// the compositor supports applying blur; available since version 1.
    pub const BLUR: Self = Self(1);
}
impl std::ops::BitOr for ExtBackgroundEffectManagerV1Capability {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl std::ops::BitAnd for ExtBackgroundEffectManagerV1Capability {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl CommandReader<Self> for ExtBackgroundEffectManagerV1Capability {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ExtBackgroundEffectManagerV1Capability {
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
