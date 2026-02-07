# uuid7time

## Project Description

Minimalist Rust utility for extracting timestamps from UUID version 7.

UUID v7 contains a 48-bit timestamp in milliseconds since Unix epoch in the first 6 bytes. This utility extracts and displays this time in human-readable ISO 8601 format.

## Project Structure

```
uuid7time/
├── Cargo.toml          # Dependencies: uuid, chrono
├── Containerfile       # Multi-stage build for static binary
```
uuid7time/
├── Cargo.toml          # Dependencies: uuid, chrono, clap, serde
├── Containerfile       # Multi-stage build for container image
├── Containerfile.build # Quick build for binary export
└── src/
    └── main.rs         # Core timestamp extraction logic
```

## Key Dependencies

- **uuid** (1.8) - UUID parsing
- **chrono** (0.4) - Date and time manipulation
- **clap** (4.5) - CLI argument parsing
- **serde** / **serde_json** (1.0) - JSON serialization

## Functionality

### Core Logic (`extract_timestamp_ms` + `format_timestamp`)

1. Parses UUID string
2. Extracts first 6 bytes (48-bit timestamp in milliseconds)
3. Converts from big-endian to u64
4. Transforms to DateTime using chrono
5. Formats as ISO 8601 with milliseconds

### Input Methods

The utility supports two UUID input methods:
- Command-line argument: `uuid7time 018d5e5e-7b3a-7000-8000-000000000000`
- Via stdin: `echo "018d5e5e-7b3a-7000-8000-000000000000" | uuid7time`
- Multiple UUIDs: `uuid7time UUID1 UUID2 UUID3` or via stdin (one per line)

### Output Formats

Supports multiple output formats via CLI flags:

- **ISO 8601** (default): `2024-01-31T07:14:26.746Z`
- **Unix seconds** (`--unix`, `-u`): `1706685266`
- **Unix milliseconds** (`--unix-ms`, `-U`): `1706685266746`
- **JSON** (`--json`, `-j`): Full structured output with all timestamp formats

Example JSON output:
```json
{
  "uuid": "018d5e5e-7b3a-7000-8000-000000000000",
  "timestamp_ms": 1706685266746,
  "timestamp_sec": 1706685266,
  "iso8601": "2024-01-31T07:14:26.746Z",
  "rfc3339": "2024-01-31T07:14:26.746+00:00"
}
```

## Build

### Quick Build (Binary Export)

Extract static binary directly to host:
```bash
docker build -f Containerfile.build -o . .
```

Binary appears in current directory, ready to use.

### Local Build with Rust
```bash
cargo build --release --target x86_64-unknown-linux-musl
```

### Container Image Build

Build as container for direct execution:
```bash
docker build -t uuid7time -f Containerfile .
echo "018d5e5e-7b3a-7000-8000-000000000000" | docker run -i uuid7time
```

Containerfile uses multi-stage build:
1. **Builder stage**: rust:alpine with musl-dev for static linking
2. **Runtime stage**: scratch (empty image) - static binary requires no runtime

## Error Handling

- Invalid UUID → error message to stderr + exit code 1
- Timestamp out of range → "Timestamp out of range" to stderr + exit code 1
- Stdin read error → graceful error handling, no panic
- `--quiet` flag → suppress error messages, only exit codes
- Batch mode → continues processing on individual UUID errors

## Implementation Features

### Static Linking
- Uses target `x86_64-unknown-linux-musl`
- Final image `FROM scratch` (~2-3 MB)
- Fully autonomous binary with no dependencies

### Performance
- Minimal allocations
- Direct UUID byte manipulation
- No unnecessary data copying

## Instructions for AI Agents

### When modifying code:
- Keep it simple - this is a single-file utility
- All changes in `src/main.rs`
- Test with valid UUID v7 values
- Verify both stdin and argument input methods

### When changing Containerfile:
- Preserve multi-stage build
- Don't change musl target without necessity
- Final image must remain minimal (scratch or distroless)

### When updating dependencies:
- Verify musl compatibility
- Avoid dependencies with dynamic linking
- Test container build

## Typical Use Cases

1. **UUID v7 debugging**: Verify timestamp generation correctness
2. **Logging**: Extract record creation time from UUID
3. **Data analysis**: Convert UUIDs to timestamps for analysis
4. **Pipeline integration**: Use via stdin in scripts

## Examples

```bash
# Single UUID
./uuid7time 018d5e5e-7b3a-7000-8000-000000000000

# From file
cat uuids.txt | ./uuid7time

# In pipeline
generate-uuid-v7 | ./uuid7time

# In container
echo "018d5e5e-7b3a-7000-8000-000000000000" | podman run -i uuid7time
```
