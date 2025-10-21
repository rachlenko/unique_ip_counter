#!/bin/bash

# Test script for the IP Counter Service

echo "Testing IP Counter Service..."

# Function to send log entry
send_log() {
  local ip=$1
  local timestamp=${2:-$(date -u +"%Y-%m-%dT%H:%M:%S.%6NZ")}

  echo "Sending log for IP: $ip"
  curl -X POST http://localhost:5000/logs \
    -H "Content-Type: application/json" \
    -d "{\"timestamp\": \"$timestamp\", \"ip\": \"$ip\", \"url\": \"/test\"}" \
    -s | jq .
}

# Test with various IPs
echo -e "\n=== Testing unique IPs ==="
send_log "192.168.1.1"
send_log "10.0.0.1"
send_log "172.16.0.1"
send_log "8.8.8.8"

echo -e "\n=== Testing duplicate IP ==="
send_log "192.168.1.1"

echo -e "\n=== Testing IPv6 ==="
send_log "2001:0db8:85a3:0000:0000:8a2e:0370:7334"
send_log "::1"

echo -e "\n=== Testing invalid IP ==="
send_log "not.an.ip"

echo -e "\n=== Checking stats ==="
curl -s http://localhost:5000/stats | jq .

echo -e "\n=== Checking Prometheus metrics ==="
curl -s http://localhost:9102/metrics | grep unique_ip_addresses

echo -e "\n=== Load test: sending 100 requests ==="
for i in {1..100}; do
  send_log "192.168.1.$((i % 256))" &
done
wait

echo -e "\n=== Final stats ==="
curl -s http://localhost:5000/stats | jq .
curl -s http://localhost:9102/metrics | grep unique_ip_addresses
