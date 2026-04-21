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

**IMPORTANT MESSAGE: THIS PROJECT IS NOT LIBRARY. IF YOU WANT THIS TO LIBRARY, CHECK libzeroback**

## Installation

```bash
cargo install --path .
```

## Usage

Create a TOML configuration file (e.g., `config.toml`):

```toml
[target]
url = "http://example.com"
method = "GET"
port = "80"
timeout = 30

[request]
header = """
User-Agent: ZeroBack/1.0
Accept: application/json
"""
```

Run the tool:

```bash
zeroback --config config.toml
```

## Configuration Format

The TOML file must contain two sections:

### [target]
- `url`: Target URL (string)
- `method`: HTTP method (GET, POST, PUT, PATCH, DELETE) (string)
- `port`: Target port (string)
- `timeout`: Request timeout in seconds (integer)

### [request]
- `header`: Raw header string (multiline string)
  - Each header on a new line
  - Format: `Key: Value`
  - Trailing commas are ignored
  - Empty lines are skipped
- `body`: Optional request body for POST, PUT, and PATCH requests (string)

## Examples

### GET Request

```toml
[target]
url = "http://httpbin.org"
method = "GET"
port = "80"
timeout = 10

[request]
header = """
User-Agent: TestClient
Accept: */*
"""
```

### POST Request with JSON Body

```toml
[target]
url = "http://httpbin.org"
method = "POST"
port = "80"
timeout = 10

[request]
header = """
Content-Type: application/json
Authorization: Bearer token123
"""
body = """{"key": "value", "message": "hello world"}"""
```

## Building from Source

```bash
git clone <repository-url>
cd zeroback
cargo build --release
```

The binary will be available at `target/release/zeroback`.

## License

LGPL-3.0-only
