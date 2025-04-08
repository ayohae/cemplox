mod case_combinations;
mod leet_combinations;
mod character_combinations;
mod sanitize;

use clap::{Parser, Subcommand, Args};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{self, BufRead, BufReader, BufWriter, Write, stdout};
use rayon::prelude::*;
use std::collections::HashSet;

/// this program generates in-depth wordlists
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// path to a file containing words to transform
    #[arg(short, long, required = true)]
    file: String,

    /// path to the output file
    #[arg(short, long, required = false)]
    out_file: Option<String>,

    /// how many words from the wordlist are processed at one time.
    /// lower if RAM consumption is too high. increase to increase processing time.
    /// you can safely raise this if you aren't doing many operations/transformations at the same time.
    #[arg(short = 'b', long, default_value_t = 5)]
    batch_size: usize,

    /// sanitize the wordlist
    #[arg(short = 's', long)]
    sanitize: bool,

    /// do leet transformations
    #[arg(short, long)]
    leet: bool,

    /// do case transformations
    #[arg(short, long)]
    case: bool,

    /// character set to use for app/pre/ins
    #[arg(short = 'C', long, default_value = "1234567890!@#$%^&*()-_=+[]{} ")]
    chars: String,

    /// choose between adding characters based on count or desired length of output
    #[command(subcommand)]
    command: Option<Commands>,


}
#[derive(Args, Debug)]
struct LengthArgs {
    /// minimum length of final words
    #[arg(short, long, default_value_t = 1)]
    min: u8,

    /// maximum length of final words
    #[arg(short = 'M', long, default_value_t = 16)]
    max: u8,

    /// append characters option
    #[arg(short, long)]
    append: bool,

    /// prepend characters option
    #[arg(short, long)]
    prepend: bool,

    /// insert characters option
    #[arg(short, long)]
    insert: bool,
}

#[derive(Args, Debug)]
struct CountArgs {
    /// append characters option
    #[arg(short, long, default_value_t = 0)]
    append: u8,

    /// prepend characters option
    #[arg(short, long, default_value_t = 0)]
    prepend: u8,

    /// insert characters option
    #[arg(short, long, default_value_t = 0)]
    insert: u8,
}

#[derive(Subcommand,Debug)]
enum Commands {
    Length(LengthArgs),
    Count(CountArgs),
}

fn process_transformations(sanitized_word_list:HashSet<String>, args: &Cli) -> HashSet<String> {
    // apply case transformations (if enabled)
    let case_transformed_words: HashSet<String> = if args.case {
        sanitized_word_list
            .into_par_iter()
            .flat_map(|word| case_combinations::case_combinations(&word))
            .collect()
    } else {
        sanitized_word_list
    };

    // apply leet transformations (if enabled)
    let leet_transformed_words: HashSet<String> = if args.leet {
        case_transformed_words
            .into_par_iter()
            .flat_map(|word| leet_combinations::leet_combinations(&word))
            .collect()
    } else {
        case_transformed_words
    };

    // apply character addition transformations
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

        None => leet_transformed_words

    };

    final_variations
}

fn process_and_write_batch(batch: &[String], args: &Cli, output: &mut Box<dyn Write>) -> io::Result<()> {
    let sanitized_batch: Vec<String> = if args.sanitize {
        batch.par_iter().flat_map(|word| sanitize::sanitize_word(word)).collect()
    } else {
        batch.to_vec()
    };

    let final_variations: HashSet<String> = process_transformations(sanitized_batch.into_iter().collect(), args);

    for variant in final_variations {
        writeln!(output, "{}", variant)?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args = Cli::parse(); // get clap args

    let path = Path::new(&args.file);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

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
            process_and_write_batch(&batch, &args, &mut output)?;
            batch.clear();
        }
    }

    if !batch.is_empty() {
        process_and_write_batch(&batch, &args, &mut output)?;
    }

    Ok(())
}


