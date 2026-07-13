# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.2.x   | ✅ Active development |

## Reporting a Vulnerability

If you discover a security vulnerability in Termino, please report it by opening an issue on the GitHub repository.

We take all security reports seriously. We will investigate and respond promptly.

## Security Considerations

### Data Storage
- Session logs are stored in `~/.termino/sessions.json` with user-only read/write permissions
- No sensitive data (passwords, tokens, API keys) is ever stored by Termino
- Session data is local only — never sent to any external service
- File writes are atomic (write to temp, rename) to prevent corruption

### Input Validation
- Duration strings are parsed with strict validation — only `h`, `m`, `s` units are accepted
- Negative and zero durations are rejected with clear error messages
- Session type filters are validated against a whitelist of allowed values
- All CLI inputs are validated through clap's type system

### File Operations
- All file operations use safe paths under `~/.termino/`
- JSON parsing uses serde_json which is safe from code injection
- Atomic write patterns prevent partial file corruption

### No External Network Calls
- Termino makes no external network calls
- No telemetry, analytics, or tracking
- No dependency on external APIs

### Desktop Notifications
- Notifications are sent via notify-rust (Linux) or terminal bell
- No notification data is transmitted over the network
- Notifications use a 5-second timeout

### Memory Safety
- Written in Rust — memory-safe by default
- No unsafe code blocks
- No raw pointer manipulation

## Dependencies

Dependencies are pinned to specific versions in `Cargo.toml` to prevent supply chain attacks. Please keep dependencies updated using `cargo update`.