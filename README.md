# logs-analyzer

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