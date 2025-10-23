mod metrics;
mod options;
mod stages;
mod worker;

use crate::cli::Cli;
use anyhow::{Context, Result};
use crossbeam_channel::bounded;
use env_logger;
use log::info;
use memmap2::Mmap;
use rayon::prelude::*;
use rayon::slice::ParallelSlice;
use rayon::ThreadPoolBuilder;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use metrics::Metrics;
use options::PipelineOptions;
use stages::process_chunk;
use worker::WorkerState;

const BUFFER_CAPACITY: usize = 64 * 1024;
const CHANNEL_MULTIPLIER: usize = 4;

pub fn run(cli: Cli) -> Result<()> {
    std::env::set_var("RUST_LOG", &cli.log_level);
    env_logger::init();
    ThreadPoolBuilder::new()
        .num_threads(cli.max_threads)
        .build_global()?;
    info!("parallel processing with {} threads", cli.max_threads);

    let options = Arc::new(PipelineOptions::from_cli(&cli));
    let metrics = Arc::new(Metrics::default());

    let input_file = File::open(&cli.file)
        .with_context(|| format!("failed to open input file {}", cli.file))?;
    let mmap = unsafe { Mmap::map(&input_file)? };

    let writer = create_writer(cli.out_file.as_deref())
        .with_context(|| "failed to create output destination".to_string())?;

    let channel_depth = cli.max_threads.max(1) * CHANNEL_MULTIPLIER;
    let (sender, receiver) = bounded(channel_depth.max(2));
    let writer_thread = spawn_writer(writer, receiver);

    let start = Instant::now();
    let options_ref = Arc::clone(&options);
    let metrics_ref = Arc::clone(&metrics);
    let sender_clone = sender.clone();

    mmap.par_split(|b| *b == b'\n')
        .for_each_init(
            move || WorkerState::new(sender_clone.clone(), BUFFER_CAPACITY),
            move |state, chunk| {
                process_chunk(
                    chunk,
                    &options_ref,
                    &metrics_ref,
                    state,
                    BUFFER_CAPACITY,
                );
            },
        );

    drop(sender);
    writer_thread.join().expect("writer thread panicked")?;

    let elapsed = start.elapsed();
    info!(
        "processed {} lines ({} invalid) -> {} variants in {:.2?}",
        metrics.lines(),
        metrics.invalid(),
        metrics.variants(),
        elapsed
    );
    Ok(())
}

fn create_writer(path: Option<&str>) -> Result<Box<dyn Write + Send>> {
    if let Some(path) = path {
        let file = File::create(path)?;
        Ok(Box::new(BufWriter::new(file)))
    } else {
        Ok(Box::new(BufWriter::new(io::stdout())))
    }
}

fn spawn_writer(
    mut writer: Box<dyn Write + Send>,
    receiver: crossbeam_channel::Receiver<Vec<u8>>,
) -> thread::JoinHandle<Result<()>> {
    thread::spawn(move || {
        for buffer in receiver {
            writer.write_all(&buffer)?;
        }
        writer.flush()?;
        Ok(())
    })
}
