# Logs Analyzer

A minimal, low-level HTTP log analyzer written in Rust.

## Log Format

Expected CSV format per log line:

```text
<DATETIME>,<IP>,<METHOD>,<PATH>,<STATUS>,<DURATION_MS>
```

**Example:**

```text
2026-11-02T10:00:00Z,192.168.1.10,GET,/users,200,45
```
or [Example](./examples/server01.log)

## Usage

```bash
# Analyze a log file
cargo run -- examples/server01.log

# Filter by HTTP method
cargo run -- examples/server01.log --method GET

# Filter by HTTP status code
cargo run -- examples/server01.log --status 500

# View help
cargo run -- --help
```