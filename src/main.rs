use anyhow::Result;
use clap::{Parser, Subcommand, Args};
use env_logger;
use log::{info};
use num_cpus;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write, stdout};
use std::path::Path;
use std::time::Instant;
use tempfile::{NamedTempFile, TempPath};

// Import our transformation modules.
mod case_combinations;
mod leet_combinations;
mod character_combinations;
mod sanitize;


/// this program generates large wordlists from provided input wordlist
/// by applying various transformations and additions
#[derive(Parser, Debug)]
#[command(version, about = "Wordlist Generator", long_about = None)]
struct Cli {
    /// path to a input wordlist file
    #[arg(short, long, required = true)]
    file: String,

    /// path to the output file. if not provided, output is written stdout.
    #[arg(short, long)]
    out_file: Option<String>,

    /// number of input words processed at one time. (smaller batches lower RAM usage)
    #[arg(short = 'b', long, default_value_t = 5)]
    batch_size: usize,

    /// sanitize the wordlist (trim and remove special chars), leaving a copy of the original
    #[arg(short = 's', long)]
    sanitize: bool,

    /// apply leetspeak transforms
    #[arg(short, long)]
    leet: bool,

    /// apply case transforms (lower to upper, upper to lower)
    #[arg(short, long)]
    case: bool,

    /// character set used for additional character transforms
    #[arg(short = 'C', long, default_value = "1234567890!@#$%^&*()-_=+[]{} ")]
    chars: String,

    /// choose between character addition by count or final word length
    #[command(subcommand)]
    command: Option<Commands>,

    /// enable tempfiles to reduce RAM usage
    #[arg(short, long)]
    tempfile_mode: bool,

    /// max number of threads for parallel processing, # of cpu cores by default
    #[arg(long, default_value_t = num_cpus::get())]
    max_threads: usize,

    /// dryrun mode. estimate counts without running.
    #[arg(long)]
    dry_run: bool,

    /// log level (error, warn, info, debug, trace)
    #[arg(short = 'L', long, default_value = "info")]
    log_level: String,
}

#[derive(Args, Debug)]
struct LengthArgs {
    /// min length of output words
    #[arg(short, long, default_value_t = 1)]
    min: u8,

    /// max length of output words.
    #[arg(short = 'M', long, default_value_t = 16)]
    max: u8,

    /// append mode
    #[arg(short, long)]
    append: bool,

    /// prepend mode
    #[arg(short, long)]
    prepend: bool,

    /// insert mode (very expensive, use with caution)
    #[arg(short, long)]
    insert: bool,
}

#[derive(Args, Debug)]
struct CountArgs {
    /// append this number of characters
    #[arg(short, long, default_value_t = 0)]
    append: u8,

    /// prepend this number of characters
    #[arg(short, long, default_value_t = 0)]
    prepend: u8,

    /// insert this number of characters (very expensive, use with caution)
    #[arg(short, long, default_value_t = 0)]
    insert: u8,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Length(LengthArgs),
    Count(CountArgs),
}

/// transform pipeline
fn process_transformations(sanitized_word_list: HashSet<String>, args: &Cli) -> HashSet<String> {
    // Apply case transformations (if enabled)
    let case_transformed_words: HashSet<String> = if args.case {
        sanitized_word_list
            .into_par_iter()
            .flat_map(|word| case_combinations::case_combinations(&word))
            .collect()
    } else {
        sanitized_word_list
    };

    // Apply leet transformations (if enabled)
    let leet_transformed_words: HashSet<String> = if args.leet {
        case_transformed_words
            .into_par_iter()
            .flat_map(|word| leet_combinations::leet_combinations(&word))
            .collect()
    } else {
        case_transformed_words
    };

    // Apply character addition transformations
    let final_variations: HashSet<String> = match &args.command {
        Some(Commands::Length(length_args)) => {
            leet_transformed_words
                .par_iter()
                .flat_map(|variation| {
                    character_combinations::length_character_combinations(
                        variation,
                        &args.chars,
                        length_args.min.into(),
                        length_args.max.into(),
                        length_args.append,
                        length_args.prepend,
                        length_args.insert,
                    )
                })
                .collect()
        }
        Some(Commands::Count(count_args)) => {
            leet_transformed_words
                .par_iter()
                .flat_map(|variation| {
                    character_combinations::count_character_combinations(
                        variation,
                        &args.chars,
                        count_args.append.into(),
                        count_args.prepend.into(),
                        count_args.insert.into(),
                    )
                })
                .collect()
        }
        None => leet_transformed_words,
    };

    final_variations
}

/// process the batch
fn process_batch(batch: &[String], args: &Cli) -> HashSet<String> {
    info!("processing a batch of {} words", batch.len());
    let sanitized_batch: Vec<String> = if args.sanitize {
        batch
            .par_iter()
            .flat_map(|word| sanitize::sanitize_word(word))
            .collect()
    } else {
        batch.to_vec()
    };

    let final_variations = process_transformations(sanitized_batch.into_iter().collect(), args);
    info!("batch produced {} variations", final_variations.len());
    final_variations
}

/// merge results from multiple tempfiles into the final output file
fn merge_tempfiles(temp_paths: Vec<tempfile::TempPath>, args: &Cli) -> io::Result<()> {
    info!("merging tempfiles...");
    let mut merged_set = HashSet::new();

    for temp_path in temp_paths {
        // Open each temporary file from its path.
        let file = File::open(&temp_path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            merged_set.insert(line);
        }
    }

    info!("merged total of {} unique variations", merged_set.len());
    if args.dry_run {
        info!("dryrun enabled. skipping output.");
        return Ok(());
    }

    // write the merged variations
    let mut output: Box<dyn Write> = match &args.out_file {
        Some(path) => {
            let file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(path)?;
            Box::new(BufWriter::new(file))
        }
        None => Box::new(stdout().lock()),
    };

    for variant in merged_set {
        writeln!(output, "{}", variant)?;
    }

    Ok(())
}


fn main() -> Result<()> {
    let args = Cli::parse();

    // Set logging level and initialize logger.
    std::env::set_var("RUST_LOG", &args.log_level);
    env_logger::init();

    // Configure the Rayon thread pool based on the CLI setting.
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.max_threads)
        .build_global()
        .unwrap();
    info!("starting wordlist generation with {} threads", args.max_threads);

    let start_time = Instant::now();

    let path = Path::new(&args.file);
    let file = File::open(&path)
        .map_err(|e| anyhow::anyhow!("failed to open input file {}: {}", args.file, e))?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Depending on the tempfile_mode flag, either process in memory or via temporary files.
    if args.tempfile_mode {
        info!("running in tempfile mode to reduce memory usage.");
        let mut temp_paths: Vec<TempPath> = Vec::new();
        let mut temp_files = Vec::new();
        let batch_size = args.batch_size;
        let mut batch = Vec::with_capacity(batch_size);

        while let Some(line) = lines.next() {
            batch.push(line?);
            if batch.len() == batch_size {
                let variations = process_batch(&batch, &args);
                // Write the batch results to a temporary file.
                let mut temp_file = NamedTempFile::new()?;
                {
                    let mut writer = BufWriter::new(&mut temp_file);
                    for variant in variations {
                        writeln!(writer, "{}", variant)?;
                    }
                }
                let temp_path = temp_file.into_temp_path();
                temp_files.push(temp_path);
                batch.clear();
            }
        }
        if !batch.is_empty() {
            let variations = process_batch(&batch, &args);
            let mut temp_file = NamedTempFile::new()?;
            {
                let mut writer = BufWriter::new(&mut temp_file);
                for variant in variations {
                    writeln!(writer, "{}", variant)?;
                }
            }
            let temp_path = temp_file.into_temp_path();
            temp_paths.push(temp_path);
        }
        merge_tempfiles(temp_paths, &args)?;
    } else {
        info!("running in entirely in‑memory mode.");
        let mut output: Box<dyn Write> = match args.out_file.clone() {
            Some(path) => {
                let file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(path)?;
                Box::new(BufWriter::new(file))
            }
            None => Box::new(stdout().lock()),
        };

        let batch_size = args.batch_size;
        let mut batch = Vec::with_capacity(batch_size);

        while let Some(line) = lines.next() {
            batch.push(line?);
            if batch.len() == batch_size {
                let variations = process_batch(&batch, &args);
                if args.dry_run {
                    info!("dryrun: would output {} variations.", variations.len());
                } else {
                    for variant in variations {
                        writeln!(output, "{}", variant)?;
                    }
                }
                batch.clear();
            }
        }
        if !batch.is_empty() {
            let variations = process_batch(&batch, &args);
            if args.dry_run {
                info!("dryrun: should output {} variations.", variations.len());
            } else {
                for variant in variations {
                    writeln!(output, "{}", variant)?;
                }
            }
        }
    }

    let duration = start_time.elapsed();
    info!("processing completed in {:.2?}.", duration);
    Ok(())
}






