use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Default)]
pub struct Metrics {
    lines: AtomicU64,
    variants: AtomicU64,
    invalid: AtomicU64,
}

impl Metrics {
    pub fn record_line(&self) {
        self.lines.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_variant(&self) {
        self.variants.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_invalid(&self) {
        self.invalid.fetch_add(1, Ordering::Relaxed);
    }

    pub fn lines(&self) -> u64 {
        self.lines.load(Ordering::Relaxed)
    }

    pub fn variants(&self) -> u64 {
        self.variants.load(Ordering::Relaxed)
    }

    pub fn invalid(&self) -> u64 {
        self.invalid.load(Ordering::Relaxed)
    }
}
