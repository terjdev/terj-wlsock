use crate::protocol::ClientState;
use crate::protocol::CommandWriter;
use crate::protocol::core::*;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Contains every core-protocol request and delegates its encoding and decoding.
///
/// Collects the request elements in [Wayland XML](https://gitlab.freedesktop.org/wayland/wayland/-/blob/main/protocol/wayland.xml).
/// One source element is shown below:
/// ```xml
/// <request name="sync">
///       <description summary="asynchronous roundtrip">
///         The sync request asks the server to emit the 'done' event
///         on the returned wl_callback object.  Since requests are
///         handled in-order and events are delivered in-order, this can
///         be used as a barrier to ensure all previous requests and the
///         resulting events have been handled.
///
///         The object returned by this request will be destroyed by the
///         compositor after the callback is fired and as such the client must not
///         attempt to use it after that point.
///
///         The callback_data passed in the callback is undefined and should be ignored.
///       </description>
///       <arg name="callback" type="new_id" interface="wl_callback" summary="callback object for the sync request" />
///     </request>
/// ```
/// Each message structure documents its complete XML definition.
#[derive(Debug)]
pub enum CommandRequest {
    /// Request [WlDisplaySyncRequest].
    WlDisplaySync(WlDisplaySyncRequest),
    /// Request [WlDisplayGetRegistryRequest].
    WlDisplayGetRegistry(WlDisplayGetRegistryRequest),
    /// Request [WlRegistryBindRequest].
    WlRegistryBind(WlRegistryBindRequest),
    /// Request [WlCompositorCreateSurfaceRequest].
    WlCompositorCreateSurface(WlCompositorCreateSurfaceRequest),
    /// Request [WlCompositorCreateRegionRequest].
    WlCompositorCreateRegion(WlCompositorCreateRegionRequest),
    /// Request [WlCompositorReleaseRequest].
    WlCompositorRelease(WlCompositorReleaseRequest),
    /// Request [WlShmPoolCreateBufferRequest].
    WlShmPoolCreateBuffer(WlShmPoolCreateBufferRequest),
    /// Request [WlShmPoolDestroyRequest].
    WlShmPoolDestroy(WlShmPoolDestroyRequest),
    /// Request [WlShmPoolResizeRequest].
    WlShmPoolResize(WlShmPoolResizeRequest),
    /// Request [WlShmCreatePoolRequest].
    WlShmCreatePool(WlShmCreatePoolRequest),
    /// Request [WlShmReleaseRequest].
    WlShmRelease(WlShmReleaseRequest),
    /// Request [WlBufferDestroyRequest].
    WlBufferDestroy(WlBufferDestroyRequest),
    /// Request [WlDataOfferAcceptRequest].
    WlDataOfferAccept(WlDataOfferAcceptRequest),
    /// Request [WlDataOfferReceiveRequest].
    WlDataOfferReceive(WlDataOfferReceiveRequest),
    /// Request [WlDataOfferDestroyRequest].
    WlDataOfferDestroy(WlDataOfferDestroyRequest),
    /// Request [WlDataOfferFinishRequest].
    WlDataOfferFinish(WlDataOfferFinishRequest),
    /// Request [WlDataOfferSetActionsRequest].
    WlDataOfferSetActions(WlDataOfferSetActionsRequest),
    /// Request [WlDataSourceOfferRequest].
    WlDataSourceOffer(WlDataSourceOfferRequest),
    /// Request [WlDataSourceDestroyRequest].
    WlDataSourceDestroy(WlDataSourceDestroyRequest),
    /// Request [WlDataSourceSetActionsRequest].
    WlDataSourceSetActions(WlDataSourceSetActionsRequest),
    /// Request [WlDataDeviceStartDragRequest].
    WlDataDeviceStartDrag(WlDataDeviceStartDragRequest),
    /// Request [WlDataDeviceSetSelectionRequest].
    WlDataDeviceSetSelection(WlDataDeviceSetSelectionRequest),
    /// Request [WlDataDeviceReleaseRequest].
    WlDataDeviceRelease(WlDataDeviceReleaseRequest),
    /// Request [WlDataDeviceManagerCreateDataSourceRequest].
    WlDataDeviceManagerCreateDataSource(WlDataDeviceManagerCreateDataSourceRequest),
    /// Request [WlDataDeviceManagerGetDataDeviceRequest].
    WlDataDeviceManagerGetDataDevice(WlDataDeviceManagerGetDataDeviceRequest),
    /// Request [WlDataDeviceManagerReleaseRequest].
    WlDataDeviceManagerRelease(WlDataDeviceManagerReleaseRequest),
    /// Request [WlShellGetShellSurfaceRequest].
    WlShellGetShellSurface(WlShellGetShellSurfaceRequest),
    /// Request [WlShellSurfacePongRequest].
    WlShellSurfacePong(WlShellSurfacePongRequest),
    /// Request [WlShellSurfaceMoveRequest].
    WlShellSurfaceMove(WlShellSurfaceMoveRequest),
    /// Request [WlShellSurfaceResizeRequest].
    WlShellSurfaceResize(WlShellSurfaceResizeRequest),
    /// Request [WlShellSurfaceSetToplevelRequest].
    WlShellSurfaceSetToplevel(WlShellSurfaceSetToplevelRequest),
    /// Request [WlShellSurfaceSetTransientRequest].
    WlShellSurfaceSetTransient(WlShellSurfaceSetTransientRequest),
    /// Request [WlShellSurfaceSetFullscreenRequest].
    WlShellSurfaceSetFullscreen(WlShellSurfaceSetFullscreenRequest),
    /// Request [WlShellSurfaceSetPopupRequest].
    WlShellSurfaceSetPopup(WlShellSurfaceSetPopupRequest),
    /// Request [WlShellSurfaceSetMaximizedRequest].
    WlShellSurfaceSetMaximized(WlShellSurfaceSetMaximizedRequest),
    /// Request [WlShellSurfaceSetTitleRequest].
    WlShellSurfaceSetTitle(WlShellSurfaceSetTitleRequest),
    /// Request [WlShellSurfaceSetClassRequest].
    WlShellSurfaceSetClass(WlShellSurfaceSetClassRequest),
    /// Request [WlSurfaceDestroyRequest].
    WlSurfaceDestroy(WlSurfaceDestroyRequest),
    /// Request [WlSurfaceAttachRequest].
    WlSurfaceAttach(WlSurfaceAttachRequest),
    /// Request [WlSurfaceDamageRequest].
    WlSurfaceDamage(WlSurfaceDamageRequest),
    /// Request [WlSurfaceFrameRequest].
    WlSurfaceFrame(WlSurfaceFrameRequest),
    /// Request [WlSurfaceSetOpaqueRegionRequest].
    WlSurfaceSetOpaqueRegion(WlSurfaceSetOpaqueRegionRequest),
    /// Request [WlSurfaceSetInputRegionRequest].
    WlSurfaceSetInputRegion(WlSurfaceSetInputRegionRequest),
    /// Request [WlSurfaceCommitRequest].
    WlSurfaceCommit(WlSurfaceCommitRequest),
    /// Request [WlSurfaceSetBufferTransformRequest].
    WlSurfaceSetBufferTransform(WlSurfaceSetBufferTransformRequest),
    /// Request [WlSurfaceSetBufferScaleRequest].
    WlSurfaceSetBufferScale(WlSurfaceSetBufferScaleRequest),
    /// Request [WlSurfaceDamageBufferRequest].
    WlSurfaceDamageBuffer(WlSurfaceDamageBufferRequest),
    /// Request [WlSurfaceOffsetRequest].
    WlSurfaceOffset(WlSurfaceOffsetRequest),
    /// Request [WlSurfaceGetReleaseRequest].
    WlSurfaceGetRelease(WlSurfaceGetReleaseRequest),
    /// Request [WlSeatGetPointerRequest].
    WlSeatGetPointer(WlSeatGetPointerRequest),
    /// Request [WlSeatGetKeyboardRequest].
    WlSeatGetKeyboard(WlSeatGetKeyboardRequest),
    /// Request [WlSeatGetTouchRequest].
    WlSeatGetTouch(WlSeatGetTouchRequest),
    /// Request [WlSeatReleaseRequest].
    WlSeatRelease(WlSeatReleaseRequest),
    /// Request [WlPointerSetCursorRequest].
    WlPointerSetCursor(WlPointerSetCursorRequest),
    /// Request [WlPointerReleaseRequest].
    WlPointerRelease(WlPointerReleaseRequest),
    /// Request [WlKeyboardReleaseRequest].
    WlKeyboardRelease(WlKeyboardReleaseRequest),
    /// Request [WlTouchReleaseRequest].
    WlTouchRelease(WlTouchReleaseRequest),
    /// Request [WlOutputReleaseRequest].
    WlOutputRelease(WlOutputReleaseRequest),
    /// Request [WlRegionDestroyRequest].
    WlRegionDestroy(WlRegionDestroyRequest),
    /// Request [WlRegionAddRequest].
    WlRegionAdd(WlRegionAddRequest),
    /// Request [WlRegionSubtractRequest].
    WlRegionSubtract(WlRegionSubtractRequest),
    /// Request [WlSubcompositorDestroyRequest].
    WlSubcompositorDestroy(WlSubcompositorDestroyRequest),
    /// Request [WlSubcompositorGetSubsurfaceRequest].
    WlSubcompositorGetSubsurface(WlSubcompositorGetSubsurfaceRequest),
    /// Request [WlSubsurfaceDestroyRequest].
    WlSubsurfaceDestroy(WlSubsurfaceDestroyRequest),
    /// Request [WlSubsurfaceSetPositionRequest].
    WlSubsurfaceSetPosition(WlSubsurfaceSetPositionRequest),
    /// Request [WlSubsurfacePlaceAboveRequest].
    WlSubsurfacePlaceAbove(WlSubsurfacePlaceAboveRequest),
    /// Request [WlSubsurfacePlaceBelowRequest].
    WlSubsurfacePlaceBelow(WlSubsurfacePlaceBelowRequest),
    /// Request [WlSubsurfaceSetSyncRequest].
    WlSubsurfaceSetSync(WlSubsurfaceSetSyncRequest),
    /// Request [WlSubsurfaceSetDesyncRequest].
    WlSubsurfaceSetDesync(WlSubsurfaceSetDesyncRequest),
    /// Request [WlFixesDestroyRequest].
    WlFixesDestroy(WlFixesDestroyRequest),
    /// Request [WlFixesDestroyRegistryRequest].
    WlFixesDestroyRegistry(WlFixesDestroyRegistryRequest),
    /// Request [WlFixesAckGlobalRemoveRequest].
    WlFixesAckGlobalRemove(WlFixesAckGlobalRemoveRequest),
}
impl CommandReader<Self> for CommandRequest {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        let interface = Self::reader_state(state)?
            .object_interface(object_id)
            .ok_or(FdReaderError::InvalidProtocol)?;
        match interface.name() {
            "wl_display" => match opcode {
                WlDisplaySyncRequest::OPCODE => Ok(Self::WlDisplaySync(
                    WlDisplaySyncRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlDisplayGetRegistryRequest::OPCODE => Ok(Self::WlDisplayGetRegistry(
                    WlDisplayGetRegistryRequest::read(object_id, opcode, reader, state).await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            "wl_registry" => match opcode {
                WlRegistryBindRequest::OPCODE => Ok(Self::WlRegistryBind(
                    WlRegistryBindRequest::read(object_id, opcode, reader, state).await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            "wl_compositor" => match opcode {
                WlCompositorCreateSurfaceRequest::OPCODE => Ok(Self::WlCompositorCreateSurface(
                    WlCompositorCreateSurfaceRequest::read(object_id, opcode, reader, state)
                        .await?,
                )),
                WlCompositorCreateRegionRequest::OPCODE => Ok(Self::WlCompositorCreateRegion(
                    WlCompositorCreateRegionRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlCompositorReleaseRequest::OPCODE => Ok(Self::WlCompositorRelease(
                    WlCompositorReleaseRequest::read(object_id, opcode, reader, state).await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            "wl_shm_pool" => match opcode {
                WlShmPoolCreateBufferRequest::OPCODE => Ok(Self::WlShmPoolCreateBuffer(
                    WlShmPoolCreateBufferRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlShmPoolDestroyRequest::OPCODE => Ok(Self::WlShmPoolDestroy(
                    WlShmPoolDestroyRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlShmPoolResizeRequest::OPCODE => Ok(Self::WlShmPoolResize(
                    WlShmPoolResizeRequest::read(object_id, opcode, reader, state).await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            "wl_shm" => match opcode {
                WlShmCreatePoolRequest::OPCODE => Ok(Self::WlShmCreatePool(
                    WlShmCreatePoolRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlShmReleaseRequest::OPCODE => Ok(Self::WlShmRelease(
                    WlShmReleaseRequest::read(object_id, opcode, reader, state).await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            "wl_buffer" => match opcode {
                WlBufferDestroyRequest::OPCODE => Ok(Self::WlBufferDestroy(
                    WlBufferDestroyRequest::read(object_id, opcode, reader, state).await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            "wl_data_offer" => match opcode {
                WlDataOfferAcceptRequest::OPCODE => Ok(Self::WlDataOfferAccept(
                    WlDataOfferAcceptRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlDataOfferReceiveRequest::OPCODE => Ok(Self::WlDataOfferReceive(
                    WlDataOfferReceiveRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlDataOfferDestroyRequest::OPCODE => Ok(Self::WlDataOfferDestroy(
                    WlDataOfferDestroyRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlDataOfferFinishRequest::OPCODE => Ok(Self::WlDataOfferFinish(
                    WlDataOfferFinishRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlDataOfferSetActionsRequest::OPCODE => Ok(Self::WlDataOfferSetActions(
                    WlDataOfferSetActionsRequest::read(object_id, opcode, reader, state).await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            "wl_data_source" => match opcode {
                WlDataSourceOfferRequest::OPCODE => Ok(Self::WlDataSourceOffer(
                    WlDataSourceOfferRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlDataSourceDestroyRequest::OPCODE => Ok(Self::WlDataSourceDestroy(
                    WlDataSourceDestroyRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlDataSourceSetActionsRequest::OPCODE => Ok(Self::WlDataSourceSetActions(
                    WlDataSourceSetActionsRequest::read(object_id, opcode, reader, state).await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            "wl_data_device" => match opcode {
                WlDataDeviceStartDragRequest::OPCODE => Ok(Self::WlDataDeviceStartDrag(
                    WlDataDeviceStartDragRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlDataDeviceSetSelectionRequest::OPCODE => Ok(Self::WlDataDeviceSetSelection(
                    WlDataDeviceSetSelectionRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlDataDeviceReleaseRequest::OPCODE => Ok(Self::WlDataDeviceRelease(
                    WlDataDeviceReleaseRequest::read(object_id, opcode, reader, state).await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            "wl_data_device_manager" => match opcode {
                WlDataDeviceManagerCreateDataSourceRequest::OPCODE => {
                    Ok(Self::WlDataDeviceManagerCreateDataSource(
                        WlDataDeviceManagerCreateDataSourceRequest::read(
                            object_id, opcode, reader, state,
                        )
                        .await?,
                    ))
                }
                WlDataDeviceManagerGetDataDeviceRequest::OPCODE => {
                    Ok(Self::WlDataDeviceManagerGetDataDevice(
                        WlDataDeviceManagerGetDataDeviceRequest::read(
                            object_id, opcode, reader, state,
                        )
                        .await?,
                    ))
                }
                WlDataDeviceManagerReleaseRequest::OPCODE => Ok(Self::WlDataDeviceManagerRelease(
                    WlDataDeviceManagerReleaseRequest::read(object_id, opcode, reader, state)
                        .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            "wl_shell" => match opcode {
                WlShellGetShellSurfaceRequest::OPCODE => Ok(Self::WlShellGetShellSurface(
                    WlShellGetShellSurfaceRequest::read(object_id, opcode, reader, state).await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            "wl_shell_surface" => match opcode {
                WlShellSurfacePongRequest::OPCODE => Ok(Self::WlShellSurfacePong(
                    WlShellSurfacePongRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlShellSurfaceMoveRequest::OPCODE => Ok(Self::WlShellSurfaceMove(
                    WlShellSurfaceMoveRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlShellSurfaceResizeRequest::OPCODE => Ok(Self::WlShellSurfaceResize(
                    WlShellSurfaceResizeRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlShellSurfaceSetToplevelRequest::OPCODE => Ok(Self::WlShellSurfaceSetToplevel(
                    WlShellSurfaceSetToplevelRequest::read(object_id, opcode, reader, state)
                        .await?,
                )),
                WlShellSurfaceSetTransientRequest::OPCODE => Ok(Self::WlShellSurfaceSetTransient(
                    WlShellSurfaceSetTransientRequest::read(object_id, opcode, reader, state)
                        .await?,
                )),
                WlShellSurfaceSetFullscreenRequest::OPCODE => {
                    Ok(Self::WlShellSurfaceSetFullscreen(
                        WlShellSurfaceSetFullscreenRequest::read(object_id, opcode, reader, state)
                            .await?,
                    ))
                }
                WlShellSurfaceSetPopupRequest::OPCODE => Ok(Self::WlShellSurfaceSetPopup(
                    WlShellSurfaceSetPopupRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlShellSurfaceSetMaximizedRequest::OPCODE => Ok(Self::WlShellSurfaceSetMaximized(
                    WlShellSurfaceSetMaximizedRequest::read(object_id, opcode, reader, state)
                        .await?,
                )),
                WlShellSurfaceSetTitleRequest::OPCODE => Ok(Self::WlShellSurfaceSetTitle(
                    WlShellSurfaceSetTitleRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlShellSurfaceSetClassRequest::OPCODE => Ok(Self::WlShellSurfaceSetClass(
                    WlShellSurfaceSetClassRequest::read(object_id, opcode, reader, state).await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            "wl_surface" => match opcode {
                WlSurfaceDestroyRequest::OPCODE => Ok(Self::WlSurfaceDestroy(
                    WlSurfaceDestroyRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlSurfaceAttachRequest::OPCODE => Ok(Self::WlSurfaceAttach(
                    WlSurfaceAttachRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlSurfaceDamageRequest::OPCODE => Ok(Self::WlSurfaceDamage(
                    WlSurfaceDamageRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlSurfaceFrameRequest::OPCODE => Ok(Self::WlSurfaceFrame(
                    WlSurfaceFrameRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlSurfaceSetOpaqueRegionRequest::OPCODE => Ok(Self::WlSurfaceSetOpaqueRegion(
                    WlSurfaceSetOpaqueRegionRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlSurfaceSetInputRegionRequest::OPCODE => Ok(Self::WlSurfaceSetInputRegion(
                    WlSurfaceSetInputRegionRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlSurfaceCommitRequest::OPCODE => Ok(Self::WlSurfaceCommit(
                    WlSurfaceCommitRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlSurfaceSetBufferTransformRequest::OPCODE => {
                    Ok(Self::WlSurfaceSetBufferTransform(
                        WlSurfaceSetBufferTransformRequest::read(object_id, opcode, reader, state)
                            .await?,
                    ))
                }
                WlSurfaceSetBufferScaleRequest::OPCODE => Ok(Self::WlSurfaceSetBufferScale(
                    WlSurfaceSetBufferScaleRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlSurfaceDamageBufferRequest::OPCODE => Ok(Self::WlSurfaceDamageBuffer(
                    WlSurfaceDamageBufferRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlSurfaceOffsetRequest::OPCODE => Ok(Self::WlSurfaceOffset(
                    WlSurfaceOffsetRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlSurfaceGetReleaseRequest::OPCODE => Ok(Self::WlSurfaceGetRelease(
                    WlSurfaceGetReleaseRequest::read(object_id, opcode, reader, state).await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            "wl_seat" => match opcode {
                WlSeatGetPointerRequest::OPCODE => Ok(Self::WlSeatGetPointer(
                    WlSeatGetPointerRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlSeatGetKeyboardRequest::OPCODE => Ok(Self::WlSeatGetKeyboard(
                    WlSeatGetKeyboardRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlSeatGetTouchRequest::OPCODE => Ok(Self::WlSeatGetTouch(
                    WlSeatGetTouchRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlSeatReleaseRequest::OPCODE => Ok(Self::WlSeatRelease(
                    WlSeatReleaseRequest::read(object_id, opcode, reader, state).await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            "wl_pointer" => match opcode {
                WlPointerSetCursorRequest::OPCODE => Ok(Self::WlPointerSetCursor(
                    WlPointerSetCursorRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlPointerReleaseRequest::OPCODE => Ok(Self::WlPointerRelease(
                    WlPointerReleaseRequest::read(object_id, opcode, reader, state).await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            "wl_keyboard" => match opcode {
                WlKeyboardReleaseRequest::OPCODE => Ok(Self::WlKeyboardRelease(
                    WlKeyboardReleaseRequest::read(object_id, opcode, reader, state).await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            "wl_touch" => match opcode {
                WlTouchReleaseRequest::OPCODE => Ok(Self::WlTouchRelease(
                    WlTouchReleaseRequest::read(object_id, opcode, reader, state).await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            "wl_output" => match opcode {
                WlOutputReleaseRequest::OPCODE => Ok(Self::WlOutputRelease(
                    WlOutputReleaseRequest::read(object_id, opcode, reader, state).await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            "wl_region" => match opcode {
                WlRegionDestroyRequest::OPCODE => Ok(Self::WlRegionDestroy(
                    WlRegionDestroyRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlRegionAddRequest::OPCODE => Ok(Self::WlRegionAdd(
                    WlRegionAddRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlRegionSubtractRequest::OPCODE => Ok(Self::WlRegionSubtract(
                    WlRegionSubtractRequest::read(object_id, opcode, reader, state).await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            "wl_subcompositor" => match opcode {
                WlSubcompositorDestroyRequest::OPCODE => Ok(Self::WlSubcompositorDestroy(
                    WlSubcompositorDestroyRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlSubcompositorGetSubsurfaceRequest::OPCODE => {
                    Ok(Self::WlSubcompositorGetSubsurface(
                        WlSubcompositorGetSubsurfaceRequest::read(object_id, opcode, reader, state)
                            .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            "wl_subsurface" => match opcode {
                WlSubsurfaceDestroyRequest::OPCODE => Ok(Self::WlSubsurfaceDestroy(
                    WlSubsurfaceDestroyRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlSubsurfaceSetPositionRequest::OPCODE => Ok(Self::WlSubsurfaceSetPosition(
                    WlSubsurfaceSetPositionRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlSubsurfacePlaceAboveRequest::OPCODE => Ok(Self::WlSubsurfacePlaceAbove(
                    WlSubsurfacePlaceAboveRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlSubsurfacePlaceBelowRequest::OPCODE => Ok(Self::WlSubsurfacePlaceBelow(
                    WlSubsurfacePlaceBelowRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlSubsurfaceSetSyncRequest::OPCODE => Ok(Self::WlSubsurfaceSetSync(
                    WlSubsurfaceSetSyncRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlSubsurfaceSetDesyncRequest::OPCODE => Ok(Self::WlSubsurfaceSetDesync(
                    WlSubsurfaceSetDesyncRequest::read(object_id, opcode, reader, state).await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            "wl_fixes" => match opcode {
                WlFixesDestroyRequest::OPCODE => Ok(Self::WlFixesDestroy(
                    WlFixesDestroyRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlFixesDestroyRegistryRequest::OPCODE => Ok(Self::WlFixesDestroyRegistry(
                    WlFixesDestroyRegistryRequest::read(object_id, opcode, reader, state).await?,
                )),
                WlFixesAckGlobalRemoveRequest::OPCODE => Ok(Self::WlFixesAckGlobalRemove(
                    WlFixesAckGlobalRemoveRequest::read(object_id, opcode, reader, state).await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            _ => Err(FdReaderError::InvalidProtocol),
        }
    }
}

impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::WlDisplaySync(request) => request.object_id(),
            Self::WlDisplayGetRegistry(request) => request.object_id(),
            Self::WlRegistryBind(request) => request.object_id(),
            Self::WlCompositorCreateSurface(request) => request.object_id(),
            Self::WlCompositorCreateRegion(request) => request.object_id(),
            Self::WlCompositorRelease(request) => request.object_id(),
            Self::WlShmPoolCreateBuffer(request) => request.object_id(),
            Self::WlShmPoolDestroy(request) => request.object_id(),
            Self::WlShmPoolResize(request) => request.object_id(),
            Self::WlShmCreatePool(request) => request.object_id(),
            Self::WlShmRelease(request) => request.object_id(),
            Self::WlBufferDestroy(request) => request.object_id(),
            Self::WlDataOfferAccept(request) => request.object_id(),
            Self::WlDataOfferReceive(request) => request.object_id(),
            Self::WlDataOfferDestroy(request) => request.object_id(),
            Self::WlDataOfferFinish(request) => request.object_id(),
            Self::WlDataOfferSetActions(request) => request.object_id(),
            Self::WlDataSourceOffer(request) => request.object_id(),
            Self::WlDataSourceDestroy(request) => request.object_id(),
            Self::WlDataSourceSetActions(request) => request.object_id(),
            Self::WlDataDeviceStartDrag(request) => request.object_id(),
            Self::WlDataDeviceSetSelection(request) => request.object_id(),
            Self::WlDataDeviceRelease(request) => request.object_id(),
            Self::WlDataDeviceManagerCreateDataSource(request) => request.object_id(),
            Self::WlDataDeviceManagerGetDataDevice(request) => request.object_id(),
            Self::WlDataDeviceManagerRelease(request) => request.object_id(),
            Self::WlShellGetShellSurface(request) => request.object_id(),
            Self::WlShellSurfacePong(request) => request.object_id(),
            Self::WlShellSurfaceMove(request) => request.object_id(),
            Self::WlShellSurfaceResize(request) => request.object_id(),
            Self::WlShellSurfaceSetToplevel(request) => request.object_id(),
            Self::WlShellSurfaceSetTransient(request) => request.object_id(),
            Self::WlShellSurfaceSetFullscreen(request) => request.object_id(),
            Self::WlShellSurfaceSetPopup(request) => request.object_id(),
            Self::WlShellSurfaceSetMaximized(request) => request.object_id(),
            Self::WlShellSurfaceSetTitle(request) => request.object_id(),
            Self::WlShellSurfaceSetClass(request) => request.object_id(),
            Self::WlSurfaceDestroy(request) => request.object_id(),
            Self::WlSurfaceAttach(request) => request.object_id(),
            Self::WlSurfaceDamage(request) => request.object_id(),
            Self::WlSurfaceFrame(request) => request.object_id(),
            Self::WlSurfaceSetOpaqueRegion(request) => request.object_id(),
            Self::WlSurfaceSetInputRegion(request) => request.object_id(),
            Self::WlSurfaceCommit(request) => request.object_id(),
            Self::WlSurfaceSetBufferTransform(request) => request.object_id(),
            Self::WlSurfaceSetBufferScale(request) => request.object_id(),
            Self::WlSurfaceDamageBuffer(request) => request.object_id(),
            Self::WlSurfaceOffset(request) => request.object_id(),
            Self::WlSurfaceGetRelease(request) => request.object_id(),
            Self::WlSeatGetPointer(request) => request.object_id(),
            Self::WlSeatGetKeyboard(request) => request.object_id(),
            Self::WlSeatGetTouch(request) => request.object_id(),
            Self::WlSeatRelease(request) => request.object_id(),
            Self::WlPointerSetCursor(request) => request.object_id(),
            Self::WlPointerRelease(request) => request.object_id(),
            Self::WlKeyboardRelease(request) => request.object_id(),
            Self::WlTouchRelease(request) => request.object_id(),
            Self::WlOutputRelease(request) => request.object_id(),
            Self::WlRegionDestroy(request) => request.object_id(),
            Self::WlRegionAdd(request) => request.object_id(),
            Self::WlRegionSubtract(request) => request.object_id(),
            Self::WlSubcompositorDestroy(request) => request.object_id(),
            Self::WlSubcompositorGetSubsurface(request) => request.object_id(),
            Self::WlSubsurfaceDestroy(request) => request.object_id(),
            Self::WlSubsurfaceSetPosition(request) => request.object_id(),
            Self::WlSubsurfacePlaceAbove(request) => request.object_id(),
            Self::WlSubsurfacePlaceBelow(request) => request.object_id(),
            Self::WlSubsurfaceSetSync(request) => request.object_id(),
            Self::WlSubsurfaceSetDesync(request) => request.object_id(),
            Self::WlFixesDestroy(request) => request.object_id(),
            Self::WlFixesDestroyRegistry(request) => request.object_id(),
            Self::WlFixesAckGlobalRemove(request) => request.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WlDisplaySync(request) => request.opcode(),
            Self::WlDisplayGetRegistry(request) => request.opcode(),
            Self::WlRegistryBind(request) => request.opcode(),
            Self::WlCompositorCreateSurface(request) => request.opcode(),
            Self::WlCompositorCreateRegion(request) => request.opcode(),
            Self::WlCompositorRelease(request) => request.opcode(),
            Self::WlShmPoolCreateBuffer(request) => request.opcode(),
            Self::WlShmPoolDestroy(request) => request.opcode(),
            Self::WlShmPoolResize(request) => request.opcode(),
            Self::WlShmCreatePool(request) => request.opcode(),
            Self::WlShmRelease(request) => request.opcode(),
            Self::WlBufferDestroy(request) => request.opcode(),
            Self::WlDataOfferAccept(request) => request.opcode(),
            Self::WlDataOfferReceive(request) => request.opcode(),
            Self::WlDataOfferDestroy(request) => request.opcode(),
            Self::WlDataOfferFinish(request) => request.opcode(),
            Self::WlDataOfferSetActions(request) => request.opcode(),
            Self::WlDataSourceOffer(request) => request.opcode(),
            Self::WlDataSourceDestroy(request) => request.opcode(),
            Self::WlDataSourceSetActions(request) => request.opcode(),
            Self::WlDataDeviceStartDrag(request) => request.opcode(),
            Self::WlDataDeviceSetSelection(request) => request.opcode(),
            Self::WlDataDeviceRelease(request) => request.opcode(),
            Self::WlDataDeviceManagerCreateDataSource(request) => request.opcode(),
            Self::WlDataDeviceManagerGetDataDevice(request) => request.opcode(),
            Self::WlDataDeviceManagerRelease(request) => request.opcode(),
            Self::WlShellGetShellSurface(request) => request.opcode(),
            Self::WlShellSurfacePong(request) => request.opcode(),
            Self::WlShellSurfaceMove(request) => request.opcode(),
            Self::WlShellSurfaceResize(request) => request.opcode(),
            Self::WlShellSurfaceSetToplevel(request) => request.opcode(),
            Self::WlShellSurfaceSetTransient(request) => request.opcode(),
            Self::WlShellSurfaceSetFullscreen(request) => request.opcode(),
            Self::WlShellSurfaceSetPopup(request) => request.opcode(),
            Self::WlShellSurfaceSetMaximized(request) => request.opcode(),
            Self::WlShellSurfaceSetTitle(request) => request.opcode(),
            Self::WlShellSurfaceSetClass(request) => request.opcode(),
            Self::WlSurfaceDestroy(request) => request.opcode(),
            Self::WlSurfaceAttach(request) => request.opcode(),
            Self::WlSurfaceDamage(request) => request.opcode(),
            Self::WlSurfaceFrame(request) => request.opcode(),
            Self::WlSurfaceSetOpaqueRegion(request) => request.opcode(),
            Self::WlSurfaceSetInputRegion(request) => request.opcode(),
            Self::WlSurfaceCommit(request) => request.opcode(),
            Self::WlSurfaceSetBufferTransform(request) => request.opcode(),
            Self::WlSurfaceSetBufferScale(request) => request.opcode(),
            Self::WlSurfaceDamageBuffer(request) => request.opcode(),
            Self::WlSurfaceOffset(request) => request.opcode(),
            Self::WlSurfaceGetRelease(request) => request.opcode(),
            Self::WlSeatGetPointer(request) => request.opcode(),
            Self::WlSeatGetKeyboard(request) => request.opcode(),
            Self::WlSeatGetTouch(request) => request.opcode(),
            Self::WlSeatRelease(request) => request.opcode(),
            Self::WlPointerSetCursor(request) => request.opcode(),
            Self::WlPointerRelease(request) => request.opcode(),
            Self::WlKeyboardRelease(request) => request.opcode(),
            Self::WlTouchRelease(request) => request.opcode(),
            Self::WlOutputRelease(request) => request.opcode(),
            Self::WlRegionDestroy(request) => request.opcode(),
            Self::WlRegionAdd(request) => request.opcode(),
            Self::WlRegionSubtract(request) => request.opcode(),
            Self::WlSubcompositorDestroy(request) => request.opcode(),
            Self::WlSubcompositorGetSubsurface(request) => request.opcode(),
            Self::WlSubsurfaceDestroy(request) => request.opcode(),
            Self::WlSubsurfaceSetPosition(request) => request.opcode(),
            Self::WlSubsurfacePlaceAbove(request) => request.opcode(),
            Self::WlSubsurfacePlaceBelow(request) => request.opcode(),
            Self::WlSubsurfaceSetSync(request) => request.opcode(),
            Self::WlSubsurfaceSetDesync(request) => request.opcode(),
            Self::WlFixesDestroy(request) => request.opcode(),
            Self::WlFixesDestroyRegistry(request) => request.opcode(),
            Self::WlFixesAckGlobalRemove(request) => request.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WlDisplaySync(request) => request.payload_size(),
            Self::WlDisplayGetRegistry(request) => request.payload_size(),
            Self::WlRegistryBind(request) => request.payload_size(),
            Self::WlCompositorCreateSurface(request) => request.payload_size(),
            Self::WlCompositorCreateRegion(request) => request.payload_size(),
            Self::WlCompositorRelease(request) => request.payload_size(),
            Self::WlShmPoolCreateBuffer(request) => request.payload_size(),
            Self::WlShmPoolDestroy(request) => request.payload_size(),
            Self::WlShmPoolResize(request) => request.payload_size(),
            Self::WlShmCreatePool(request) => request.payload_size(),
            Self::WlShmRelease(request) => request.payload_size(),
            Self::WlBufferDestroy(request) => request.payload_size(),
            Self::WlDataOfferAccept(request) => request.payload_size(),
            Self::WlDataOfferReceive(request) => request.payload_size(),
            Self::WlDataOfferDestroy(request) => request.payload_size(),
            Self::WlDataOfferFinish(request) => request.payload_size(),
            Self::WlDataOfferSetActions(request) => request.payload_size(),
            Self::WlDataSourceOffer(request) => request.payload_size(),
            Self::WlDataSourceDestroy(request) => request.payload_size(),
            Self::WlDataSourceSetActions(request) => request.payload_size(),
            Self::WlDataDeviceStartDrag(request) => request.payload_size(),
            Self::WlDataDeviceSetSelection(request) => request.payload_size(),
            Self::WlDataDeviceRelease(request) => request.payload_size(),
            Self::WlDataDeviceManagerCreateDataSource(request) => request.payload_size(),
            Self::WlDataDeviceManagerGetDataDevice(request) => request.payload_size(),
            Self::WlDataDeviceManagerRelease(request) => request.payload_size(),
            Self::WlShellGetShellSurface(request) => request.payload_size(),
            Self::WlShellSurfacePong(request) => request.payload_size(),
            Self::WlShellSurfaceMove(request) => request.payload_size(),
            Self::WlShellSurfaceResize(request) => request.payload_size(),
            Self::WlShellSurfaceSetToplevel(request) => request.payload_size(),
            Self::WlShellSurfaceSetTransient(request) => request.payload_size(),
            Self::WlShellSurfaceSetFullscreen(request) => request.payload_size(),
            Self::WlShellSurfaceSetPopup(request) => request.payload_size(),
            Self::WlShellSurfaceSetMaximized(request) => request.payload_size(),
            Self::WlShellSurfaceSetTitle(request) => request.payload_size(),
            Self::WlShellSurfaceSetClass(request) => request.payload_size(),
            Self::WlSurfaceDestroy(request) => request.payload_size(),
            Self::WlSurfaceAttach(request) => request.payload_size(),
            Self::WlSurfaceDamage(request) => request.payload_size(),
            Self::WlSurfaceFrame(request) => request.payload_size(),
            Self::WlSurfaceSetOpaqueRegion(request) => request.payload_size(),
            Self::WlSurfaceSetInputRegion(request) => request.payload_size(),
            Self::WlSurfaceCommit(request) => request.payload_size(),
            Self::WlSurfaceSetBufferTransform(request) => request.payload_size(),
            Self::WlSurfaceSetBufferScale(request) => request.payload_size(),
            Self::WlSurfaceDamageBuffer(request) => request.payload_size(),
            Self::WlSurfaceOffset(request) => request.payload_size(),
            Self::WlSurfaceGetRelease(request) => request.payload_size(),
            Self::WlSeatGetPointer(request) => request.payload_size(),
            Self::WlSeatGetKeyboard(request) => request.payload_size(),
            Self::WlSeatGetTouch(request) => request.payload_size(),
            Self::WlSeatRelease(request) => request.payload_size(),
            Self::WlPointerSetCursor(request) => request.payload_size(),
            Self::WlPointerRelease(request) => request.payload_size(),
            Self::WlKeyboardRelease(request) => request.payload_size(),
            Self::WlTouchRelease(request) => request.payload_size(),
            Self::WlOutputRelease(request) => request.payload_size(),
            Self::WlRegionDestroy(request) => request.payload_size(),
            Self::WlRegionAdd(request) => request.payload_size(),
            Self::WlRegionSubtract(request) => request.payload_size(),
            Self::WlSubcompositorDestroy(request) => request.payload_size(),
            Self::WlSubcompositorGetSubsurface(request) => request.payload_size(),
            Self::WlSubsurfaceDestroy(request) => request.payload_size(),
            Self::WlSubsurfaceSetPosition(request) => request.payload_size(),
            Self::WlSubsurfacePlaceAbove(request) => request.payload_size(),
            Self::WlSubsurfacePlaceBelow(request) => request.payload_size(),
            Self::WlSubsurfaceSetSync(request) => request.payload_size(),
            Self::WlSubsurfaceSetDesync(request) => request.payload_size(),
            Self::WlFixesDestroy(request) => request.payload_size(),
            Self::WlFixesDestroyRegistry(request) => request.payload_size(),
            Self::WlFixesAckGlobalRemove(request) => request.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::WlDisplaySync(request) => request.write(writer, state).await,
            Self::WlDisplayGetRegistry(request) => request.write(writer, state).await,
            Self::WlRegistryBind(request) => request.write(writer, state).await,
            Self::WlCompositorCreateSurface(request) => request.write(writer, state).await,
            Self::WlCompositorCreateRegion(request) => request.write(writer, state).await,
            Self::WlCompositorRelease(request) => request.write(writer, state).await,
            Self::WlShmPoolCreateBuffer(request) => request.write(writer, state).await,
            Self::WlShmPoolDestroy(request) => request.write(writer, state).await,
            Self::WlShmPoolResize(request) => request.write(writer, state).await,
            Self::WlShmCreatePool(request) => request.write(writer, state).await,
            Self::WlShmRelease(request) => request.write(writer, state).await,
            Self::WlBufferDestroy(request) => request.write(writer, state).await,
            Self::WlDataOfferAccept(request) => request.write(writer, state).await,
            Self::WlDataOfferReceive(request) => request.write(writer, state).await,
            Self::WlDataOfferDestroy(request) => request.write(writer, state).await,
            Self::WlDataOfferFinish(request) => request.write(writer, state).await,
            Self::WlDataOfferSetActions(request) => request.write(writer, state).await,
            Self::WlDataSourceOffer(request) => request.write(writer, state).await,
            Self::WlDataSourceDestroy(request) => request.write(writer, state).await,
            Self::WlDataSourceSetActions(request) => request.write(writer, state).await,
            Self::WlDataDeviceStartDrag(request) => request.write(writer, state).await,
            Self::WlDataDeviceSetSelection(request) => request.write(writer, state).await,
            Self::WlDataDeviceRelease(request) => request.write(writer, state).await,
            Self::WlDataDeviceManagerCreateDataSource(request) => {
                request.write(writer, state).await
            }
            Self::WlDataDeviceManagerGetDataDevice(request) => request.write(writer, state).await,
            Self::WlDataDeviceManagerRelease(request) => request.write(writer, state).await,
            Self::WlShellGetShellSurface(request) => request.write(writer, state).await,
            Self::WlShellSurfacePong(request) => request.write(writer, state).await,
            Self::WlShellSurfaceMove(request) => request.write(writer, state).await,
            Self::WlShellSurfaceResize(request) => request.write(writer, state).await,
            Self::WlShellSurfaceSetToplevel(request) => request.write(writer, state).await,
            Self::WlShellSurfaceSetTransient(request) => request.write(writer, state).await,
            Self::WlShellSurfaceSetFullscreen(request) => request.write(writer, state).await,
            Self::WlShellSurfaceSetPopup(request) => request.write(writer, state).await,
            Self::WlShellSurfaceSetMaximized(request) => request.write(writer, state).await,
            Self::WlShellSurfaceSetTitle(request) => request.write(writer, state).await,
            Self::WlShellSurfaceSetClass(request) => request.write(writer, state).await,
            Self::WlSurfaceDestroy(request) => request.write(writer, state).await,
            Self::WlSurfaceAttach(request) => request.write(writer, state).await,
            Self::WlSurfaceDamage(request) => request.write(writer, state).await,
            Self::WlSurfaceFrame(request) => request.write(writer, state).await,
            Self::WlSurfaceSetOpaqueRegion(request) => request.write(writer, state).await,
            Self::WlSurfaceSetInputRegion(request) => request.write(writer, state).await,
            Self::WlSurfaceCommit(request) => request.write(writer, state).await,
            Self::WlSurfaceSetBufferTransform(request) => request.write(writer, state).await,
            Self::WlSurfaceSetBufferScale(request) => request.write(writer, state).await,
            Self::WlSurfaceDamageBuffer(request) => request.write(writer, state).await,
            Self::WlSurfaceOffset(request) => request.write(writer, state).await,
            Self::WlSurfaceGetRelease(request) => request.write(writer, state).await,
            Self::WlSeatGetPointer(request) => request.write(writer, state).await,
            Self::WlSeatGetKeyboard(request) => request.write(writer, state).await,
            Self::WlSeatGetTouch(request) => request.write(writer, state).await,
            Self::WlSeatRelease(request) => request.write(writer, state).await,
            Self::WlPointerSetCursor(request) => request.write(writer, state).await,
            Self::WlPointerRelease(request) => request.write(writer, state).await,
            Self::WlKeyboardRelease(request) => request.write(writer, state).await,
            Self::WlTouchRelease(request) => request.write(writer, state).await,
            Self::WlOutputRelease(request) => request.write(writer, state).await,
            Self::WlRegionDestroy(request) => request.write(writer, state).await,
            Self::WlRegionAdd(request) => request.write(writer, state).await,
            Self::WlRegionSubtract(request) => request.write(writer, state).await,
            Self::WlSubcompositorDestroy(request) => request.write(writer, state).await,
            Self::WlSubcompositorGetSubsurface(request) => request.write(writer, state).await,
            Self::WlSubsurfaceDestroy(request) => request.write(writer, state).await,
            Self::WlSubsurfaceSetPosition(request) => request.write(writer, state).await,
            Self::WlSubsurfacePlaceAbove(request) => request.write(writer, state).await,
            Self::WlSubsurfacePlaceBelow(request) => request.write(writer, state).await,
            Self::WlSubsurfaceSetSync(request) => request.write(writer, state).await,
            Self::WlSubsurfaceSetDesync(request) => request.write(writer, state).await,
            Self::WlFixesDestroy(request) => request.write(writer, state).await,
            Self::WlFixesDestroyRegistry(request) => request.write(writer, state).await,
            Self::WlFixesAckGlobalRemove(request) => request.write(writer, state).await,
        }
    }
}
