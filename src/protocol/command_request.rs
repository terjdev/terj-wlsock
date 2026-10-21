use crate::protocol::ClientState;
use crate::protocol::core::*;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Contains every core-protocol request in its dedicated message structure.
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
        let interface = Self::read_state(state)?
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
