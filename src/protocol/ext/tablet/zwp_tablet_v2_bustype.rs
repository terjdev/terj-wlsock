use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwp_tablet_v2.bustype`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="bustype" since="2">
///       <description summary="bus type ">
///     Describes the bus types this tablet is connected to.
///       </description>
///       <entry name="usb" value="3" summary="USB" />
///       <entry name="bluetooth" value="5" summary="Bluetooth" />
///       <entry name="virtual" value="6" summary="Virtual" />
///       <entry name="serial" value="17" summary="Serial" />
///       <entry name="i2c" value="24" summary="I2C" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwpTabletV2Bustype(pub u32);
impl ZwpTabletV2Bustype {
    /// USB; available since version 2.
    pub const USB: Self = Self(3);
    /// Bluetooth; available since version 2.
    pub const BLUETOOTH: Self = Self(5);
    /// Virtual; available since version 2.
    pub const VIRTUAL: Self = Self(6);
    /// Serial; available since version 2.
    pub const SERIAL: Self = Self(17);
    /// I2C; available since version 2.
    pub const I2C: Self = Self(24);
}
impl CommandReader<Self> for ZwpTabletV2Bustype {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwpTabletV2Bustype {
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
