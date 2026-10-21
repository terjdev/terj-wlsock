//! Stores the interface metadata needed to decode one connection's messages.

use crate::protocol::ObjectInteface;
use std::collections::{HashMap, hash_map::Entry};
use thiserror::Error;

/// Wayland reserves IDs from 0xff000000 onward for server-created objects.
const SERVER_ID_START: u32 = 0xff000000;

/// Reports a conflict or unsupported metadata in a wire `new_id` declaration.
#[derive(Error, Debug)]
pub enum RegisterObjectError {
    /// An existing entry must not be replaced while its ID is reserved.
    #[error("Object with id {0} already registered.")]
    AlreadyRegistered(u32),
    /// The identifier range or interface version is unsupported.
    #[error("Invalid object id or interface version for {0}.")]
    InvalidObject(u32),
}

/// Stores object interfaces and versions without tracking compositor resources.
///
/// The receiver ID selects the interface-specific opcode namespace. Version
/// metadata checks message availability and determines typed `new_id` versions.
/// This table does not validate object arguments, globals, roles, or lifetimes.
/// The compositor must reject operations on resources it has already destroyed.
///
/// ID 1 always denotes wl_display at version 1. Client-created IDs remain in
/// the table until delete_id is encoded; server IDs can be removed immediately
/// upon a destructor because they do not receive delete_id acknowledgements.
/// These removals maintain the wire namespace rather than resource lifetimes.
///
/// ```xml
/// <event name="delete_id">
///       <description summary="acknowledge object ID deletion">
///         This event is used internally by the object ID management logic.
///
///         When the server stops using an object created by the client, the server
///         sends this event. In particular, after sending this event, the server
///         will no longer send any events that contain the object as the receiver
///         or as an argument.
///
///         When the client receives this event, it knows that it can reuse the
///         object ID.
///       </description>
///       <arg name="id" type="uint" summary="deleted object ID" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ClientState {
    data: HashMap<u32, ObjectInteface>,
}

impl Default for ClientState {
    fn default() -> Self {
        let mut data = HashMap::with_capacity(1);
        data.insert(1, ObjectInteface::WlDisplay { version: 1 });
        Self { data }
    }
}

impl ClientState {
    /// Returns receiver metadata without judging whether its resource is alive.
    pub(crate) fn object_interface(&self, id: u32) -> Option<ObjectInteface> {
        self.data.get(&id).copied()
    }

    /// Records a new interface without replacing an existing ID mapping.
    ///
    /// `server` selects the wire identifier range of the creating message.
    ///
    /// # Errors
    /// Rejects reserved IDs, an incorrect range, unsupported versions, or an ID
    /// whose previous mapping has not yet been released.
    pub(crate) fn register_object(
        &mut self,
        id: u32,
        ifc: ObjectInteface,
        server: bool,
    ) -> Result<(), RegisterObjectError> {
        if id <= 1
            || (id >= SERVER_ID_START) != server
            || ifc.version() == 0
            || ifc.version() > ifc.max_version()
        {
            return Err(RegisterObjectError::InvalidObject(id));
        }
        match self.data.entry(id) {
            Entry::Occupied(_) => Err(RegisterObjectError::AlreadyRegistered(id)),
            Entry::Vacant(entry) => {
                entry.insert(ifc);
                Ok(())
            }
        }
    }

    /// Removes a client ID after its delete_id acknowledgement is encoded.
    ///
    /// The caller checks the range and existence before encoding the event.
    /// Repeated removal is harmless; no compositor lifetime state is inspected.
    pub(crate) fn delete_id(&mut self, id: u32) {
        if (2..SERVER_ID_START).contains(&id) {
            self.data.remove(&id);
        }
    }

    /// Checks only whether delete_id can release an existing client ID mapping.
    pub(crate) fn can_delete_id(&self, id: u32) -> bool {
        (2..SERVER_ID_START).contains(&id) && self.data.contains_key(&id)
    }

    /// Releases server metadata on a destructor, retaining client IDs until delete_id.
    pub(crate) fn release_server_id(&mut self, id: u32) {
        if id >= SERVER_ID_START {
            self.data.remove(&id);
        }
    }
}
