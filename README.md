# 📈 matchstick

Rust-based Kraken client and benchmarking harness designed to interact with [matchbook](https://github.com/PaddyConnolly/matchbook.git) for testing and performance evaluation.

## Planned Features

* Connect to Kraken API via WebSocket
* Authenticate private endpoints with API keys
* Fetch L3 market data
* Built-in benchmarking harness for Matchbook
* Async support using Tokio runtime

## Configuration

Set your Kraken API credentials as environment variables:

```bash
export KRAKEN_API_KEY="your_api_key"
export KRAKEN_API_SECRET="your_api_secret"
```

## Usage

Run the full benchmark suite:

```bash
./scripts/bench.sh
```

The script will:

- Detect OS and verify privileges
- Run pre-flight checks (Rust, project dependencies, AC power)
- Collect detailed system information
- Configure the system for optimal benchmarking (macOS/Linux)
- Build the release binary
- Execute benchmarks (`warmup`, `criterion`, `flamegraph`, `perf`, live latency)
- Generate a summary report
