# System Design Document: Unique IP Address Counter Service

## Overview

This document outlines the system design for a log ingestion service that counts unique IP addresses and exposes metrics to Prometheus. The design addresses scalability concerns and proposes both a minimal viable solution and a production-ready architecture.

## Problem Statement

The service must:
- Receive log entries via HTTP/1.1 POST at `http://your-service:5000/logs`
- Count unique IP addresses since service start
- Expose cumulative count as a Prometheus metric at `http://your-service:9102/metrics`
- Handle potentially thousands of requests per second

## Requirements

| Requirement | Specification |
|-------------|--------------|
| **Time limit** | 3 hours |
| **Ports** | :5000 (logs), :9102 (metrics) |
| **Log endpoint** | POST :5000/logs (JSON) |
| **Metrics endpoint** | GET :9102/metrics (Prometheus format) |
| **Metric name** | `unique_ip_addresses` |
| **Deliverable** | Public Git repository |

## Scalability Analysis

### Current Approach Limitations

The proposed HTTP POST per log entry approach faces significant challenges at scale:

#### Performance Bottlenecks

- **Network overhead**: 80% of traffic consumed by protocol overhead
- **Connection exhaustion**: Port and file descriptor depletion
- **CPU overhead**: Continuous TCP handshake operations
- **Latency accumulation**: Serial connection overhead degrades throughput

#### Throughput Analysis (10K req/sec)

```
Connection parameters:
- TCP handshake: 1-3 RTT per connection
- HTTP headers: 200-500 bytes per request
- Actual payload: ~100 bytes

Efficiency calculation:
overhead_per_request = 400 bytes (headers + TCP)
payload = 100 bytes
efficiency = payload / (payload + overhead_per_request)
         = 100 / 500
         = 20%
```

**Result**: Only 20% efficiency at the application layer.

## Proposed Architecture

### Production-Ready Approach

To achieve 10K+ requests/second throughput, the following architectural improvements are recommended:

#### 1. Log Collection Layer

**Recommendation**: Implement intermediate log aggregation using industry-standard tools.

**Tool**: Filebeat
- **Documentation**: [Filebeat Overview](https://www.elastic.co/guide/en/beats/filebeat/8.19/filebeat-overview.html)
- **Purpose**: Efficient log shipping with built-in buffering and retry logic

#### 2. Message Buffer Layer

**Technology**: Redis (with TTL-based index management)

**Configuration**:
```yaml
output.redis:
  hosts: ["localhost:6379"]
  key: "filebeat-2025.10.21"
  db: 0
  timeout: 5
  ttl: 90000  # TTL in seconds (25 hours)
  datatype: hash
  
filebeat.inputs:
  - type: log
    enabled: true
    paths:
      - /var/log/*.log
```

**Benefits**:
- Automatic index expiration after 25 hours
- Decoupling of ingestion and processing
- Built-in persistence and reliability

#### 3. Processing Service

**Service**: `unique_ip_counter`

**Operation**:
- Polls Redis every 5 minutes
- Executes aggregation query for unique IP count
- Updates Prometheus metric

**Redis Query**:
```redis
FT.AGGREGATE filebeat-2025.10.21 "*" 
  GROUPBY 1 @ip 
  REDUCE COUNT 0 
  SORTBY 0 
  LIMIT 0 0
```

**Log Format**:
```json
{
  "timestamp": "2020-06-24T15:27:00.123456Z",
  "ip": "83.150.59.250",
  "url": "..."
}
```

**Note**: The `ip` field must be indexed in Redis for efficient querying.

#### 4. Metrics Exposure

- Expose `unique_ip_addresses` metric in Prometheus format
- Endpoint: `http://your-service:9102/metrics`
- Update frequency: Every 5 minutes (aligned with Redis polling)

## Architectural Improvements

### Recommended Enhancements

| Enhancement | Benefit |
|-------------|---------|
| **Batching** | Reduces per-request overhead |
| **Connection pooling / HTTP/2** | Reuses connections, eliminates handshake overhead |
| **Message Queue** (Kafka/Pulsar/RabbitMQ/Redis) | Ensures reliability and horizontal scalability |
| **Local buffering** | Smooths traffic spikes |

### Expected Performance

With the proposed architecture:
- **Throughput**: 100K+ logs/second per instance
- **Efficiency**: >90% (reduced protocol overhead)
- **Scalability**: Horizontal scaling via queue partitioning
- **Reliability**: Built-in retry logic and persistence

## Data Flow

```
[Application] 
    ↓ (writes logs)
[Filebeat] 
    ↓ (batched writes)
[Redis with TTL index] 
    ↓ (periodic polling every 5min)
[unique_ip_counter service] 
    ↓ (aggregation query)
[Prometheus metric exposure]
    ↓ (scraping)
[Prometheus Server]
```

## Implementation Considerations

### Minimal Viable Solution (3-hour constraint)

For the time-constrained assignment, implement:
1. Simple HTTP POST handler on :5000
2. In-memory HashSet for IP tracking
3. Basic Prometheus metric exporter on :9102

### Production Evolution Path

1. **Phase 1**: Add Redis caching layer
2. **Phase 2**: Integrate Filebeat for log shipping
3. **Phase 3**: Implement message queue for high-availability
4. **Phase 4**: Deploy horizontal scaling with load balancing

## Conclusion

While the assignment constraints suggest a simple HTTP POST implementation, production-grade systems require careful consideration of:
- Protocol efficiency
- Connection management
- Buffer and queue strategies
- Horizontal scalability

The proposed Redis-based architecture with Filebeat integration provides a clear path from MVP to production-ready system capable of handling 100K+ requests/second while maintaining reliability and observability.

**Next Step**: Discuss architecture with DevOps team over coffee ☕ to align on production requirements and deployment strategy.

