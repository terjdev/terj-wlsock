//! Encodes and decodes the ext-workspace extension.
//!
//! The manager announces groups and workspaces with server-created handles. Updates
//! are grouped by done; client operations are applied through manager commit.
//! Coordinate arrays contain native-endian u32 values. The caller manages groups,
//! capabilities, workspace state, names, activation, assignment and removal.
//!
//! Codecs validate framing, receiver metadata, message versions and new object IDs.
//! Child versions inherit the parent version, capped at the target maximum; core
//! buffers use version 1. New IDs are registered before writes. A failed or
//! cancelled write after registration requires closing the connection. Client IDs
//! remain reserved until `wl_display.delete_id`; destructors release server IDs.
//! Strings and fixed values are checked for wire encoding. Nullable arguments use
//! `Option`; arrays retain raw bytes. Unknown enum values and flag bits are preserved.
//! The caller enforces protocol semantics, object relationships and compositor policy.
//!
//! Defined by upstream [ext-workspace-v1.xml](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/ext-workspace/ext-workspace-v1.xml).
//! Original copyright and permission terms are preserved in LICENSE.md.

mod ext_workspace_manager_v1_commit_request;
#[doc(inline)]
pub use ext_workspace_manager_v1_commit_request::ExtWorkspaceManagerV1CommitRequest;

mod ext_workspace_manager_v1_stop_request;
#[doc(inline)]
pub use ext_workspace_manager_v1_stop_request::ExtWorkspaceManagerV1StopRequest;

mod ext_workspace_manager_v1_workspace_group_event;
#[doc(inline)]
pub use ext_workspace_manager_v1_workspace_group_event::ExtWorkspaceManagerV1WorkspaceGroupEvent;

mod ext_workspace_manager_v1_workspace_event;
#[doc(inline)]
pub use ext_workspace_manager_v1_workspace_event::ExtWorkspaceManagerV1WorkspaceEvent;

mod ext_workspace_manager_v1_done_event;
#[doc(inline)]
pub use ext_workspace_manager_v1_done_event::ExtWorkspaceManagerV1DoneEvent;

mod ext_workspace_manager_v1_finished_event;
#[doc(inline)]
pub use ext_workspace_manager_v1_finished_event::ExtWorkspaceManagerV1FinishedEvent;

mod ext_workspace_group_handle_v1_group_capabilities;
#[doc(inline)]
pub use ext_workspace_group_handle_v1_group_capabilities::ExtWorkspaceGroupHandleV1GroupCapabilities;

mod ext_workspace_group_handle_v1_create_workspace_request;
#[doc(inline)]
pub use ext_workspace_group_handle_v1_create_workspace_request::ExtWorkspaceGroupHandleV1CreateWorkspaceRequest;

mod ext_workspace_group_handle_v1_destroy_request;
#[doc(inline)]
pub use ext_workspace_group_handle_v1_destroy_request::ExtWorkspaceGroupHandleV1DestroyRequest;

mod ext_workspace_group_handle_v1_capabilities_event;
#[doc(inline)]
pub use ext_workspace_group_handle_v1_capabilities_event::ExtWorkspaceGroupHandleV1CapabilitiesEvent;

mod ext_workspace_group_handle_v1_output_enter_event;
#[doc(inline)]
pub use ext_workspace_group_handle_v1_output_enter_event::ExtWorkspaceGroupHandleV1OutputEnterEvent;

mod ext_workspace_group_handle_v1_output_leave_event;
#[doc(inline)]
pub use ext_workspace_group_handle_v1_output_leave_event::ExtWorkspaceGroupHandleV1OutputLeaveEvent;

mod ext_workspace_group_handle_v1_workspace_enter_event;
#[doc(inline)]
pub use ext_workspace_group_handle_v1_workspace_enter_event::ExtWorkspaceGroupHandleV1WorkspaceEnterEvent;

mod ext_workspace_group_handle_v1_workspace_leave_event;
#[doc(inline)]
pub use ext_workspace_group_handle_v1_workspace_leave_event::ExtWorkspaceGroupHandleV1WorkspaceLeaveEvent;

mod ext_workspace_group_handle_v1_removed_event;
#[doc(inline)]
pub use ext_workspace_group_handle_v1_removed_event::ExtWorkspaceGroupHandleV1RemovedEvent;

mod ext_workspace_handle_v1_state;
#[doc(inline)]
pub use ext_workspace_handle_v1_state::ExtWorkspaceHandleV1State;

mod ext_workspace_handle_v1_workspace_capabilities;
#[doc(inline)]
pub use ext_workspace_handle_v1_workspace_capabilities::ExtWorkspaceHandleV1WorkspaceCapabilities;

mod ext_workspace_handle_v1_destroy_request;
#[doc(inline)]
pub use ext_workspace_handle_v1_destroy_request::ExtWorkspaceHandleV1DestroyRequest;

mod ext_workspace_handle_v1_activate_request;
#[doc(inline)]
pub use ext_workspace_handle_v1_activate_request::ExtWorkspaceHandleV1ActivateRequest;

mod ext_workspace_handle_v1_deactivate_request;
#[doc(inline)]
pub use ext_workspace_handle_v1_deactivate_request::ExtWorkspaceHandleV1DeactivateRequest;

mod ext_workspace_handle_v1_assign_request;
#[doc(inline)]
pub use ext_workspace_handle_v1_assign_request::ExtWorkspaceHandleV1AssignRequest;

mod ext_workspace_handle_v1_remove_request;
#[doc(inline)]
pub use ext_workspace_handle_v1_remove_request::ExtWorkspaceHandleV1RemoveRequest;

mod ext_workspace_handle_v1_id_event;
#[doc(inline)]
pub use ext_workspace_handle_v1_id_event::ExtWorkspaceHandleV1IdEvent;

mod ext_workspace_handle_v1_name_event;
#[doc(inline)]
pub use ext_workspace_handle_v1_name_event::ExtWorkspaceHandleV1NameEvent;

mod ext_workspace_handle_v1_coordinates_event;
#[doc(inline)]
pub use ext_workspace_handle_v1_coordinates_event::ExtWorkspaceHandleV1CoordinatesEvent;

mod ext_workspace_handle_v1_state_event;
#[doc(inline)]
pub use ext_workspace_handle_v1_state_event::ExtWorkspaceHandleV1StateEvent;

mod ext_workspace_handle_v1_capabilities_event;
#[doc(inline)]
pub use ext_workspace_handle_v1_capabilities_event::ExtWorkspaceHandleV1CapabilitiesEvent;

mod ext_workspace_handle_v1_removed_event;
#[doc(inline)]
pub use ext_workspace_handle_v1_removed_event::ExtWorkspaceHandleV1RemovedEvent;

mod command_request;
#[doc(inline)]
pub use command_request::CommandRequest;

mod command_event;
#[doc(inline)]
pub use command_event::CommandEvent;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;
