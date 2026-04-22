<div align="center">
  <picture>
    <img alt="ZeroBack"
         src="https://raw.githubusercontent.com/Synth1105/zeroback/master/zeroback.svg"
         width="50%">
  </picture>
</div>


# ZeroBack


![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/zeroback?style=for-the-badge&labelColor=%23393552&color=%23232136)

A simple HTTP client tool for sending requests with custom headers, configured via TOML.

## Features

- Parse TOML configuration files for target URL, method, port, timeout, and headers
- Support for GET, POST, PUT, PATCH, DELETE HTTP methods
- Custom header parsing from string format
- Timeout configuration
- Response status, headers, and body output

Important note: This program is not a library. If you are looking for a library, please try searching for libzeroback.

## Installation

```bash
cargo install zeroback
```
Binary is located at ~/.cargo/bin/zb.

## Usage

Create a TOML configuration file (e.g., `config.toml`):

```toml
[target]
url = "http://example.com"
method = "GET"
port = "80"
timeout = 30

[request.header]
User-Agent = "ZeroBack/1.0"
Accept = "application/json"
```

Run the tool:

```bash
zb --config config.toml
```

## Configuration Format

The TOML file must contain two sections:

### [target]
- `url`: Target URL (string)
- `method`: HTTP method (GET, POST, PUT, PATCH, DELETE) (string)
- `port`: Target port (string)
- `timeout`: Request timeout in seconds (integer)

### [request]
- `header`: Key = value
  - Each header on a new line
  - Format: `Key = "Value"`
- `body`: Optional request body for POST, PUT, and PATCH requests 
## Examples

### GET Request

```toml
[target]
url = "http://httpbin.org"
method = "GET"
port = "80"
timeout = 10

[request.header]
User-Agent = "TestClient"
Accept = "*/*"
```

### POST Request with JSON Body

```toml
[target]
url = "http://httpbin.org"
method = "POST"
port = "80"
timeout = 10

[request.header]
User-Agent = "ZeroBack/1.0"
Accept = "application/json"
[request.body]
asdf = "jkl"
foo = "bar"
```

## Building from Source

```bash
git clone <repository-url>
cd zeroback
cargo build --release
```

The binary will be available at `target/release/zb`.

## License

LGPL-3.0-only
