use crossbeam_channel::Sender;

pub struct WorkerState {
    buffer: Vec<u8>,
    sender: Sender<Vec<u8>>,
}

impl WorkerState {
    pub fn new(sender: Sender<Vec<u8>>, capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            sender,
        }
    }

    pub fn push_line(&mut self, line: &str, capacity: usize) {
        self.buffer.extend_from_slice(line.as_bytes());
        self.buffer.push(b'\n');
        if self.buffer.len() >= capacity {
            self.flush(capacity);
        }
    }

    fn flush(&mut self, capacity: usize) {
        if self.buffer.is_empty() {
            return;
        }
        let mut out = Vec::new();
        std::mem::swap(&mut out, &mut self.buffer);
        if self.sender.send(out).is_ok() {
            self.buffer = Vec::with_capacity(capacity);
        } else {
            self.buffer.clear();
        }
    }
}

impl Drop for WorkerState {
    fn drop(&mut self) {
        if self.buffer.is_empty() {
            return;
        }
        let mut out = Vec::new();
        std::mem::swap(&mut out, &mut self.buffer);
        let _ = self.sender.send(out);
    }
}
