//! Encodes and decodes the xdg-foreign protocol family.
//!
//! Both v1 and v2 exporters/importers support cross-client parent relationships.
//! Export handles are strings; destroyed notifications leave imported objects inert
//! until destruction. The caller validates handles, surface roles and parent cycles.
//!
//! Codecs validate receiver protocol, opcode, minimum version and complete framing.
//! New child IDs inherit the receiver version capped at the target maximum; core
//! keyboard objects use version 1. Registration precedes writes; failed or cancelled
//! writes after registration require closing the connection. Client IDs remain
//! reserved until `wl_display.delete_id`; destructors release server IDs.
//! Fixed values use `f64` with signed 24.8 encoding. Strings and fixed values are
//! validated before writes. Nullable values use `Option`; arrays retain raw bytes.
//! Unknown enum values and flag bits are preserved. Protocol semantics belong to
//! the caller. Original copyright and permission terms are in LICENSE.md.
//! Source: [xdg-foreign-unstable-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/unstable/xdg-foreign/xdg-foreign-unstable-v1.xml).
//! Source: [xdg-foreign-unstable-v2.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/unstable/xdg-foreign/xdg-foreign-unstable-v2.xml).

mod zxdg_exporter_v1_destroy_request;
#[doc(inline)]
pub use zxdg_exporter_v1_destroy_request::ZxdgExporterV1DestroyRequest;

mod zxdg_exporter_v1_export_request;
#[doc(inline)]
pub use zxdg_exporter_v1_export_request::ZxdgExporterV1ExportRequest;

mod zxdg_importer_v1_destroy_request;
#[doc(inline)]
pub use zxdg_importer_v1_destroy_request::ZxdgImporterV1DestroyRequest;

mod zxdg_importer_v1_import_request;
#[doc(inline)]
pub use zxdg_importer_v1_import_request::ZxdgImporterV1ImportRequest;

mod zxdg_exported_v1_destroy_request;
#[doc(inline)]
pub use zxdg_exported_v1_destroy_request::ZxdgExportedV1DestroyRequest;

mod zxdg_exported_v1_handle_event;
#[doc(inline)]
pub use zxdg_exported_v1_handle_event::ZxdgExportedV1HandleEvent;

mod zxdg_imported_v1_destroy_request;
#[doc(inline)]
pub use zxdg_imported_v1_destroy_request::ZxdgImportedV1DestroyRequest;

mod zxdg_imported_v1_set_parent_of_request;
#[doc(inline)]
pub use zxdg_imported_v1_set_parent_of_request::ZxdgImportedV1SetParentOfRequest;

mod zxdg_imported_v1_destroyed_event;
#[doc(inline)]
pub use zxdg_imported_v1_destroyed_event::ZxdgImportedV1DestroyedEvent;

mod zxdg_exporter_v2_error;
#[doc(inline)]
pub use zxdg_exporter_v2_error::ZxdgExporterV2Error;

mod zxdg_exporter_v2_destroy_request;
#[doc(inline)]
pub use zxdg_exporter_v2_destroy_request::ZxdgExporterV2DestroyRequest;

mod zxdg_exporter_v2_export_toplevel_request;
#[doc(inline)]
pub use zxdg_exporter_v2_export_toplevel_request::ZxdgExporterV2ExportToplevelRequest;

mod zxdg_importer_v2_destroy_request;
#[doc(inline)]
pub use zxdg_importer_v2_destroy_request::ZxdgImporterV2DestroyRequest;

mod zxdg_importer_v2_import_toplevel_request;
#[doc(inline)]
pub use zxdg_importer_v2_import_toplevel_request::ZxdgImporterV2ImportToplevelRequest;

mod zxdg_exported_v2_destroy_request;
#[doc(inline)]
pub use zxdg_exported_v2_destroy_request::ZxdgExportedV2DestroyRequest;

mod zxdg_exported_v2_handle_event;
#[doc(inline)]
pub use zxdg_exported_v2_handle_event::ZxdgExportedV2HandleEvent;

mod zxdg_imported_v2_error;
#[doc(inline)]
pub use zxdg_imported_v2_error::ZxdgImportedV2Error;

mod zxdg_imported_v2_destroy_request;
#[doc(inline)]
pub use zxdg_imported_v2_destroy_request::ZxdgImportedV2DestroyRequest;

mod zxdg_imported_v2_set_parent_of_request;
#[doc(inline)]
pub use zxdg_imported_v2_set_parent_of_request::ZxdgImportedV2SetParentOfRequest;

mod zxdg_imported_v2_destroyed_event;
#[doc(inline)]
pub use zxdg_imported_v2_destroyed_event::ZxdgImportedV2DestroyedEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
