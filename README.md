[![crates.io](https://img.shields.io/crates/v/valve-server-query?style=plastic)](https://crates.io/crates/valve-server-query)
[![Documentation](https://img.shields.io/badge/docs-passing-green?style=platic)](https://docs.rs/valve-server-query/0.4.5/valve_server_query/)

# Description

Crate allowing access to Valve's _Server Query_ protocol.

- 🚀 Blazingly Fast
- 🔒 Type Safe
- ⚡ Efficient

Want to use it from Python? You can do that!
Want to us it from JavaScript? You can do that too!

Rust is an accessible language that can be utilized in most any scenario, and become more accessible each day!

# Installation

In your `Cargo.toml` file.

```toml
[dependencies]
valve-server-query = "0.4"
```

Or, using the `cargo add` subcommand.

```sh
cargo add valve-server-query
```

# Usage

```rust
use valve_server_query::Server;

let server = Server::new("127.0.0.1:12345").expect("Connect to dedicated server running Valve game");

let info = server.info().expect("Get general server information");
let players = server.players().expect("Get server player information");
let rules = server.rules().expect("Get server rules");
```

## WebAssembly (Wasm)

This will panic in a Wasm environment (`target = "wasm32"`) due to UDP sockets not being supported.

# Contributing

Pull requests are welcome.

Requests for additional features are welcome: Create a GitHub Issue.

If you would like additional content, such as the abiltiy to query the [Master Server Query Protocol](https://developer.valvesoftware.com/wiki/Master_Server_Query_Protocol), then feel free to contact me or create a GitHub Issue.

# Support

Feel free to create an issue if you experience any problems with this package.
