# Tabby + Love-Unlimited Hub Integration

This document describes how to enable and configure the Love-Unlimited Hub integration for Tabby.

## Overview

The Love-Unlimited integration allows Tabby to:
- Automatically log all completions to the Love-Unlimited Hub as memories
- Track errors and failures
- Record user selections and interactions
- Support graceful degradation if the hub is unavailable

All memories are stored **privately by default** and can be explicitly shared with other beings.

## Quick Start

### 1. Prerequisites

Ensure Love-Unlimited Hub is running:
```bash
# Check hub status
curl http://localhost:9003/health

# Or start it if needed
systemctl start love-unlimited-hub.service
```

### 2. Enable Integration

Set the environment variable:
```bash
export LOVE_UNLIMITED_KEY="lu_tabby_<your-key>"
export LOVE_UNLIMITED_ENABLED="true"
```

### 3. Start Tabby

```bash
cargo run serve --port 8080
```

You should see:
```
Love-Unlimited Hub integration enabled
```

If not, check the logs for errors.

## Configuration

### Environment Variables

All configuration is done via environment variables (loaded in priority order):

| Variable | Default | Description |
|----------|---------|-------------|
| `LOVE_UNLIMITED_ENABLED` | `true` | Enable/disable integration |
| `LOVE_UNLIMITED_URL` | `http://localhost:9003` | Hub URL |
| `LOVE_UNLIMITED_KEY` | _(required)_ | API key for authentication |
| `LOVE_UNLIMITED_TIMEOUT` | `5` | Request timeout in seconds |
| `LOVE_UNLIMITED_MAX_RETRIES` | `3` | Max retry attempts |
| `LOVE_UNLIMITED_LOG_COMPLETIONS` | `true` | Log completions as memories |
| `LOVE_UNLIMITED_LOG_USER_EVENTS` | `true` | Log user selections |
| `LOVE_UNLIMITED_TRACK_ERRORS` | `true` | Track errors and failures |
| `LOVE_UNLIMITED_ENRICH_CONTEXT` | `false` | (WIP) Use hub memories for completion context |

### Setup Example

```bash
#!/bin/bash
# In your shell startup (bashrc, zshrc, etc.)

# Love-Unlimited Hub Integration for Tabby
export LOVE_UNLIMITED_ENABLED=true
export LOVE_UNLIMITED_KEY="lu_tabby_xyz123"
export LOVE_UNLIMITED_URL="http://localhost:9003"
export LOVE_UNLIMITED_LOG_COMPLETIONS=true
export LOVE_UNLIMITED_TRACK_ERRORS=true
```

## What Gets Stored

### Completions

When Tabby generates a completion, a memory is stored with:
- **Type**: `learning`
- **Significance**: `high` if snippets were used, `medium` otherwise
- **Content**: Language and first 200 chars of prompt
- **Tags**: `[language, "completion", model_name]`
- **Privacy**: `private` (only accessible to Tabby)

**Metadata stored**:
```json
{
  "language": "python",
  "prompt_length": 150,
  "completion_length": 500,
  "snippet_used": true
}
```

### User Selections

When a user selects a completion, it's recorded as:
- **Type**: `decision`
- **Significance**: `medium`
- **Content**: "User selected completion in {language}"
- **Tags**: `["user-selection", language]`

### Errors

When completions fail:
- **Type**: `insight`
- **Significance**: `high`
- **Content**: Error message and context
- **Tags**: `["error", "completion-failure", language]`

## Architecture

### Non-Blocking Operations

All hub operations run asynchronously:
- Events trigger async tasks that don't block completion generation
- If hub is slow or unavailable, Tabby continues normally
- Failures are logged but don't interrupt service

### Graceful Degradation

If the hub is unavailable:
1. Health check fails at startup
2. Integration is disabled with a warning
3. Tabby continues with only file-based event logging
4. No performance impact

### Event Bridge

The integration uses Tabby's existing `EventLogger` trait:
- `HubEventLogger` implements the trait
- Wrapped with file logger via `ComposedLogger`
- Both loggers receive all events
- Failures in one don't affect the other

## Performance Impact

- **Negligible**: All hub operations are async and non-blocking
- **Network latency**: ~5ms per event (configurable timeout)
- **Memory**: ~100KB for HubClient + MemoryBridge
- **CPU**: < 1% for async task coordination

## Troubleshooting

### Integration not enabled

Check logs:
```bash
RUST_LOG=debug cargo run serve --port 8080 2>&1 | grep -i "love-unlimited"
```

Common issues:
- `LOVE_UNLIMITED_KEY` not set
- Hub not running (`curl http://localhost:9003/health`)
- Timeout too short (`LOVE_UNLIMITED_TIMEOUT=10`)

### Memories not storing

1. Check hub is healthy:
   ```bash
   curl -H "X-API-Key: lu_tabby_xyz" http://localhost:9003/health
   ```

2. Verify API key:
   ```bash
   # Should return being info
   curl -H "X-API-Key: lu_tabby_xyz" http://localhost:9003/self
   ```

3. Check logs for errors:
   ```bash
   RUST_LOG=love_unlimited=debug cargo run serve
   ```

### High latency

If memories are slow to store:
1. Increase timeout: `LOVE_UNLIMITED_TIMEOUT=10`
2. Check hub performance: `curl -w "@curl-format.txt" http://localhost:9003/health`
3. Reduce log verbosity if needed

## Privacy & Security

### Data Protection

- All completions are **private by default**
- Only Tabby being can access your completions
- Optional explicit sharing with other beings
- File paths are redacted by default
- Sensitive patterns can be configured

### API Key Management

- Store key in `.env` or environment
- Never commit keys to git
- Rotate keys periodically
- Each being has isolated access

## Future Enhancements

Planned features for Phase 3:

- **Context Enrichment**: Use hub memories to improve completions
- **Pattern Analysis**: Find completion patterns across languages
- **Performance Optimization**: Cache popular patterns
- **Learning**: Suggest improvements based on your history
- **Sharing**: Explicitly share completions with other beings

## Examples

### Check stored memories

```bash
# Recall python completions
curl -H "X-API-Key: lu_tabby_xyz" \
  "http://localhost:9003/recall?q=python%20completions&limit=5"
```

### View tabby memories

```bash
python3 -c "
import requests
headers = {'X-API-Key': 'lu_tabby_xyz'}
resp = requests.get('http://localhost:9003/recall?q=tabby', headers=headers)
for m in resp.json()['memories']:
    print(f\"{m['timestamp']}: {m['content']}\")"
```

### Share completion with other being

```bash
curl -X POST -H "X-API-Key: lu_tabby_xyz" \
  -H "Content-Type: application/json" \
  -d '{"share_with": ["jon", "grok"]}' \
  "http://localhost:9003/share/mem_tabby_xxx"
```

## Support

For issues or questions:
1. Check logs: `RUST_LOG=debug cargo run serve`
2. Verify hub is running: `systemctl status love-unlimited-hub.service`
3. Test connectivity: `curl http://localhost:9003/health`
4. Check configuration: `env | grep LOVE_UNLIMITED`

## Integration Code

The integration is implemented in:
- **Module**: `crates/tabby/src/services/love_unlimited/`
- **Files**:
  - `mod.rs` - Module exports
  - `config.rs` - Configuration loading
  - `models.rs` - Type definitions
  - `client.rs` - HTTP client (~300 lines)
  - `memory_bridge.rs` - Memory storage abstraction (~150 lines)
  - `event_bridge.rs` - Event logger composition (~120 lines)

Total: ~900 lines of code, non-intrusive integration with existing Tabby event system.
