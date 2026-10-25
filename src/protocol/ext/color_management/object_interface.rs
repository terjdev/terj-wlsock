//! Identifies Color management interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    WpColorManagerV1 { version: u32 },
    WpColorManagementOutputV1 { version: u32 },
    WpColorManagementSurfaceV1 { version: u32 },
    WpColorManagementSurfaceFeedbackV1 { version: u32 },
    WpImageDescriptionCreatorIccV1 { version: u32 },
    WpImageDescriptionCreatorParamsV1 { version: u32 },
    WpImageDescriptionV1 { version: u32 },
    WpImageDescriptionInfoV1 { version: u32 },
    WpImageDescriptionReferenceV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::WpColorManagerV1 { .. } => "wp_color_manager_v1",
            Self::WpColorManagementOutputV1 { .. } => "wp_color_management_output_v1",
            Self::WpColorManagementSurfaceV1 { .. } => "wp_color_management_surface_v1",
            Self::WpColorManagementSurfaceFeedbackV1 { .. } => {
                "wp_color_management_surface_feedback_v1"
            }
            Self::WpImageDescriptionCreatorIccV1 { .. } => "wp_image_description_creator_icc_v1",
            Self::WpImageDescriptionCreatorParamsV1 { .. } => {
                "wp_image_description_creator_params_v1"
            }
            Self::WpImageDescriptionV1 { .. } => "wp_image_description_v1",
            Self::WpImageDescriptionInfoV1 { .. } => "wp_image_description_info_v1",
            Self::WpImageDescriptionReferenceV1 { .. } => "wp_image_description_reference_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::WpColorManagerV1 { version } => version,
            Self::WpColorManagementOutputV1 { version } => version,
            Self::WpColorManagementSurfaceV1 { version } => version,
            Self::WpColorManagementSurfaceFeedbackV1 { version } => version,
            Self::WpImageDescriptionCreatorIccV1 { version } => version,
            Self::WpImageDescriptionCreatorParamsV1 { version } => version,
            Self::WpImageDescriptionV1 { version } => version,
            Self::WpImageDescriptionInfoV1 { version } => version,
            Self::WpImageDescriptionReferenceV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::WpColorManagerV1 { .. } => 3,
            Self::WpColorManagementOutputV1 { .. } => 3,
            Self::WpColorManagementSurfaceV1 { .. } => 3,
            Self::WpColorManagementSurfaceFeedbackV1 { .. } => 3,
            Self::WpImageDescriptionCreatorIccV1 { .. } => 3,
            Self::WpImageDescriptionCreatorParamsV1 { .. } => 3,
            Self::WpImageDescriptionV1 { .. } => 3,
            Self::WpImageDescriptionInfoV1 { .. } => 3,
            Self::WpImageDescriptionReferenceV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "wp_color_manager_v1" => Self::WpColorManagerV1 { version },
            "wp_color_management_output_v1" => Self::WpColorManagementOutputV1 { version },
            "wp_color_management_surface_v1" => Self::WpColorManagementSurfaceV1 { version },
            "wp_color_management_surface_feedback_v1" => {
                Self::WpColorManagementSurfaceFeedbackV1 { version }
            }
            "wp_image_description_creator_icc_v1" => {
                Self::WpImageDescriptionCreatorIccV1 { version }
            }
            "wp_image_description_creator_params_v1" => {
                Self::WpImageDescriptionCreatorParamsV1 { version }
            }
            "wp_image_description_v1" => Self::WpImageDescriptionV1 { version },
            "wp_image_description_info_v1" => Self::WpImageDescriptionInfoV1 { version },
            "wp_image_description_reference_v1" => Self::WpImageDescriptionReferenceV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
