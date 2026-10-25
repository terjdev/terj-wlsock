use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwp_text_input_v1.content_purpose`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="content_purpose">
///       <description summary="content purpose">
///     The content purpose allows to specify the primary purpose of a text
///     input.
///
///     This allows an input method to show special purpose input panels with
///     extra characters or to disallow some characters.
///       </description>
///       <entry name="normal" value="0" summary="default input, allowing all characters" />
///       <entry name="alpha" value="1" summary="allow only alphabetic characters" />
///       <entry name="digits" value="2" summary="allow only digits" />
///       <entry name="number" value="3" summary="input a number (including decimal separator and sign)" />
///       <entry name="phone" value="4" summary="input a phone number" />
///       <entry name="url" value="5" summary="input an URL" />
///       <entry name="email" value="6" summary="input an email address" />
///       <entry name="name" value="7" summary="input a name of a person" />
///       <entry name="password" value="8" summary="input a password (combine with password or sensitive_data hint)" />
///       <entry name="date" value="9" summary="input a date" />
///       <entry name="time" value="10" summary="input a time" />
///       <entry name="datetime" value="11" summary="input a date and time" />
///       <entry name="terminal" value="12" summary="input for a terminal" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwpTextInputV1ContentPurpose(pub u32);
impl ZwpTextInputV1ContentPurpose {
    /// default input, allowing all characters; available since version 1.
    pub const NORMAL: Self = Self(0);
    /// allow only alphabetic characters; available since version 1.
    pub const ALPHA: Self = Self(1);
    /// allow only digits; available since version 1.
    pub const DIGITS: Self = Self(2);
    /// input a number (including decimal separator and sign); available since version 1.
    pub const NUMBER: Self = Self(3);
    /// input a phone number; available since version 1.
    pub const PHONE: Self = Self(4);
    /// input an URL; available since version 1.
    pub const URL: Self = Self(5);
    /// input an email address; available since version 1.
    pub const EMAIL: Self = Self(6);
    /// input a name of a person; available since version 1.
    pub const NAME: Self = Self(7);
    /// input a password (combine with password or sensitive_data hint); available since version 1.
    pub const PASSWORD: Self = Self(8);
    /// input a date; available since version 1.
    pub const DATE: Self = Self(9);
    /// input a time; available since version 1.
    pub const TIME: Self = Self(10);
    /// input a date and time; available since version 1.
    pub const DATETIME: Self = Self(11);
    /// input for a terminal; available since version 1.
    pub const TERMINAL: Self = Self(12);
}
impl CommandReader<Self> for ZwpTextInputV1ContentPurpose {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwpTextInputV1ContentPurpose {
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
