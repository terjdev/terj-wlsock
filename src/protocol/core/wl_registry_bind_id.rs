use crate::protocol::{ClientState, CommandWriter, CommandWriterExt, ObjectInteface};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Represents the dynamic new_id argument of wl_registry.bind.
///
/// On the wire, an untyped new_id expands into an interface name,
/// version, and new identifier. This is a nested bind argument, not a standalone message.
/// Encoding validates its values but does not register the object; the enclosing
/// bind request registers it before any payload I/O.
/// ```xml
/// <arg name="id" type="new_id" summary="bound object" />
/// ```
#[derive(Debug)]
pub struct WlRegistryBindId {
    /// Interface name encoded as a Wayland string.
    pub interface: String,
    /// Requested version of the advertised interface.
    pub version: u32,
    /// New object identifier in the client-allocated range.
    pub id: u32,
}

impl CommandWriter for WlRegistryBindId {
    fn object_id(&self) -> u32 {
        0
    }
    fn opcode(&self) -> u16 {
        0
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[
            FdWriter::encoded_string_size(Some(&self.interface)),
            Some(4),
            Some(4),
        ])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        if self.payload_size() != writer.remaining_quota()
            || !FdWriter::is_valid_string(&self.interface)
            || ObjectInteface::from_name(&self.interface, self.version).is_none()
            || !(2..0xff000000).contains(&self.id)
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.interface).await?;
        writer.next_u32(self.version).await?;
        writer.next_u32(self.id).await?;
        Ok(())
    }
}
