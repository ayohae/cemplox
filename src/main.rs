mod case_combinations;
mod leet_combinations;
mod character_combinations;

use clap::Parser;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};


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
    let args = Args::parse(); // get args from Clap

    let path = Path::new(&args.file);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    let mut word_variations: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(valid_line) => word_variations.push(valid_line.clone()),
            Err(_) => {},
        }
    }


    // start the vector of transformations

    if args.case { // if case transformations is true, then apply
        word_variations = word_variations.into_iter().flat_map(|w| case_combinations::case_combinations(&w)).collect();
    }

    if args.leet { // if leet transformations is true, then apply
        word_variations = word_variations.into_iter().flat_map(|w| leet_combinations::leet_combinations(&w)).collect();
    }

    // iterate through the leet and case variations, generate char variants for each
    for variation in word_variations {
        let final_variations = character_combinations::generate_char_variants(
            &variation,
            &args.chars,
            args.min.into(),
            args.max.into(),
            args.append,
            args.prepend,
            args.insert,
        );

        // print out each variation, keeping chunks smaller
        for variation in final_variations {
            println!("{}", variation);
        }
    }
}
