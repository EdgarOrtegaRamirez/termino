# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | ✅ Active development |

## Reporting a Vulnerability

If you discover a security vulnerability in Termino, please report it by opening an issue on the GitHub repository.

We take all security reports seriously. We will investigate and respond promptly.

## Security Considerations

### Data Storage
- Session logs are stored in `~/.termino/sessions.json` with user-only read/write permissions
- No sensitive data (passwords, tokens, API keys) is ever stored by Termino
- Session data is local only — never sent to any external service

### Input Validation
- Duration strings are parsed with strict validation — only `h`, `m`, `s` units are accepted
- Negative and zero durations are rejected with clear error messages
- Session type filters are validated against a whitelist of allowed values

### File Operations
- All file operations use safe paths under `~/.termino/`
- JSON parsing uses `json.load()` which is safe from code injection
- File writes use atomic write patterns (write to temp, rename)

### No External Network Calls
- Termino makes no external network calls
- No telemetry, analytics, or tracking
- No dependency on external APIs

### Desktop Notifications
- Notifications are sent via `notify-send` (Linux) or terminal bell
- No notification data is transmitted over the network

## Dependencies

Dependencies are pinned to specific versions in `pyproject.toml` to prevent supply chain attacks. Please keep dependencies updated.