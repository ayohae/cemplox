use anyhow::Result;
use clap::{Parser, Subcommand, Args};
use env_logger;
use log::info;
use num_cpus;
use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::sync::{Arc, Mutex};

mod sanitize;
mod case_combinations;
mod leet_combinations;
mod character_combinations;

/// generate larger wordlists based on transformations of wordlists
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long)] file: String,
    #[arg(short, long)] out_file: Option<String>,
    #[arg(short, long)] sanitize: bool,
    #[arg(short, long)] case: bool,
    #[arg(short, long)] leet: bool,
    #[arg(short = 'C', long, default_value = "1234567890!@#$%^&*()-=_+[]{} ")] chars: String,
    #[command(subcommand)] command: Option<Commands>,
    #[arg(long, default_value_t = num_cpus::get())] max_threads: usize,
    #[arg(short = 'L', long, default_value = "info")] log_level: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Length(LengthArgs),
    Count(CountArgs),
}

#[derive(Args, Debug)]
struct LengthArgs {
    #[arg(short, long, default_value_t = 2)] min: usize,
    #[arg(short = 'M', long, default_value_t = 16)] max: usize,
    #[arg(short, long)] append: bool,
    #[arg(short, long)] prepend: bool,
    #[arg(short, long)] insert: bool,
}

#[derive(Args, Debug)]
struct CountArgs {
    #[arg(short, long, default_value_t = 0)] append: usize,
    #[arg(short, long, default_value_t = 0)] prepend: usize,
    #[arg(short, long, default_value_t = 0)] insert: usize,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    std::env::set_var("RUST_LOG", &args.log_level);
    env_logger::init();
    ThreadPoolBuilder::new().num_threads(args.max_threads).build_global()?;
    info!("parallel processing with {} threads", args.max_threads);

    // init reader and writer
    let reader = BufReader::new(File::open(&args.file)?);
    let writer: Box<dyn Write + Send> = if let Some(path) = &args.out_file {
        Box::new(BufWriter::new(File::create(path)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };
    let shared_writer = Arc::new(Mutex::new(writer));

    // par pipeline
    reader
        .lines()
        .filter_map(Result::ok)
        .par_bridge()
        .flat_map_iter(move |line| {
            let iter: Box<dyn Iterator<Item=String> + Send> = if args.sanitize {
                Box::new(sanitize::stream(&line))
            } else {
                Box::new(std::iter::once(line))
            };
            iter
        })
        .flat_map_iter(move |w| {
            let iter: Box<dyn Iterator<Item=String> + Send> = if args.case {
                Box::new(case_combinations::stream_cases(&w))
            } else {
                Box::new(std::iter::once(w))
            };
            iter
        })
        .flat_map_iter(move |w| {
            let iter: Box<dyn Iterator<Item=String> + Send> = if args.leet {
                Box::new(leet_combinations::stream_leet(&w))
            } else {
                Box::new(std::iter::once(w))
            };
            iter
        })
        .flat_map_iter(move |w| {
            let iter: Box<dyn Iterator<Item=String> + Send> = match &args.command {
                Some(Commands::Length(opt)) => Box::new(character_combinations::stream_length(
                    &w, &args.chars,
                    opt.min, opt.max,
                    opt.append, opt.prepend, opt.insert,
                )),
                Some(Commands::Count(opt)) => Box::new(character_combinations::stream_count(
                    &w, &args.chars,
                    opt.append, opt.prepend, opt.insert,
                )),
                None => Box::new(std::iter::once(w)),
            };
            iter
        })
        .for_each_with(shared_writer, |writer, variant| {
            let mut w = writer.lock().unwrap();
            writeln!(w, "{}", variant).unwrap();
        });

    Ok(())
}
