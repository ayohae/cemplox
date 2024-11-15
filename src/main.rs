mod case_combinations;
mod leet_combinations;
mod character_combinations;
use clap::Parser;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use rayon::prelude::*;


/// this program generates in-depth wordlists
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path to a file containing words to transform
    #[arg(short, long, required = true)]
    file: String,

    /// minimum length of final words
    #[arg(short, long, default_value_t = 1)]
    min: u8,

    /// maximum length of final words
    #[arg(short = 'M', long, default_value_t = 8)]
    max: u8,

    /// do leet transformations
    #[arg(short, long)]
    leet: bool,

    /// do case transformations
    #[arg(short, long)]
    case: bool,

    /// append characters option
    #[arg(short, long)]
    append: bool,

    /// prepend characters option
    #[arg(short, long)]
    prepend: bool,

    /// insert characters option
    #[arg(short, long)]
    insert: bool,

    /// character set to use for app/pre/ins
    #[arg(short = 'C', long, default_value = "1234567890!@#$%^&*()-_=+[]{} ")]
    chars: String,
}

use std::collections::HashSet;

fn main() {
    let args = Args::parse(); // get clap args

    let path = Path::new(&args.file);
    let file = File::open(&path).expect("Failed to open file"); // open file
    let reader = io::BufReader::new(file); // read file

    // read lines from file
    let word_list: HashSet<String> = reader
        .lines()
        .par_bridge() // convert to par iterator
        .filter_map(|line| line.ok()) // filter out errors
        .collect();

    // apply case transformations (if enabled)
    let case_transformed_words: HashSet<String> = if args.case {
        word_list
            .into_par_iter()
            .flat_map(|word| case_combinations::case_combinations(&word))
            .collect()
    } else {
        word_list
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
    let final_variations: HashSet<String> = leet_transformed_words
        .par_iter()
        .flat_map(|variation| {
            character_combinations::character_combinations(
                variation,
                &args.chars,
                args.min.into(),
                args.max.into(),
                args.append,
                args.prepend,
                args.insert,
            )
        })
        .collect();

    // print all results variants
    for variant in final_variations {
        println!("{}", variant);
    }

}
