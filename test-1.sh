#!/bin/bash

send_log() {
  local ip=$1
  local timestamp=${2:-$(date -u +"%Y-%m-%dT%H:%M:%S.%6NZ")}

  echo "Sending log for IP: $ip"
  curl -X POST http://localhost:5000/logs \
    -H "Content-Type: application/json" \
    -d "[{\"ip\": \"$ip\"}]" \
    -s
}

# Test with various IPs
echo -e "\n=== Testing unique IPs ==="
send_log "192.168.1.1"
send_log "10.0.0.1"
send_log "172.16.0.1"
send_log "8.8.8.8"

curl -s http://localhost:9102/metrics
