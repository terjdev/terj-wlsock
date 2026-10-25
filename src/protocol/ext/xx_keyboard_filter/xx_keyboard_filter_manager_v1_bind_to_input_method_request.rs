use super::ObjectInteface as XxKeyboardFilterObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_keyboard_filter_manager_v1.bind_to_input_method`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="bind_to_input_method">
///       <description summary="Bind a keyboard to an input method">
///         Bind a keyboard to an input method for the purpose of capturing key presses before they reach the text input client.
///
///         When a wl_keyboard is bound, the compositor must redirect to it the input events intended for the focused surface with text input enabled. The wl_keyboard instance receives no other events from then on.
///         See xx_keyboard_filter.filter.
///
///         For the bound wl_keyboard instance to intercept events, the following conditions must be fulfilled:
///         - there's a focused surface,
///         - the surface has an enabled text input object,
///         - the bound input method is active (for the meaning of "active", see input_method.activate, input_method.deactivate).
///
///         When those conditions are fulfilled, the compositor must start redirecting input events intended for the text input surface to the wl_keyboard bound with this request. Otherwise, the text input surface receives events without intercepting them.
///
///         Be aware that the text input client might use a wl_keyboard object(s) of different version(s) than the one used by the input method. The compositor should issue events as it would normally do for the versions in question. This protocol assumes that events to multiple keyboards of different protocol versions are equivalent.
///
///         Background:
///
///         Whenever the input method is activated, the compositor must start sending it keyboard events intended for the text-input client, so that the input method can be controlled using a keyboard.
///         Traditionally, from the user perspective, input methods receive keys as if they were an overlay: keys which are interesting to the input method gain a special input method meaning, all others work as usual.
///         The binding and the xx_keyboard_filter.filter request together make this possible by letting the input method indicate which events it is interested in.
///
///         Conceptually, when a wl_keyboard is bound to an input_method, the compositor prevents all keyboard events directed to the text input client from reaching it. They are delayed until the input method decides how to filter them using the xx_keyboard_filter.filter request.
///
///         Arguments:
///
///         The wl_keyboard must not be already bound to another interface.
///         The wl_keyboard must only receive events between committed .activate and .deactivate.
///
///         The surface argument represents an arbitrary wl_surface. When issuing wl_keyboard.enter and wl_keyboard.leave on the bound wl_keyboard, the compositor must replace the original surface argument with the one provided by the input method in this request.
///
///         Because the wl_keyboard.enter and wl_keyboard.leave events require a surface as the target, one must be provided even if the input method doesn't display one. A dummy one is sufficient. The provided wl_surface will not be used for any other purpose than explained above.
///
///         The surface must outlive the input method.
///
///         NOTE: This feature works much better with compositor-side key repeat introduced in wl_seat version 10. This protocol doesn't provide controls for filtering repeat key events generated client-side.
///         A compositor implementing this protocol should implement compositor-side key repeat.
///
///         This request takes effect immediately.
///
///         Attempting to bind a keyboard to an input method which is already bound must cause the already_bound error.
///         Attempting to bind a keyboard object which was already bound must cause the already_bound error.
///         Attempting to bind a keyboard object to an input method acting on a different seat must cause the wrong_seat error.
///
///         Once any of the bound objects are destroyed, the xx_keyboard_filter_v1 instance becomes disabled and it must ignore all following requests.
///
///         When the input method gets destroyed, the compositor must stop issuing events to the keyboard and ignore any further requests to xx_keyboard_filter, except xx_keyboard_filter.destroy.
///       </description>
///       <arg name="keyboard" type="object" interface="wl_keyboard" />
///       <arg name="input_method" type="object" interface="xx_input_method_v1" />
///       <arg name="surface" type="object" interface="wl_surface" />
///       <arg name="extensions" type="new_id" interface="xx_keyboard_filter_v1" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxKeyboardFilterManagerV1BindToInputMethodRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `keyboard`.
    pub keyboard: u32,
    /// Wire argument `input_method`.
    pub input_method: u32,
    /// Wire argument `surface`.
    pub surface: u32,
    /// Wire argument `extensions`.
    pub extensions: u32,
}
impl XxKeyboardFilterManagerV1BindToInputMethodRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxKeyboardFilterManagerV1BindToInputMethodRequest {
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
impl InterfaceReader for XxKeyboardFilterManagerV1BindToInputMethodRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxKeyboardFilter(
                XxKeyboardFilterObjectInteface::XxKeyboardFilterManagerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let keyboard = reader.next_u32().await?;
        let input_method = reader.next_u32().await?;
        let surface = reader.next_u32().await?;
        let extensions = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::XxKeyboardFilter(
                XxKeyboardFilterObjectInteface::XxKeyboardFilterV1 {
                    version: interface.version().min(1),
                },
            );
            Self::reader_state(state)?
                .register_object(extensions, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            keyboard,
            input_method,
            surface,
            extensions,
        })
    }
}
impl CommandWriter for XxKeyboardFilterManagerV1BindToInputMethodRequest {
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
            "xx_keyboard_filter_manager_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::XxKeyboardFilter(
                XxKeyboardFilterObjectInteface::XxKeyboardFilterManagerV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::XxKeyboardFilter(
                XxKeyboardFilterObjectInteface::XxKeyboardFilterV1 {
                    version: interface.version().min(1),
                },
            );
            Self::writer_state(state)?
                .register_object(self.extensions, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.keyboard).await?;
        writer.next_u32(self.input_method).await?;
        writer.next_u32(self.surface).await?;
        writer.next_u32(self.extensions).await?;
        Ok(())
    }
}
