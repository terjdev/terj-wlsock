//! Defines core Wayland object interfaces and their negotiated versions.

/// Identifies a client object interface and its negotiated protocol version.
///
/// Variants correspond to the interfaces defined in [Wayland XML](https://gitlab.freedesktop.org/wayland/wayland/-/blob/main/protocol/wayland.xml).
/// Object registration validates the negotiated version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    /// Object of interface wl_display; maximum supported version: 1.
    /// ```xml
    /// <interface name="wl_display" version="1" />
    /// ```
    WlDisplay { version: u32 },
    /// Object of interface wl_registry; maximum supported version: 1.
    /// ```xml
    /// <interface name="wl_registry" version="1" />
    /// ```
    WlRegistry { version: u32 },
    /// Object of interface wl_callback; maximum supported version: 1.
    /// ```xml
    /// <interface name="wl_callback" version="1" frozen="true" />
    /// ```
    WlCallback { version: u32 },
    /// Object of interface wl_compositor; maximum supported version: 7.
    /// ```xml
    /// <interface name="wl_compositor" version="7" />
    /// ```
    WlCompositor { version: u32 },
    /// Object of interface wl_shm_pool; maximum supported version: 3.
    /// ```xml
    /// <interface name="wl_shm_pool" version="3" />
    /// ```
    WlShmPool { version: u32 },
    /// Object of interface wl_shm; maximum supported version: 3.
    /// ```xml
    /// <interface name="wl_shm" version="3" />
    /// ```
    WlShm { version: u32 },
    /// Object of interface wl_buffer; maximum supported version: 1.
    /// ```xml
    /// <interface name="wl_buffer" version="1" frozen="true" />
    /// ```
    WlBuffer { version: u32 },
    /// Object of interface wl_data_offer; maximum supported version: 4.
    /// ```xml
    /// <interface name="wl_data_offer" version="4" />
    /// ```
    WlDataOffer { version: u32 },
    /// Object of interface wl_data_source; maximum supported version: 4.
    /// ```xml
    /// <interface name="wl_data_source" version="4" />
    /// ```
    WlDataSource { version: u32 },
    /// Object of interface wl_data_device; maximum supported version: 4.
    /// ```xml
    /// <interface name="wl_data_device" version="4" />
    /// ```
    WlDataDevice { version: u32 },
    /// Object of interface wl_data_device_manager; maximum supported version: 4.
    /// ```xml
    /// <interface name="wl_data_device_manager" version="4" />
    /// ```
    WlDataDeviceManager { version: u32 },
    /// Object of interface wl_shell; maximum supported version: 1.
    /// ```xml
    /// <interface name="wl_shell" version="1" />
    /// ```
    WlShell { version: u32 },
    /// Object of interface wl_shell_surface; maximum supported version: 1.
    /// ```xml
    /// <interface name="wl_shell_surface" version="1" />
    /// ```
    WlShellSurface { version: u32 },
    /// Object of interface wl_surface; maximum supported version: 7.
    /// ```xml
    /// <interface name="wl_surface" version="7" />
    /// ```
    WlSurface { version: u32 },
    /// Object of interface wl_seat; maximum supported version: 11.
    /// ```xml
    /// <interface name="wl_seat" version="11" />
    /// ```
    WlSeat { version: u32 },
    /// Object of interface wl_pointer; maximum supported version: 11.
    /// ```xml
    /// <interface name="wl_pointer" version="11" />
    /// ```
    WlPointer { version: u32 },
    /// Object of interface wl_keyboard; maximum supported version: 11.
    /// ```xml
    /// <interface name="wl_keyboard" version="11" />
    /// ```
    WlKeyboard { version: u32 },
    /// Object of interface wl_touch; maximum supported version: 11.
    /// ```xml
    /// <interface name="wl_touch" version="11" />
    /// ```
    WlTouch { version: u32 },
    /// Object of interface wl_output; maximum supported version: 4.
    /// ```xml
    /// <interface name="wl_output" version="4" />
    /// ```
    WlOutput { version: u32 },
    /// Object of interface wl_region; maximum supported version: 7.
    /// ```xml
    /// <interface name="wl_region" version="7" />
    /// ```
    WlRegion { version: u32 },
    /// Object of interface wl_subcompositor; maximum supported version: 1.
    /// ```xml
    /// <interface name="wl_subcompositor" version="1" />
    /// ```
    WlSubcompositor { version: u32 },
    /// Object of interface wl_subsurface; maximum supported version: 1.
    /// ```xml
    /// <interface name="wl_subsurface" version="1" />
    /// ```
    WlSubsurface { version: u32 },
    /// Object of interface wl_fixes; maximum supported version: 2.
    /// ```xml
    /// <interface name="wl_fixes" version="2" />
    /// ```
    WlFixes { version: u32 },
}
impl ObjectInteface {
    /// Returns the interface name defined in XML.
    pub fn name(self) -> &'static str {
        match self {
            Self::WlDisplay { .. } => "wl_display",
            Self::WlRegistry { .. } => "wl_registry",
            Self::WlCallback { .. } => "wl_callback",
            Self::WlCompositor { .. } => "wl_compositor",
            Self::WlShmPool { .. } => "wl_shm_pool",
            Self::WlShm { .. } => "wl_shm",
            Self::WlBuffer { .. } => "wl_buffer",
            Self::WlDataOffer { .. } => "wl_data_offer",
            Self::WlDataSource { .. } => "wl_data_source",
            Self::WlDataDevice { .. } => "wl_data_device",
            Self::WlDataDeviceManager { .. } => "wl_data_device_manager",
            Self::WlShell { .. } => "wl_shell",
            Self::WlShellSurface { .. } => "wl_shell_surface",
            Self::WlSurface { .. } => "wl_surface",
            Self::WlSeat { .. } => "wl_seat",
            Self::WlPointer { .. } => "wl_pointer",
            Self::WlKeyboard { .. } => "wl_keyboard",
            Self::WlTouch { .. } => "wl_touch",
            Self::WlOutput { .. } => "wl_output",
            Self::WlRegion { .. } => "wl_region",
            Self::WlSubcompositor { .. } => "wl_subcompositor",
            Self::WlSubsurface { .. } => "wl_subsurface",
            Self::WlFixes { .. } => "wl_fixes",
        }
    }
    /// Returns the negotiated object version.
    pub fn version(self) -> u32 {
        match self {
            Self::WlDisplay { version } => version,
            Self::WlRegistry { version } => version,
            Self::WlCallback { version } => version,
            Self::WlCompositor { version } => version,
            Self::WlShmPool { version } => version,
            Self::WlShm { version } => version,
            Self::WlBuffer { version } => version,
            Self::WlDataOffer { version } => version,
            Self::WlDataSource { version } => version,
            Self::WlDataDevice { version } => version,
            Self::WlDataDeviceManager { version } => version,
            Self::WlShell { version } => version,
            Self::WlShellSurface { version } => version,
            Self::WlSurface { version } => version,
            Self::WlSeat { version } => version,
            Self::WlPointer { version } => version,
            Self::WlKeyboard { version } => version,
            Self::WlTouch { version } => version,
            Self::WlOutput { version } => version,
            Self::WlRegion { version } => version,
            Self::WlSubcompositor { version } => version,
            Self::WlSubsurface { version } => version,
            Self::WlFixes { version } => version,
        }
    }
    /// Returns the highest supported version of this interface.
    pub fn max_version(self) -> u32 {
        match self {
            Self::WlDisplay { .. } => 1,
            Self::WlRegistry { .. } => 1,
            Self::WlCallback { .. } => 1,
            Self::WlCompositor { .. } => 7,
            Self::WlShmPool { .. } => 3,
            Self::WlShm { .. } => 3,
            Self::WlBuffer { .. } => 1,
            Self::WlDataOffer { .. } => 4,
            Self::WlDataSource { .. } => 4,
            Self::WlDataDevice { .. } => 4,
            Self::WlDataDeviceManager { .. } => 4,
            Self::WlShell { .. } => 1,
            Self::WlShellSurface { .. } => 1,
            Self::WlSurface { .. } => 7,
            Self::WlSeat { .. } => 11,
            Self::WlPointer { .. } => 11,
            Self::WlKeyboard { .. } => 11,
            Self::WlTouch { .. } => 11,
            Self::WlOutput { .. } => 4,
            Self::WlRegion { .. } => 7,
            Self::WlSubcompositor { .. } => 1,
            Self::WlSubsurface { .. } => 1,
            Self::WlFixes { .. } => 2,
        }
    }
    /// Creates a known interface, rejecting zero or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "wl_display" => Self::WlDisplay { version },
            "wl_registry" => Self::WlRegistry { version },
            "wl_callback" => Self::WlCallback { version },
            "wl_compositor" => Self::WlCompositor { version },
            "wl_shm_pool" => Self::WlShmPool { version },
            "wl_shm" => Self::WlShm { version },
            "wl_buffer" => Self::WlBuffer { version },
            "wl_data_offer" => Self::WlDataOffer { version },
            "wl_data_source" => Self::WlDataSource { version },
            "wl_data_device" => Self::WlDataDevice { version },
            "wl_data_device_manager" => Self::WlDataDeviceManager { version },
            "wl_shell" => Self::WlShell { version },
            "wl_shell_surface" => Self::WlShellSurface { version },
            "wl_surface" => Self::WlSurface { version },
            "wl_seat" => Self::WlSeat { version },
            "wl_pointer" => Self::WlPointer { version },
            "wl_keyboard" => Self::WlKeyboard { version },
            "wl_touch" => Self::WlTouch { version },
            "wl_output" => Self::WlOutput { version },
            "wl_region" => Self::WlRegion { version },
            "wl_subcompositor" => Self::WlSubcompositor { version },
            "wl_subsurface" => Self::WlSubsurface { version },
            "wl_fixes" => Self::WlFixes { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
