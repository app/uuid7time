# uuid7time

**Fast, static binary** for extracting timestamps from UUID version 7.

**1.2 MB · No dependencies · Cross-platform**

UUID v7 embeds a 48-bit timestamp (milliseconds since Unix epoch) in its first 6 bytes. This tool extracts and formats it for human and machine consumption.

## Features

- **🚀 Static binary**: 1.2 MB standalone executable, no runtime dependencies
- **⚡ Built with Rust**: Memory-safe, zero-cost abstractions, blazing fast
- **📦 Cross-platform**: Linux (x86_64, ARM64), macOS Apple Silicon
- **Multiple output formats**: ISO 8601, Unix timestamp (seconds/milliseconds), JSON
- **Batch processing**: Handle multiple UUIDs via stdin or arguments
- **Pipeline-friendly**: Clean output, proper error handling, exit codes
- **Fast**: Direct byte manipulation, minimal allocations

## Installation

### Download Binary

Download the latest static binary from [Releases](https://github.com/app/uuid7time/releases):

**Linux x86_64**:
```bash
curl -L https://github.com/app/uuid7time/releases/latest/download/uuid7time-linux-x86_64 -o uuid7time
chmod +x uuid7time
sudo mv uuid7time /usr/local/bin/
```

**Linux ARM64**:
```bash
curl -L https://github.com/app/uuid7time/releases/latest/download/uuid7time-linux-aarch64 -o uuid7time
chmod +x uuid7time
sudo mv uuid7time /usr/local/bin/
```

**macOS Apple Silicon**:
```bash
curl -L https://github.com/app/uuid7time/releases/latest/download/uuid7time-macos-arm64 -o uuid7time
chmod +x uuid7time
sudo mv uuid7time /usr/local/bin/
```

### Build from Source

**Quick build** (binary exported to current directory):

```bash
git clone https://github.com/app/uuid7time.git
cd uuid7time
docker build -f Containerfile.build -o . .
```

**Or build as container image**:

```bash
docker build -t uuid7time -f Containerfile .
# Run directly: echo "UUID" | docker run -i uuid7time
```

**Or with Rust toolchain**:

```bash
cargo build --release --target x86_64-unknown-linux-musl
```

## Usage

### Basic Examples

```bash
# Single UUID (default ISO 8601 format)
uuid7time 018d5e5e-7b3a-7000-8000-000000000000
# Output: 2024-01-31T07:14:26.746Z

# Unix timestamp (seconds)
uuid7time --unix 018d5e5e-7b3a-7000-8000-000000000000
# Output: 1706685266

# Unix timestamp (milliseconds)
uuid7time --unix-ms 018d5e5e-7b3a-7000-8000-000000000000
# Output: 1706685266746

# JSON output
uuid7time --json 018d5e5e-7b3a-7000-8000-000000000000
# Output: {"uuid":"...","timestamp_ms":1706685266746,"timestamp_sec":1706685266,"iso8601":"...","rfc3339":"..."}
```

### Batch Processing

```bash
# Multiple UUIDs as arguments
uuid7time UUID1 UUID2 UUID3

# From stdin (one per line)
cat uuids.txt | uuid7time --unix

# Extract from logs and sort by time
grep -oE '[0-9a-f-]{36}' app.log | uuid7time --unix | sort -n
```

### Pipeline Integration

```bash
# With jq for JSON processing
cat data.json | jq -r '.events[].id' | uuid7time --json | jq -s '.'

# Filter recent UUIDs (last hour)
cat uuids.txt | uuid7time --unix | awk -v now=$(date +%s) '$1 > now-3600'

# Database export with timestamps
psql -t -c "SELECT id FROM events" | uuid7time --unix-ms > timestamps.csv
```

## Output Formats

| Format | Flag | Example |
|--------|------|---------|
| ISO 8601 | `--iso` (default) | `2024-01-31T07:14:26.746Z` |
| Unix seconds | `--unix`, `-u` | `1706685266` |
| Unix milliseconds | `--unix-ms`, `-U` | `1706685266746` |
| JSON | `--json`, `-j` | `{"uuid":"...","timestamp_ms":...}` |

## Options

```
Usage: uuid7time [OPTIONS] [UUID]...

Arguments:
  [UUID]...  UUID(s) to extract timestamp from

Options:
  -f, --format <FORMAT>  Output format: iso, unix, unix-ms, json [default: iso]
  -u, --unix             Output unix timestamp in seconds
  -U, --unix-ms          Output unix timestamp in milliseconds
  -j, --json             Output JSON format
  -q, --quiet            Suppress error messages
  -h, --help             Print help
  -V, --version          Print version
```

## Use Cases

- **Debugging**: Verify UUID v7 generation correctness
- **Log analysis**: Extract creation timestamps from UUID-based event IDs
- **Data pipelines**: Convert UUIDs to sortable timestamps for analytics
- **Monitoring**: Check if resources were created within expected timeframes

## Technical Details

- **Language**: Rust
- **Dependencies**: uuid, chrono, clap, serde/serde_json
- **Binary size**: ~1.2 MB (static, musl-linked)
- **Platforms**: 
  - Linux x86_64 (static binary, musl)
  - Linux ARM64/aarch64 (static binary, musl)
  - macOS Apple Silicon (ARM64)

See [AGENTS.md](AGENTS.md) for detailed implementation documentation.

## License

Apache 2.0

See [LICENSE](LICENSE) for details.

## Contributing

Issues and pull requests welcome!
