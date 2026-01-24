## Matchstick Performance Report

### System Configuration

| | M1 MacBook | AMD Ryzen 5 |
|---|---|---|
| CPU | | |
| Cores/Threads | | |
| RAM | | |
| OS | | |
| Rust Version | | |
| Build Profile | | |

### Microbenchmarks (Criterion)

| Operation | M1 Mean | M1 p99 | Ryzen Mean | Ryzen p99 |
|---|---|---|---|---|
| `new()` | | | | |
| `add_order()` | | | | |
| `cancel_order()` | | | | |
| `modify_order()` | | | | |
| `match_orders()` | | | | |
| `get_levels()` | | | | |

### Live Latency (Kraken L3 Feed)

| Metric | M1 | Ryzen |
|---|---|---|
| Add p50 | | |
| Add p95 | | |
| Add p99 | | |
| Add p99.9 | | |
| Add max | | |
| Modify p50 | | |
| Modify p99 | | |
| Delete p50 | | |
| Delete p99 | | |
| Jitter (stddev) | | |

### Throughput

| Metric | M1 | Ryzen |
|---|---|---|
| Messages/sec (sustained) | | |
| Messages/sec (peak) | | |
| Orders/sec (add) | | |
| Orders/sec (modify) | | |
| Orders/sec (delete) | | |

### System Stats (perf stat)

| Metric | M1 | Ryzen |
|---|---|---|
| L1 cache miss rate | | |
| L2 cache miss rate | | |
| L3 cache miss rate | | |
| Branch misprediction rate | | |
| Instructions per cycle | | |
| Context switches | | |

### Flamegraph Hotspots (Top 5)

| Rank | M1 Function | M1 % | Ryzen Function | Ryzen % |
|---|---|---|---|---|
| 1 | | | | |
| 2 | | | | |
| 3 | | | | |
| 4 | | | | |
| 5 | | | | |

### Memory

| Metric | M1 | Ryzen |
|---|---|---|
| Peak RSS | | |
| Allocations (hot path) | | |
| Allocation rate | | |
