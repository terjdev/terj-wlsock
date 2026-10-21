/// Represents the dynamic new_id argument of wl_registry.bind.
///
/// On the wire, an untyped new_id expands into an interface name,
/// version, and new identifier. This is a nested bind argument, not a standalone message.
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
