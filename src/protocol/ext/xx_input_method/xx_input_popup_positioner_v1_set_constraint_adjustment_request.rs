use super::ObjectInteface as XxInputMethodObjectInteface;
use super::XxInputPopupPositionerV1ConstraintAdjustment;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_input_popup_positioner_v1.set_constraint_adjustment`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_constraint_adjustment">
///       <description summary="set the adjustment to be done when constrained">
///         Specify how the popup should be positioned if the originally intended
///         position caused the surface to be constrained, meaning at least
///         partially outside positioning boundaries set by the compositor. The
///         adjustment is set by constructing a bitmask describing the adjustment to
///         be made when the surface is constrained on that axis.
///
///         If no bit for one axis is set, the compositor will assume that the child
///         surface should not change its position on that axis when constrained.
///
///         If more than one bit for one axis is set, the order of how adjustments
///         are applied is specified in the corresponding adjustment descriptions.
///
///         The default adjustment is none.
///       </description>
///       <arg name="constraint_adjustment" type="uint" enum="constraint_adjustment" summary="bit mask of constraint adjustments" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxInputPopupPositionerV1SetConstraintAdjustmentRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// bit mask of constraint adjustments.
    pub constraint_adjustment: XxInputPopupPositionerV1ConstraintAdjustment,
}
impl XxInputPopupPositionerV1SetConstraintAdjustmentRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 4;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxInputPopupPositionerV1SetConstraintAdjustmentRequest {
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
impl InterfaceReader for XxInputPopupPositionerV1SetConstraintAdjustmentRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxInputMethod(
                XxInputMethodObjectInteface::XxInputPopupPositionerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let constraint_adjustment =
            XxInputPopupPositionerV1ConstraintAdjustment(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            constraint_adjustment,
        })
    }
}
impl CommandWriter for XxInputPopupPositionerV1SetConstraintAdjustmentRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "xx_input_popup_positioner_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::XxInputMethod(
                XxInputMethodObjectInteface::XxInputPopupPositionerV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.constraint_adjustment.0).await?;
        Ok(())
    }
}
