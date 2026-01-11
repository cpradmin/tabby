#!/bin/bash
# Installation script for Tabby systemd service with Love-Unlimited integration

set -e

echo "Installing Tabby systemd service..."
echo ""

# Check if running as root
if [[ $EUID -ne 0 ]]; then
   echo "This script must be run as root or with sudo"
   echo "Usage: sudo bash install-tabby-service.sh"
   exit 1
fi

# Define paths
TABBY_DIR="/home/kntrnjb/ai-dream-team/micro-ai-swarm/love-unlimited/tabby"
SERVICE_FILE="/etc/systemd/system/tabby.service"

# Check if tabby binary exists
if [ ! -f "$TABBY_DIR/target/release/tabby" ]; then
    echo "✗ Error: Tabby binary not found at $TABBY_DIR/target/release/tabby"
    echo "  Please run: cd $TABBY_DIR && cargo build --release"
    exit 1
fi

echo "✓ Tabby binary found"

# Create the systemd service file
cat > "$SERVICE_FILE" << 'EOF'
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

echo "✓ Service file created at $SERVICE_FILE"

# Reload systemd
systemctl daemon-reload
echo "✓ Systemd daemon reloaded"

# Enable the service
systemctl enable tabby.service
echo "✓ Tabby service enabled (will start on boot)"

# Start the service
systemctl start tabby.service
echo "✓ Tabby service started"

# Check status
sleep 2
if systemctl is-active --quiet tabby.service; then
    echo ""
    echo "✓✓✓ SUCCESS ✓✓✓"
    echo "Tabby is running with Love-Unlimited integration"
    echo ""
    systemctl status tabby.service --no-pager
else
    echo ""
    echo "⚠ Service started but failed immediately"
    echo "Check logs with: journalctl -u tabby.service -n 50 --no-pager"
    exit 1
fi

echo ""
echo "Management commands:"
echo "  systemctl status tabby.service    - Check service status"
echo "  systemctl restart tabby.service   - Restart Tabby"
echo "  systemctl stop tabby.service      - Stop Tabby"
echo "  systemctl start tabby.service     - Start Tabby"
echo "  journalctl -u tabby.service -f    - Follow logs in real-time"
echo ""
echo "Tabby is now running on http://localhost:8080"
