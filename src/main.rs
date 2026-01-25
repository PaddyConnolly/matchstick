use battery::{Manager, State};
use chrono::prelude::*;
use matchbook::orderbook::Orderbook;
use matchstick::adapter::ParseError;
use matchstick::kraken::client::KrakenClient;
use matchstick::messages::{Data, Response};
use std::fs::{self, create_dir_all};
use std::process::{Command, Stdio, exit};
use sysinfo::System;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt};

fn init_logging() {
    fmt()
        .with_env_filter(EnvFilter::new("INFO")) // only info and above
        .without_time()
        .with_target(false) // omit module paths
        .with_level(true) // show INFO/WARN/ERROR
        .init();
}

pub fn process_message(orderbook: &mut Orderbook, message: Response) -> Result<(), ParseError> {
    if message.channel != "level3" {
        return Err(ParseError::InvalidChannel);
    }
    if message.message_type != "update" {
        return Err(ParseError::InvalidType);
    }
    if message.data.is_empty() {
        return Err(ParseError::Empty);
    }

    for data in &message.data {
        // Process bids
        for event in &data.bids {
            process_event(orderbook, event, true)?;
        }
        // Process asks
        for event in &data.asks {
            process_event(orderbook, event, false)?;
        }
    }

    Ok(())
}

fn process_event(
    orderbook: &mut Orderbook,
    event: &OrderEvent,
    is_bid: bool,
) -> Result<(), ParseError> {
    match event.event {
        EventType::Add => {
            let order = to_order(event, is_bid);
            orderbook.add(order)?;
        }
        EventType::Modify => {
            let id = to_order_id(&event.order_id);
            let qty = to_quantity(event.order_qty);
            orderbook.modify_order(id, qty)?;
        }
        EventType::Delete => {
            let id = to_order_id(&event.order_id);
            orderbook.cancel_order(id)?;
        }
    }
    Ok(())
}
// Directory to save benchmark reports
const REPORT_DIR: &str = "reports";

// Check if running on AC power
fn on_ac_power() -> bool {
    let manager = Manager::new().ok();
    if let Some(mgr) = manager {
        for bat in mgr.batteries().into_iter().flatten() {
            if bat.unwrap().state() == State::Charging {
                return true;
            }
        }
        false
    } else {
        true // assume AC if no battery info
    }
}

// Run a command and exit on failure
fn run_or_exit(cmd: &mut Command, error_msg: &str) {
    if !cmd.status().expect(error_msg).success() {
        error!("{}", error_msg);
        exit(1);
    }
}

// Pre-flight checks
fn preflight_checks() {
    info!("Running pre-flight checks...");

    // Project directory
    let project_dir = std::env::current_dir().expect("Failed to get cwd");
    if !project_dir.join("Cargo.toml").exists() {
        error!("Cargo.toml not found in {}", project_dir.display());
        exit(1);
    }

    // Matchbook dependency
    let cargo_toml =
        fs::read_to_string(project_dir.join("Cargo.toml")).expect("Failed to read Cargo.toml");
    if !cargo_toml.contains("matchbook") {
        error!("matchbook dependency not found in Cargo.toml");
        exit(1);
    }

    // AC power
    if !on_ac_power() {
        error!("Device is not connected to AC power");
        exit(1);
    } else {
        info!("Device is connected to AC power");
    }
}

// Collect system info
fn collect_system_info() {
    info!("Collecting system information...");

    let mut sys = System::new_all();
    sys.refresh_all();

    info!("Date: {}", Utc::now());
    info!("OS: {}", System::name().unwrap_or_else(|| "Unknown".into()));
    info!(
        "Kernel: {}",
        System::kernel_version().unwrap_or_else(|| "Unknown".into())
    );
    info!("Architecture: {}", std::env::consts::ARCH);
    info!(
        "CPU: {} cores",
        System::physical_core_count()
            .map(|c| c.to_string())
            .unwrap_or_else(|| "Unknown".into())
    );
    info!(
        "Memory: {} GB",
        sys.total_memory() as f64 / 1024_f64.powi(3)
    );

    // Rust versions
    let rustc_version = Command::new("rustc")
        .arg("--version")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "rustc not installed".into());
    let cargo_version = Command::new("cargo")
        .arg("--version")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "cargo not installed".into());

    info!("rustc: {}", rustc_version);
    info!("cargo: {}", cargo_version);

    // Git commits
    let project_dir = std::env::current_dir().unwrap();
    let matchstick_commit = Command::new("git")
        .args([
            "-C",
            project_dir.to_str().unwrap(),
            "rev-parse",
            "--short",
            "HEAD",
        ])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".into());
    info!("matchstick: {}", matchstick_commit);

    let matchbook_path = project_dir.join("../matchbook");
    if matchbook_path.exists() {
        let matchbook_commit = Command::new("git")
            .args([
                "-C",
                matchbook_path.to_str().unwrap(),
                "rev-parse",
                "--short",
                "HEAD",
            ])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "unknown".into());
        info!("matchbook: {}", matchbook_commit);
    }

    info!("System info collection complete.");
}

// Build release
fn build_release() {
    info!("Updating dependencies...");
    run_or_exit(
        Command::new("cargo").arg("update").arg("--quiet"),
        "Failed to update Cargo.lock",
    );

    info!("Building release binary...");
    run_or_exit(
        Command::new("cargo")
            .arg("build")
            .arg("--release")
            .arg("--quiet"),
        "Build failed",
    );
}

/// Warmup phase: run a quick build & bench to prime caches
fn warmup() {
    info!("Running warmup...");
    // Run cargo bench with --no-run to build benchmarks
    run_or_exit(
        Command::new("cargo")
            .arg("bench")
            .arg("--no-run")
            .arg("--quiet"),
        "Warmup build failed",
    );
}

/// Criterion benchmarks
fn run_criterion() {
    info!("Running Criterion benchmarks...");
    run_or_exit(
        Command::new("cargo")
            .arg("bench")
            .arg("--quiet")
            .stdout(Stdio::null())
            .stderr(Stdio::null()),
        "Criterion benchmarks failed",
    );
}

/// Flamegraph generation
#[allow(dead_code)]
fn run_flamegraph() {
    info!("Generating flamegraph...");
    // Assumes cargo-flamegraph is installed
    run_or_exit(
        Command::new("cargo").arg("flamegraph"),
        "Flamegraph generation failed",
    );
}

/// perf stat measurements (Linux only)
fn run_perf_stat() {
    info!("Running perf stat...");
    #[cfg(target_os = "linux")]
    run_or_exit(
        Command::new("perf")
            .arg("stat")
            .arg("-r")
            .arg("5")
            .arg("./target/release/matchstick"),
        "perf stat failed",
    );

    #[cfg(not(target_os = "linux"))]
    info!("perf stat not supported on this OS, skipping");
}

/// Live latency measurements (custom)
async fn run_live_latency() -> Result<LatencyStats, Box<dyn std::error::Error>> {
    info!("Running live latency benchmarks...");
    dotenvy::dotenv().ok();

    let mut client = KrakenClient::new().await?;
    let mut orderbook = Orderbook::new();
    let mut stats = LatencyStats::new();

    let duration = std::time::Duration::from_secs(60);
    let start = std::time::Instant::now();

    while start.elapsed() < duration {
        let msg = client.read().await?;

        if let Ok(update) = serde_json::from_str::<Update>(&msg) {
            let op_start = std::time::Instant::now();
            let result = process_update(&mut orderbook, update);
            let elapsed = op_start.elapsed();

            match result {
                Ok(()) => stats.record_add(elapsed), // TODO: distinguish add/modify/delete
                Err(_) => {}
            }
        }
    }

    info!("Processed messages for {:?}", duration);
    Ok(stats)
}

// Generate summary
fn generate_summary() {
    create_dir_all(REPORT_DIR).expect("Failed to create report dir");
    fs::write(
        format!("{}/SUMMARY.md", REPORT_DIR),
        "# Benchmark Summary\n\nResults go here",
    )
    .expect("Failed to write summary");
    info!("Results saved to: {}", REPORT_DIR);
    info!("View summary: cat {}/SUMMARY.md", REPORT_DIR);
}

// Run benchmark suite
#[tokio::main]
async fn main() {
    init_logging();
    preflight_checks();
    collect_system_info();
    build_release();

    warmup();
    run_criterion();
    //run_flamegraph();
    run_perf_stat();
    run_live_latency().await;

    generate_summary();

    info!("{}", "Benchmark Complete");
}
