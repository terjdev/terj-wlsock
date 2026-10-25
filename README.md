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

## License

[![License: MIT](https://img.shields.io/badge/License-MIT-36BC9B?style=for-the-badge)](LICENSE.md)

## Support Me

[![Buy Me a Coffee](https://img.shields.io/badge/Buy_Me_a_Coffee-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=000000)](https://buymeacoffee.com/terjdev)

If you have found my project useful, I would be very grateful if you would support its development with a voluntary donation.


