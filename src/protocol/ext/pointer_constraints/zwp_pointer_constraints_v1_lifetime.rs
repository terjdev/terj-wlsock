use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwp_pointer_constraints_v1.lifetime`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="lifetime">
///       <description summary="constraint lifetime">
///     These values represent different lifetime semantics. They are passed
///     as arguments to the factory requests to specify how the constraint
///     lifetimes should be managed.
///       </description>
///       <entry name="oneshot" value="1">
///     <description summary="the pointer constraint is defunct once deactivated">
///       A oneshot pointer constraint will never reactivate once it has been
///       deactivated. See the corresponding deactivation event
///       (wp_locked_pointer.unlocked and wp_confined_pointer.unconfined) for
///       details.
///     </description>
///       </entry>
///       <entry name="persistent" value="2">
///     <description summary="the pointer constraint may reactivate">
///       A persistent pointer constraint may again reactivate once it has
///       been deactivated. See the corresponding deactivation event
///       (wp_locked_pointer.unlocked and wp_confined_pointer.unconfined) for
///       details.
///     </description>
///       </entry>
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwpPointerConstraintsV1Lifetime(pub u32);
impl ZwpPointerConstraintsV1Lifetime {
    /// the pointer constraint is defunct once deactivated; available since version 1.
    pub const ONESHOT: Self = Self(1);
    /// the pointer constraint may reactivate; available since version 1.
    pub const PERSISTENT: Self = Self(2);
}
impl CommandReader<Self> for ZwpPointerConstraintsV1Lifetime {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwpPointerConstraintsV1Lifetime {
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
