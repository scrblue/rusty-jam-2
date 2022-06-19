# Running
By default the server is in WebRTC mode and can only handle connections from the WASM client. To use
native client, switch the feature on the server from `use-webrtc` to `use-udp`.

Follow the CLI help for the server. Note that the socket address you give the client is the first IP
you give the server, even in WebRTC mode.

Do not use `0.0.0.0` as an IP for the server, it will not work at the moment.

Launch the client in WASM by running `trunk serve` after installing `trunk` and the wasm target for
Rust.
