mod case_combinations;
mod leet_combinations;
mod character_combinations;
mod sanitize;
mod output;

use clap::{Parser, Subcommand, Args};
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
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

    /// sanitize the wordlist
    #[arg(short = 's', long)]
    sanitize: bool,

    /// do leet transformations
    #[arg(short, long)]
    leet: bool,

    #[command(subcommand)]
    command: Option<Commands>,

    /// do case transformations
    #[arg(short, long)]
    case: bool,

    /// character set to use for app/pre/ins
    #[arg(short = 'C', long, default_value = "1234567890!@#$%^&*()-_=+[]{} ")]
    chars: String,
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

fn main() {
    let args = Cli::parse(); // get clap args

    let path = Path::new(&args.file);
    let file = File::open(&path).expect("Failed to open file"); // open file
    let reader = io::BufReader::new(file); // read file

    // read lines from file
    let word_list: HashSet<String> = reader
        .lines()
        .par_bridge() // convert to par iterator
        .filter_map(|line| line.ok()) // filter out errors
        .collect();

    let sanitized_word_list: HashSet<String> = if args.sanitize {
        word_list
            .into_par_iter()
            .flat_map(|word| sanitize::sanitize_word(&word))
            .collect()
    } else {
        word_list
    };

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
    let final_variations: HashSet<String> = match args.command {
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

    // print all results variants
    output::output_results(final_variations, args.out_file).expect("Error with output file.");

}
