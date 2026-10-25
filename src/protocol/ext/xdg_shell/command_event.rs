use super::{
    XdgPopupConfigureEvent, XdgPopupPopupDoneEvent, XdgPopupRepositionedEvent,
    XdgSurfaceConfigureEvent, XdgToplevelCloseEvent, XdgToplevelConfigureBoundsEvent,
    XdgToplevelConfigureEvent, XdgToplevelWmCapabilitiesEvent, XdgWmBasePingEvent,
};
use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches XDG shell events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [XdgWmBasePingEvent].
    XdgWmBasePing(XdgWmBasePingEvent),
    /// Message [XdgSurfaceConfigureEvent].
    XdgSurfaceConfigure(XdgSurfaceConfigureEvent),
    /// Message [XdgToplevelConfigureEvent].
    XdgToplevelConfigure(XdgToplevelConfigureEvent),
    /// Message [XdgToplevelCloseEvent].
    XdgToplevelClose(XdgToplevelCloseEvent),
    /// Message [XdgToplevelConfigureBoundsEvent].
    XdgToplevelConfigureBounds(XdgToplevelConfigureBoundsEvent),
    /// Message [XdgToplevelWmCapabilitiesEvent].
    XdgToplevelWmCapabilities(XdgToplevelWmCapabilitiesEvent),
    /// Message [XdgPopupConfigureEvent].
    XdgPopupConfigure(XdgPopupConfigureEvent),
    /// Message [XdgPopupPopupDoneEvent].
    XdgPopupPopupDone(XdgPopupPopupDoneEvent),
    /// Message [XdgPopupRepositionedEvent].
    XdgPopupRepositioned(XdgPopupRepositionedEvent),
}
impl CommandReader<Self> for CommandEvent {
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
impl InterfaceReader for CommandEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        match interface {
            ObjectInteface::XdgShell(XdgShellObjectInteface::XdgWmBase { .. }) => match opcode {
                XdgWmBasePingEvent::OPCODE => Ok(Self::XdgWmBasePing(
                    XdgWmBasePingEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XdgShell(XdgShellObjectInteface::XdgSurface { .. }) => match opcode {
                XdgSurfaceConfigureEvent::OPCODE => Ok(Self::XdgSurfaceConfigure(
                    XdgSurfaceConfigureEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XdgShell(XdgShellObjectInteface::XdgToplevel { .. }) => match opcode {
                XdgToplevelConfigureEvent::OPCODE => Ok(Self::XdgToplevelConfigure(
                    XdgToplevelConfigureEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgToplevelCloseEvent::OPCODE => Ok(Self::XdgToplevelClose(
                    XdgToplevelCloseEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgToplevelConfigureBoundsEvent::OPCODE => Ok(Self::XdgToplevelConfigureBounds(
                    XdgToplevelConfigureBoundsEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgToplevelWmCapabilitiesEvent::OPCODE => Ok(Self::XdgToplevelWmCapabilities(
                    XdgToplevelWmCapabilitiesEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XdgShell(XdgShellObjectInteface::XdgPopup { .. }) => match opcode {
                XdgPopupConfigureEvent::OPCODE => Ok(Self::XdgPopupConfigure(
                    XdgPopupConfigureEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgPopupPopupDoneEvent::OPCODE => Ok(Self::XdgPopupPopupDone(
                    XdgPopupPopupDoneEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgPopupRepositionedEvent::OPCODE => Ok(Self::XdgPopupRepositioned(
                    XdgPopupRepositionedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            _ => Err(FdReaderError::InvalidProtocol),
        }
    }
}
impl CommandWriter for CommandEvent {
    fn object_id(&self) -> u32 {
        match self {
            Self::XdgWmBasePing(message) => message.object_id(),
            Self::XdgSurfaceConfigure(message) => message.object_id(),
            Self::XdgToplevelConfigure(message) => message.object_id(),
            Self::XdgToplevelClose(message) => message.object_id(),
            Self::XdgToplevelConfigureBounds(message) => message.object_id(),
            Self::XdgToplevelWmCapabilities(message) => message.object_id(),
            Self::XdgPopupConfigure(message) => message.object_id(),
            Self::XdgPopupPopupDone(message) => message.object_id(),
            Self::XdgPopupRepositioned(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XdgWmBasePing(message) => message.opcode(),
            Self::XdgSurfaceConfigure(message) => message.opcode(),
            Self::XdgToplevelConfigure(message) => message.opcode(),
            Self::XdgToplevelClose(message) => message.opcode(),
            Self::XdgToplevelConfigureBounds(message) => message.opcode(),
            Self::XdgToplevelWmCapabilities(message) => message.opcode(),
            Self::XdgPopupConfigure(message) => message.opcode(),
            Self::XdgPopupPopupDone(message) => message.opcode(),
            Self::XdgPopupRepositioned(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XdgWmBasePing(message) => message.payload_size(),
            Self::XdgSurfaceConfigure(message) => message.payload_size(),
            Self::XdgToplevelConfigure(message) => message.payload_size(),
            Self::XdgToplevelClose(message) => message.payload_size(),
            Self::XdgToplevelConfigureBounds(message) => message.payload_size(),
            Self::XdgToplevelWmCapabilities(message) => message.payload_size(),
            Self::XdgPopupConfigure(message) => message.payload_size(),
            Self::XdgPopupPopupDone(message) => message.payload_size(),
            Self::XdgPopupRepositioned(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XdgWmBasePing(message) => message.write(writer, state).await,
            Self::XdgSurfaceConfigure(message) => message.write(writer, state).await,
            Self::XdgToplevelConfigure(message) => message.write(writer, state).await,
            Self::XdgToplevelClose(message) => message.write(writer, state).await,
            Self::XdgToplevelConfigureBounds(message) => message.write(writer, state).await,
            Self::XdgToplevelWmCapabilities(message) => message.write(writer, state).await,
            Self::XdgPopupConfigure(message) => message.write(writer, state).await,
            Self::XdgPopupPopupDone(message) => message.write(writer, state).await,
            Self::XdgPopupRepositioned(message) => message.write(writer, state).await,
        }
    }
}
