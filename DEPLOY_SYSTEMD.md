# Deploying Tabby as a Systemd Service

This guide explains how to deploy Tabby as a systemd service with automatic startup and Love-Unlimited Hub integration.

## Prerequisites

- Tabby built: `cargo build --release`
- Love-Unlimited Hub running and registered Tabby being with API key: `lu_tabby_VCnVIGgHq4Lx2VCQPz_y_ggYPrGTRHxW`
- systemd-based Linux system (Ubuntu, Debian, Fedora, etc.)
- Sudo/root access for installing system services

## Quick Installation

### Option 1: Automated Installation Script (Recommended)

Run the installation script with sudo:

```bash
cd /home/kntrnjb/ai-dream-team/micro-ai-swarm/love-unlimited/tabby
sudo bash /tmp/install-tabby-service.sh
```

This will:
1. Validate the Tabby binary exists
2. Create the systemd service file
3. Reload systemd configuration
4. Enable Tabby to start on boot
5. Start Tabby immediately
6. Verify the service is running

### Option 2: Manual Installation

#### Step 1: Create the Service File

Create `/etc/systemd/system/tabby.service` with sudo:

```bash
sudo tee /etc/systemd/system/tabby.service > /dev/null << 'EOF'
[Unit]
Description=Tabby Code Completion Service with Love-Unlimited Integration
After=network.target love-unlimited-hub.service
Wants=love-unlimited-hub.service

[Service]
Type=simple
User=kntrnjb
Group=kntrnjb
WorkingDirectory=/home/kntrnjb/ai-dream-team/micro-ai-swarm/love-unlimited/tabby

# Love-Unlimited Hub Integration
Environment="LOVE_UNLIMITED_ENABLED=true"
Environment="LOVE_UNLIMITED_URL=http://localhost:9003"
Environment="LOVE_UNLIMITED_KEY=lu_tabby_VCnVIGgHq4Lx2VCQPz_y_ggYPrGTRHxW"
Environment="LOVE_UNLIMITED_TIMEOUT=5"
Environment="LOVE_UNLIMITED_MAX_RETRIES=3"
Environment="LOVE_UNLIMITED_LOG_COMPLETIONS=true"
Environment="LOVE_UNLIMITED_LOG_USER_EVENTS=true"
Environment="LOVE_UNLIMITED_TRACK_ERRORS=true"

# Tabby Configuration
Environment="TABBY_WEBSERVER_JWT_TOKEN_SECRET="
Environment="RUST_LOG=info"

# Service execution
ExecStart=/home/kntrnjb/ai-dream-team/micro-ai-swarm/love-unlimited/tabby/target/release/tabby serve --port 8080 --host 0.0.0.0

# Auto-restart behavior
Restart=always
RestartSec=5

# Resource limits
LimitNOFILE=65535
LimitNPROC=65535

# Timeout for graceful shutdown
TimeoutStopSec=30

# Logging
StandardOutput=journal
StandardError=journal
SyslogIdentifier=tabby

[Install]
WantedBy=multi-user.target
EOF
```

#### Step 2: Reload Systemd

```bash
sudo systemctl daemon-reload
```

#### Step 3: Enable the Service

Enable Tabby to start automatically on boot:

```bash
sudo systemctl enable tabby.service
```

#### Step 4: Start the Service

```bash
sudo systemctl start tabby.service
```

#### Step 5: Verify Installation

Check that the service is running:

```bash
sudo systemctl status tabby.service
```

## Service Management

### Check Service Status

```bash
systemctl status tabby.service
```

Expected output:
```
● tabby.service - Tabby Code Completion Service with Love-Unlimited Integration
     Loaded: loaded (/etc/systemd/system/tabby.service; enabled; preset: enabled)
     Active: active (running) since Sat 2026-01-11 05:10:00 CST; 2min ago
   Main PID: 1234567 (tabby)
      Tasks: 42 (limit: 4618)
     Memory: 245.3M
```

### View Service Logs

View recent logs:
```bash
journalctl -u tabby.service -n 50 --no-pager
```

Follow logs in real-time:
```bash
journalctl -u tabby.service -f
```

Filter by priority level:
```bash
journalctl -u tabby.service -p info  # info, warning, err, crit, etc.
```

### Start/Stop/Restart

Start the service:
```bash
sudo systemctl start tabby.service
```

Stop the service:
```bash
sudo systemctl stop tabby.service
```

Restart the service:
```bash
sudo systemctl restart tabby.service
```

Reload configuration without restarting:
```bash
sudo systemctl reload tabby.service
```

### Enable/Disable Auto-Start

Enable auto-start on boot:
```bash
sudo systemctl enable tabby.service
```

Disable auto-start on boot (service still runs until stopped):
```bash
sudo systemctl disable tabby.service
```

Check if enabled:
```bash
systemctl is-enabled tabby.service
```

## Configuration

### Environment Variables

The service uses these environment variables configured in the systemd file:

**Love-Unlimited Integration:**
- `LOVE_UNLIMITED_ENABLED=true` - Enable hub integration
- `LOVE_UNLIMITED_URL=http://localhost:9003` - Hub location
- `LOVE_UNLIMITED_KEY=...` - API authentication key
- `LOVE_UNLIMITED_TIMEOUT=5` - Request timeout (seconds)
- `LOVE_UNLIMITED_MAX_RETRIES=3` - Failed request retries
- `LOVE_UNLIMITED_LOG_COMPLETIONS=true` - Store completions
- `LOVE_UNLIMITED_LOG_USER_EVENTS=true` - Store user actions
- `LOVE_UNLIMITED_TRACK_ERRORS=true` - Store failures

**Tabby Configuration:**
- `RUST_LOG=info` - Logging level (debug, info, warn, error)
- `TABBY_WEBSERVER_JWT_TOKEN_SECRET=` - Leave empty for auto-generation

To modify environment variables, edit the service file:

```bash
sudo systemctl edit tabby.service
```

Then restart:
```bash
sudo systemctl restart tabby.service
```

### Service Parameters

Edit the service file to change:
- `--port 8080` - API port
- `--host 0.0.0.0` - Bind address

Example: Change port to 9000:
```bash
sudo systemctl edit tabby.service
# Find ExecStart line and change --port 8080 to --port 9000
sudo systemctl restart tabby.service
```

## Troubleshooting

### Service won't start

Check the error logs:
```bash
journalctl -u tabby.service -n 50 --no-pager
```

Common issues:
- **Binary not found**: Ensure Tabby is built at `target/release/tabby`
- **Port already in use**: Change port in service file or kill process using port 8080
- **Love-Unlimited Hub not running**: Start the hub service first

### Service crashes repeatedly

Check logs:
```bash
journalctl -u tabby.service -f
```

Verify Love-Unlimited Hub is running:
```bash
curl http://localhost:9003/health
```

Verify API key is valid:
```bash
curl -H "X-API-Key: lu_tabby_VCnVIGgHq4Lx2VCQPz_y_ggYPrGTRHxW" \
  http://localhost:9003/self
```

### Check resource usage

Monitor memory and CPU:
```bash
systemctl status tabby.service
# Or
ps aux | grep tabby
```

View detailed resource information:
```bash
journalctl -u tabby.service --output=verbose | grep Memory
```

### Service disabled on reboot

Check if enabled:
```bash
systemctl is-enabled tabby.service
```

Enable auto-start:
```bash
sudo systemctl enable tabby.service
```

## Systemd Service Architecture

```
┌─────────────────────────────────────┐
│    Linux Kernel (systemd)           │
├─────────────────────────────────────┤
│ tabby.service (Type=simple)         │
├─────────────────────────────────────┤
│  After: network.target              │
│  Wants: love-unlimited-hub.service  │
├─────────────────────────────────────┤
│  ✓ Auto-restart on failure          │
│  ✓ Logging to journalctl            │
│  ✓ Resource limits configured       │
│  ✓ 5s restart delay                 │
│  ✓ 30s shutdown timeout             │
└─────────────────────────────────────┘
         ↓
    Tabby Server
  Port: 8080
  ↓
  Code Completions
  Event Logging
  ↓
  Love-Unlimited Hub
  Port: 9003
  ↓
  Memory Storage
  Shared Memories
```

## Integration with Love-Unlimited

When Tabby runs as a systemd service:

1. **Service starts** → Tabby boots with Love-Unlimited integration enabled
2. **Hub connection** → HubClient connects to http://localhost:9003
3. **Events trigger** → Completion events automatically logged
4. **Memories stored** → Async background tasks send memories to hub
5. **Service restarts** → Automatic restart on failure (5s delay)
6. **Logs tracked** → All activity logged to journalctl

## Verification

Verify all components are working:

```bash
# 1. Check Tabby service
systemctl status tabby.service

# 2. Check Tabby API
curl http://localhost:8080/v1/health | jq '.'

# 3. Check Love-Unlimited Hub
curl http://localhost:9003/health | jq '.'

# 4. Verify integration enabled
journalctl -u tabby.service | grep "Love-Unlimited"

# 5. Check Tabby's memories in hub
curl -H "X-API-Key: lu_tabby_VCnVIGgHq4Lx2VCQPz_y_ggYPrGTRHxW" \
  "http://localhost:9003/recall?q=completion&limit=5" | jq '.memories[]'
```

## Security Considerations

**API Key Management:**
- API key stored in systemd service file (restricted permissions)
- Consider using systemd environment files for sensitive data
- Rotate keys periodically

**Network Security:**
- Tabby listens on 0.0.0.0:8080 (all interfaces)
- Restrict network access with firewall if needed
- Love-Unlimited Hub only listens on localhost by default

**File Permissions:**
- Service file: `/etc/systemd/system/tabby.service` (644 root:root)
- Logs accessible via journalctl for all users
- Tabby data directory inherited from parent process

## Performance Tuning

### Memory Usage
Current: ~245-300 MB (includes llama.cpp server, tantivy index, embeddings)

To monitor:
```bash
watch -n 1 'systemctl status tabby.service | grep Memory'
```

### Connection Limits
Set via resource limits in service file:
- `LimitNOFILE=65535` - Max open files
- `LimitNPROC=65535` - Max processes

### Timeout Configuration
- `TimeoutStopSec=30` - Grace period for shutdown
- `RestartSec=5` - Delay before restart on failure

## Monitoring Integration

Systemd service is compatible with:
- **journalctl** - System journal viewing
- **systemd-analyze** - Performance analysis
- **monit/supervisor** - External monitoring (optional)
- **Prometheus** - Metrics export (future)

Example monitoring command:
```bash
systemd-analyze verify tabby.service
```

## Next Steps

After deploying Tabby with systemd:

1. **Test completions**: Use CLI or IDE extension
2. **Monitor memories**: Query Love-Unlimited Hub for stored memories
3. **Configure logging**: Adjust RUST_LOG level as needed
4. **Set up monitoring**: Integrate with your monitoring stack
5. **Implement backup**: Back up Love-Unlimited data/chromadb directory

## Related Services

Tabby works alongside:
- **love-unlimited-hub.service** - Memory storage system
- **ollama.service** - Local LLM provider (if using local embeddings)

Manage all services:
```bash
systemctl status love-unlimited-hub.service
systemctl status ollama.service
systemctl status tabby.service
```

---

**Philosophy:** Love unlimited. Until next time. 💙

Tabby + Love-Unlimited = Code completion with memory.
