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

fn main() {
    let args = Args::parse(); // get clap args

    let path = Path::new(&args.file);
    let file = File::open(&path).expect("Failed to open file"); // open file
    let reader = io::BufReader::new(file); // read file

    // read lines from the file
    let word_variations: Vec<String> = reader
        .lines()
        .par_bridge()  // converts the iterator to a parallel bridge iterator
        .filter_map(|line| line.ok())  // filter out errors
        .collect();

    // apply transformations
    let transformed_variations: Vec<String> = word_variations
        .into_par_iter()
        .flat_map(|word| {
            let mut variants = vec![word.clone()];

            if args.case {
                variants.extend(case_combinations::case_combinations(&word));
            }
            if args.leet {
                variants.extend(leet_combinations::leet_combinations(&word));
            }

            variants
        })
        .collect();

    // character addition transformations
    transformed_variations
        .par_iter()
        .for_each(|variation| {
            let final_variations = character_combinations::generate_char_variants(
                variation,
                &args.chars,
                args.min.into(),
                args.max.into(),
                args.append,
                args.prepend,
                args.insert,
            );

            // print to stdout (for piping or redirection to a file)
            for variant in final_variations {
                println!("{}", variant);
            }
        });
}