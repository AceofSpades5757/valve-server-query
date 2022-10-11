# Valve Server Query

# Description

Crate allowing access to Valve's _Server Query_ protocol.

- :rocket: Blazingly Fast
- :lock: Type Safe
- :zap: Efficient

Want to use it from Python? You can do that!
Want to us it from JavaScript? You can do that too!

Rust is an accessible language that can be utilized in most any scenario, and become more accessible each day!

# Installation

`cargo install valve-server-query`

In your `Cargo.toml` file.

```toml
[dependencies]
valve-server-query = "0.3.5"
```

# Usage

```rust
use valve_server_query::Client;

let client = Client::new("ip:port").expects("Failed to connect to the server.");

let info = client.info().expects("Failed to get server info.");
let players = client.players().expects("Failed to get server players.");
let rules = client.rules().expects("Failed to get server rules.");
```

# Contributing

Pull requests are welcome.

Requests for additional features are welcome: Create a GitHub Issue.

If you would like additional content, such as the abiltiy to query the [Master Server Query Protocol](https://developer.valvesoftware.com/wiki/Master_Server_Query_Protocol), then feel free to contact me or create a GitHub Issue.

# Support

Feel free to create an issue if you experience any problems with this package.
