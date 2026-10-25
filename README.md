<div align="center">

# terj-wlsock

[![crates.io](https://img.shields.io/crates/v/terj-wlsock?style=for-the-badge&logo=rust&color=E57324)](https://crates.io/crates/terj-wlsock)
[![Rust](https://img.shields.io/badge/Rust-2024-DEA584?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-36BC9B?style=for-the-badge)](LICENSE.md)
[![Buy Me a Coffee](https://img.shields.io/badge/Buy_Me_a_Coffee-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=000000)](https://buymeacoffee.com/terjdev)

</div>

An asynchronous, pure Rust implementation of the Wayland transport protocol for
clients and servers. It uses Tokio to exchange typed requests and events over
bounded channels, with automatic message encoding and decoding over Unix sockets.

Supports the Wayland core protocol, the stable, staging, unstable, and experimental
extensions from wayland-protocols listed below, and all wlr-protocols extensions.

The library does not implement compositor behavior, rendering or
window management. It only transports typed protocol messages between clients
and servers, including their encoding and decoding.

## Get started

```sh
cargo add terj-wlsock
```

`terj_wlsock::client::Socket::create` and `terj_wlsock::server::Socket::create` returns a socket handle and two thread-safe message queues based
on `tokio::sync::mpsc`: a command sender and a notification receiver. Socket I/O
runs asynchronously in the background until `close()` completes or the
handle is dropped.

### Client

```rust
use terj_wlsock::client::{Socket, Options};
use tokio::net::UnixStream;

let options = Options::default();
let stream = UnixStream::connect(/* socket path */).await?;
let (_handle, requests, mut events) = Socket::create(stream, options)?;

requests.send(/* client::CommandSend */).await?;
while let Some(message) = events.recv().await {
    // Handle client::CommandRecv.
}
```

### Server

```rust
use terj_wlsock::server::{Socket, Options};
use tokio::net::UnixListener;

let options = Options::default();
let listener = UnixListener::bind(/* socket path */)?;
let (_handle, events, mut requests) = Socket::create(listener, options)?;

while let Some(message) = requests.recv().await {
    // Handle server::CommandRecv.
    events.send(/* server::CommandSend */).await?;
}
```

## How it works

`Socket::create` takes an existing Tokio Unix stream or Unix listener and returns
`(handle, command_sender, command_receiver)`.

Keep the socket handle alive for as long as you need the client connection or server listener alive.
The command sender can be cloned and used from multiple tasks; the receiver can
be moved to a dedicated notification loop. A successful channel send means the
command was queued; it does not acknowledge delivery or processing by the peer.

### Options

Both client and server options expose `send_queue_limit` and `recv_queue_limit`,
each defaulting to **128 messages**. Both values must be nonzero.

### Shutdown

Use `handle.close().await?` to request graceful shutdown and wait for background tasks.
Continue draining notifications concurrently until the receiver returns
`None`. Waiting
for `close()` before draining a full queue can block shutdown.

Closing cancels socket I/O; it does not guarantee that every queued message is
flushed. Dropping the socket handle aborts its background task without waiting
for cleanup.

### Features

Only the [core Wayland protocol](https://wayland.app/protocols/wayland) is enabled by default (`default = []`). Explicitly
select the extensions your application needs through Cargo features:

```sh
cargo add terj-wlsock --features ext-xdg-shell,ext-dmabuf
```

Each individual `ext-*` or `wlr-*` feature enables that protocol's types and
request/event dispatch. Use `ext-stable`, `ext-staging`, `ext-unstable`, or
`ext-experimental` to enable a category. Use `ext` for all four categories,
`wlr` for all listed WLR extensions, or `all` for both groups:

```sh
cargo add terj-wlsock --features all
```

Features are additive across the dependency graph. They enable codecs at compile
time; applications still discover globals and negotiate interface versions at
runtime. Object-ID references to another protocol do not automatically enable its
feature: select each protocol you intend to use. Shared codec types do enable
dependencies: `ext-xx-image-capture-color-management` enables `ext-color-management`
for created objects, and `ext-xx-input-method` enables `ext-xx-text-input` for enums.

**Versions** shows the supported interface-version range for each protocol
family. Individual interfaces may have a lower maximum.

#### Wayland extensions (`ext`)

Each protocol family appears once, in the category containing its lowest supported
version, even when higher versions belong to another category.

##### Stable

| Extension | Cargo feature | Versions | Purpose |
|---|---|---|---|
| [linux-dmabuf](https://wayland.app/protocols/linux-dmabuf-v1) | `ext-dmabuf` | 1-6 | DMA-BUF buffers, modifiers and allocation feedback. |
| [presentation-time](https://wayland.app/protocols/presentation-time) | `ext-presentation-time` | 1-3 | Presentation timestamps and refresh feedback. |
| [tablet](https://wayland.app/protocols/tablet-v2) | `ext-tablet` | 1-2 | Tablet-v2 tools, pads, groups, rings, strips and dials. |
| [viewporter](https://wayland.app/protocols/viewporter) | `ext-viewporter` | 1 | Surface cropping and scaling. |
| [xdg-shell](https://wayland.app/protocols/xdg-shell) | `ext-xdg-shell` | 1-7 | Stable desktop windows, popups and positioners. |

##### Staging

| Extension | Cargo feature | Versions | Purpose |
|---|---|---|---|
| [alpha-modifier](https://wayland.app/protocols/alpha-modifier-v1) | `ext-alpha-modifier` | 1 | Surface alpha multiplier. |
| [ext-background-effect](https://wayland.app/protocols/ext-background-effect-v1) | `ext-background-effect` | 1 | Blur regions behind translucent surfaces. |
| [color-management](https://wayland.app/protocols/color-management-v1) | `ext-color-management` | 1-3 | ICC, color spaces and HDR metadata. |
| [color-representation](https://wayland.app/protocols/color-representation-v1) | `ext-color-representation` | 1 | Alpha modes, YCbCr coefficients and ranges. |
| [commit-timing](https://wayland.app/protocols/commit-timing-v1) | `ext-commit-timing` | 1 | Earliest presentation time for commits. |
| [content-type](https://wayland.app/protocols/content-type-v1) | `ext-content-type` | 1 | Photo, video and game content hints. |
| [cursor-shape](https://wayland.app/protocols/cursor-shape-v1) | `ext-cursor-shape` | 1-2 | Standard cursor shapes for pointers and tools. |
| [ext-data-control](https://wayland.app/protocols/ext-data-control-v1) | `ext-data-control` | 1 | Clipboard and primary-selection management. |
| [drm-lease](https://wayland.app/protocols/drm-lease-v1) | `ext-drm-lease` | 1 | Lease DRM connectors and resources to clients. |
| [fifo](https://wayland.app/protocols/fifo-v1) | `ext-fifo` | 1 | Refresh-cycle barriers for surface commits. |
| [ext-foreign-toplevel-list](https://wayland.app/protocols/ext-foreign-toplevel-list-v1) | `ext-foreign-toplevel-list` | 1 | Window handles, titles, app IDs and identifiers. |
| [fractional-scale](https://wayland.app/protocols/fractional-scale-v1) | `ext-fractional-scale` | 1 | Preferred surface scale in units of 1/120. |
| [ext-idle-notify](https://wayland.app/protocols/ext-idle-notify-v1) | `ext-idle-notify` | 1-2 | Per-seat idle and resumed notifications. |
| [ext-image-capture-source](https://wayland.app/protocols/ext-image-capture-source-v1) | `ext-image-capture-source` | 1 | Output and window capture-source descriptors. |
| [ext-image-copy-capture](https://wayland.app/protocols/ext-image-copy-capture-v1) | `ext-image-copy-capture` | 1 | Capture sessions, frames, damage and cursors. |
| [linux-drm-syncobj](https://wayland.app/protocols/linux-drm-syncobj-v1) | `ext-linux-drm-syncobj` | 1 | Explicit synchronization with DRM timeline syncobjs. |
| [pointer-warp](https://wayland.app/protocols/pointer-warp-v1) | `ext-pointer-warp` | 1 | Request a surface-relative pointer position. |
| [security-context](https://wayland.app/protocols/security-context-v1) | `ext-security-context` | 1 | Sandbox metadata for new client connections. |
| [ext-session-lock](https://wayland.app/protocols/ext-session-lock-v1) | `ext-session-lock` | 1 | Session locking and per-output lock surfaces. |
| [single-pixel-buffer](https://wayland.app/protocols/single-pixel-buffer-v1) | `ext-single-pixel-buffer` | 1 | One-pixel RGBA buffers for solid colors. |
| [tearing-control](https://wayland.app/protocols/tearing-control-v1) | `ext-tearing-control` | 1 | Per-surface vsync or asynchronous presentation hints. |
| [ext-transient-seat](https://wayland.app/protocols/ext-transient-seat-v1) | `ext-transient-seat` | 1 | Temporary seats for independent input sessions. |
| [ext-workspace](https://wayland.app/protocols/ext-workspace-v1) | `ext-workspace` | 1 | Workspace groups, state and client operations. |
| [xdg-activation](https://wayland.app/protocols/xdg-activation-v1) | `ext-xdg-activation` | 1 | Activation tokens and application focus requests. |
| [xdg-dialog](https://wayland.app/protocols/xdg-dialog-v1) | `ext-xdg-dialog` | 1 | Dialog relationships and modal hints. |
| [xdg-session-management](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/xdg-session-management/xdg-session-management-v1.xml) | `ext-xdg-session-management` | 1 | Persistent sessions and window-state restoration. |
| [xdg-system-bell](https://wayland.app/protocols/xdg-system-bell-v1) | `ext-xdg-system-bell` | 1 | Audible or visual bell requests. |
| [xdg-toplevel-drag](https://wayland.app/protocols/xdg-toplevel-drag-v1) | `ext-xdg-toplevel-drag` | 1 | Move toplevels during drag-and-drop. |
| [xdg-toplevel-icon](https://wayland.app/protocols/xdg-toplevel-icon-v1) | `ext-xdg-toplevel-icon` | 1 | Per-window icons from names or buffers. |
| [xdg-toplevel-tag](https://wayland.app/protocols/xdg-toplevel-tag-v1) | `ext-xdg-toplevel-tag` | 1 | Window tags and translated descriptions. |
| [xwayland-shell](https://wayland.app/protocols/xwayland-shell-v1) | `ext-xwayland-shell` | 1 | Associate Wayland surfaces with X11 windows. |

##### Unstable

| Extension | Cargo feature | Versions | Purpose |
|---|---|---|---|
| [fullscreen-shell](https://wayland.app/protocols/fullscreen-shell-unstable-v1) | `ext-fullscreen-shell` | 1 | Legacy fullscreen surfaces and display modes. |
| [idle-inhibit](https://wayland.app/protocols/idle-inhibit-unstable-v1) | `ext-idle-inhibit` | 1 | Prevent idle behavior for active surface content. |
| [input-method](https://wayland.app/protocols/input-method-unstable-v1) | `ext-input-method` | 1 | Legacy input-method-v1 contexts and input panels. |
| [input-timestamps](https://wayland.app/protocols/input-timestamps-unstable-v1) | `ext-input-timestamps` | 1 | High-resolution timestamps for input events. |
| [keyboard-shortcuts-inhibit](https://wayland.app/protocols/keyboard-shortcuts-inhibit-unstable-v1) | `ext-keyboard-shortcuts-inhibit` | 1 | Per-surface suppression of compositor shortcuts. |
| [linux-explicit-synchronization](https://wayland.app/protocols/linux-explicit-synchronization-unstable-v1) | `ext-linux-explicit-synchronization` | 1-2 | Legacy sync-file acquire and release fences. |
| [pointer-constraints](https://wayland.app/protocols/pointer-constraints-unstable-v1) | `ext-pointer-constraints` | 1 | Lock or confine pointers to surfaces. |
| [pointer-gestures](https://wayland.app/protocols/pointer-gestures-unstable-v1) | `ext-pointer-gestures` | 1-3 | Swipe, pinch and hold gestures. |
| [primary-selection](https://wayland.app/protocols/primary-selection-unstable-v1) | `ext-primary-selection` | 1 | Primary selection and MIME-based data transfer. |
| [relative-pointer](https://wayland.app/protocols/relative-pointer-unstable-v1) | `ext-relative-pointer` | 1 | Accelerated and raw relative pointer motion. |
| [text-input (v1, v3)](https://wayland.app/protocols/text-input-unstable-v3) | `ext-text-input` | 1-2 | Text-input-v1/v3 composition, IME state and actions. |
| [xdg-decoration](https://wayland.app/protocols/xdg-decoration-unstable-v1) | `ext-xdg-decoration` | 1-2 | Client/server window-decoration negotiation. |
| [xdg-foreign (v1, v2)](https://wayland.app/protocols/xdg-foreign-unstable-v2) | `ext-xdg-foreign` | 1 | v1/v2 cross-client surface-parent relationships. |
| [xdg-output](https://wayland.app/protocols/xdg-output-unstable-v1) | `ext-xdg-output` | 1-3 | Logical output geometry, names and descriptions. |
| [xwayland-keyboard-grab](https://wayland.app/protocols/xwayland-keyboard-grab-unstable-v1) | `ext-xwayland-keyboard-grab` | 1 | Xwayland keyboard-grab coordination. |

#### Experimental extensions (`ext`)

Experimental names retain the upstream `xx-` prefix in Cargo features and
protocol names, and use `xx_` in Rust module names. These features are included in
`ext` and `all`, and remain disabled by default. Experimental protocols coexist
with their stable or unstable counterparts as distinct wire interfaces.

| Extension | Cargo feature | Versions | Purpose |
|---|---|---|---|
| [xx-cutouts](https://wayland.app/protocols/xx-cutouts-v1) | `ext-xx-cutouts` | 1 | Hardware display cutouts and rounded corners. |
| [xx-fractional-scale](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/experimental/xx-fractional-scale/xx-fractional-scale-v2.xml) | `ext-xx-fractional-scale` | 1 | Unsigned 8.24 surface scale factors and client scale requests. |
| [xx-hotkey](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/experimental/xx-hotkey/xx-hotkey-v1.xml) | `ext-xx-hotkey` | 1 | Client-managed global key and button bindings. |
| [xx-image-capture-color-management](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/experimental/xx-image-capture-color-management/xx-image-capture-color-management-v1.xml) | `ext-xx-image-capture-color-management` | 1 | Capture-source colors and frame image descriptions. |
| [xx-input-method](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/experimental/xx-input-method/xx-input-method-v2.xml) | `ext-xx-input-method` | 1-4 | Input methods, composition actions and positioned popups. |
| [xx-keyboard-filter](https://wayland.app/protocols/xx-keyboard-filter-v1) | `ext-xx-keyboard-filter` | 1 | Input-method keyboard filtering decisions. |
| [xx-session-management](https://wayland.app/protocols/xx-session-management-v1) | `ext-xx-session-management` | 1 | Experimental persistent application sessions. |
| [xx-text-input](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/experimental/xx-text-input/xx-text-input-v3.xml) | `ext-xx-text-input` | 1-3 | Text composition, cursor movement and supported features. |
| [xx-zones](https://wayland.app/protocols/xx-zones-v1) | `ext-xx-zones` | 1 | Client placement preferences within compositor zones. |

#### WLR extensions (`wlr`)

| Extension | Cargo feature | Versions | Purpose |
|---|---|---|---|
| [wlr-data-control](https://wayland.app/protocols/wlr-data-control-unstable-v1) | `wlr-data-control` | 1-2 | Clipboard and primary-selection management. |
| [wlr-export-dmabuf](https://wayland.app/protocols/wlr-export-dmabuf-unstable-v1) | `wlr-export-dmabuf` | 1 | Read-only screen capture through exported DMA-BUFs. |
| [wlr-foreign-toplevel-management](https://wayland.app/protocols/wlr-foreign-toplevel-management-unstable-v1) | `wlr-foreign-toplevel-management` | 1-3 | List windows and request window-management actions. |
| [wlr-gamma-control](https://wayland.app/protocols/wlr-gamma-control-unstable-v1) | `wlr-gamma-control` | 1 | Per-output gamma ramps supplied through FDs. |
| [wlr-input-inhibitor](https://wayland.app/protocols/wlr-input-inhibitor-unstable-v1) | `wlr-input-inhibitor` | 1 | Exclusive input routing for trusted clients. |
| [wlr-layer-shell](https://wayland.app/protocols/wlr-layer-shell-unstable-v1) | `wlr-layer-shell` | 1-5 | Panels, backgrounds, overlays and anchored surfaces. |
| [wlr-output-management](https://wayland.app/protocols/wlr-output-management-unstable-v1) | `wlr-output-management` | 1-4 | Configure heads, modes, layout, scale and VRR. |
| [wlr-output-power-management](https://wayland.app/protocols/wlr-output-power-management-unstable-v1) | `wlr-output-power-management` | 1 | Control output power-saving modes. |
| [wlr-screencopy](https://wayland.app/protocols/wlr-screencopy-unstable-v1) | `wlr-screencopy` | 1-3 | Copy output frames or regions into client buffers. |
| [wlr-virtual-pointer](https://wayland.app/protocols/wlr-virtual-pointer-unstable-v1) | `wlr-virtual-pointer` | 1-2 | Inject pointer motion, buttons and scroll events. |

## License

[![License: MIT](https://img.shields.io/badge/License-MIT-36BC9B?style=for-the-badge)](LICENSE.md)

Wayland protocol definitions and documentation excerpts, including their
extensions and wlr-protocols, retain their original copyright notices and
permission terms. See [protocol third-party licenses](src/protocol/LICENSE.md)
for the core MIT license and individual protocol notices.

## Support Me

[![Buy Me a Coffee](https://img.shields.io/badge/Buy_Me_a_Coffee-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=000000)](https://buymeacoffee.com/terjdev)

If you have found my project useful, I would be very grateful if you would support its development with a voluntary donation.


