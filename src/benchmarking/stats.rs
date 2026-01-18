use hdrhistogram::Histogram;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct LatencyStats {
    add_order: Histogram<u64>,
    cancel_order: Histogram<u64>,
    match_order: Histogram<u64>,
}

#[allow(dead_code)]
pub struct LatencyReport {
    p50: u64,
    p95: u64,
    p99: u64,
}

#[allow(dead_code)]
pub struct StatsSummary {
    add_order: LatencyReport,
    cancel_order: LatencyReport,
    match_order: LatencyReport,
}

impl LatencyStats {
    pub fn new() -> LatencyStats {
        LatencyStats {
            add_order: Histogram::new(3).unwrap(),
            cancel_order: Histogram::new(3).unwrap(),
            match_order: Histogram::new(3).unwrap(),
        }
    }

    pub fn record_add(&mut self, value: Duration) {
        self.add_order.record(value.as_nanos() as u64).ok();
    }
    pub fn record_cancel(&mut self, value: Duration) {
        self.cancel_order.record(value.as_nanos() as u64).ok();
    }
    pub fn record_match(&mut self, value: Duration) {
        self.match_order.record(value.as_nanos() as u64).ok();
    }

    pub fn get_stats(&self) -> StatsSummary {
        let add_order = LatencyReport {
            p50: self.add_order.value_at_percentile(50.0),
            p95: self.add_order.value_at_percentile(95.0),
            p99: self.add_order.value_at_percentile(99.0),
        };
        let cancel_order = LatencyReport {
            p50: self.cancel_order.value_at_percentile(50.0),
            p95: self.cancel_order.value_at_percentile(95.0),
            p99: self.cancel_order.value_at_percentile(99.0),
        };
        let match_order = LatencyReport {
            p50: self.match_order.value_at_percentile(50.0),
            p95: self.match_order.value_at_percentile(95.0),
            p99: self.match_order.value_at_percentile(99.0),
        };
        StatsSummary {
            add_order,
            cancel_order,
            match_order,
        }
    }
}

impl Default for LatencyStats {
    fn default() -> Self {
        Self::new()
    }
}
