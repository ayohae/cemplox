use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use log::error;
use sysinfo::{get_current_pid, System};

const POLL_INTERVAL: Duration = Duration::from_millis(500);

pub struct WatchdogGuard {
    running: Arc<AtomicBool>,
    handle: JoinHandle<()>,
}

pub fn spawn(limit_mb: u64) -> WatchdogGuard {
    let running = Arc::new(AtomicBool::new(true));
    let thread_flag = Arc::clone(&running);
    let handle = thread::spawn(move || {
        let mut system = System::new();
        let pid = get_current_pid().expect("failed to obtain current pid");
        while thread_flag.load(Ordering::Relaxed) {
            system.refresh_process(pid);
            if let Some(process) = system.process(pid) {
                let rss_mb = process.memory() / 1024;
                if rss_mb >= limit_mb {
                    error!(
                        "memory watchdog exiting: rss {} MB exceeded limit {} MB",
                        rss_mb, limit_mb
                    );
                    std::process::exit(2);
                }
            }
            thread::sleep(POLL_INTERVAL);
        }
    });
    WatchdogGuard { running, handle }
}

impl WatchdogGuard {
    pub fn stop(self) {
        self.running.store(false, Ordering::Relaxed);
        let _ = self.handle.join();
    }
}
