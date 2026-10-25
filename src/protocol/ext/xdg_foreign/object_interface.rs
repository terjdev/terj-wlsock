//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZxdgExporterV1 { version: u32 },
    ZxdgImporterV1 { version: u32 },
    ZxdgExportedV1 { version: u32 },
    ZxdgImportedV1 { version: u32 },
    ZxdgExporterV2 { version: u32 },
    ZxdgImporterV2 { version: u32 },
    ZxdgExportedV2 { version: u32 },
    ZxdgImportedV2 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZxdgExporterV1 { .. } => "zxdg_exporter_v1",
            Self::ZxdgImporterV1 { .. } => "zxdg_importer_v1",
            Self::ZxdgExportedV1 { .. } => "zxdg_exported_v1",
            Self::ZxdgImportedV1 { .. } => "zxdg_imported_v1",
            Self::ZxdgExporterV2 { .. } => "zxdg_exporter_v2",
            Self::ZxdgImporterV2 { .. } => "zxdg_importer_v2",
            Self::ZxdgExportedV2 { .. } => "zxdg_exported_v2",
            Self::ZxdgImportedV2 { .. } => "zxdg_imported_v2",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZxdgExporterV1 { version } => version,
            Self::ZxdgImporterV1 { version } => version,
            Self::ZxdgExportedV1 { version } => version,
            Self::ZxdgImportedV1 { version } => version,
            Self::ZxdgExporterV2 { version } => version,
            Self::ZxdgImporterV2 { version } => version,
            Self::ZxdgExportedV2 { version } => version,
            Self::ZxdgImportedV2 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZxdgExporterV1 { .. } => 1,
            Self::ZxdgImporterV1 { .. } => 1,
            Self::ZxdgExportedV1 { .. } => 1,
            Self::ZxdgImportedV1 { .. } => 1,
            Self::ZxdgExporterV2 { .. } => 1,
            Self::ZxdgImporterV2 { .. } => 1,
            Self::ZxdgExportedV2 { .. } => 1,
            Self::ZxdgImportedV2 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zxdg_exporter_v1" => Self::ZxdgExporterV1 { version },
            "zxdg_importer_v1" => Self::ZxdgImporterV1 { version },
            "zxdg_exported_v1" => Self::ZxdgExportedV1 { version },
            "zxdg_imported_v1" => Self::ZxdgImportedV1 { version },
            "zxdg_exporter_v2" => Self::ZxdgExporterV2 { version },
            "zxdg_importer_v2" => Self::ZxdgImporterV2 { version },
            "zxdg_exported_v2" => Self::ZxdgExportedV2 { version },
            "zxdg_imported_v2" => Self::ZxdgImportedV2 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
