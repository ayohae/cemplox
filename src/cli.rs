use clap::{Args, Parser, Subcommand};
use num_cpus;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(short, long)]
    pub file: String,
    #[arg(short, long)]
    pub out_file: Option<String>,
    #[arg(short, long)]
    pub sanitize: bool,
    #[arg(short, long)]
    pub case: bool,
    #[arg(short, long)]
    pub leet: bool,
    #[arg(short = 'C', long, default_value = "1234567890!@#$%^&*()-=_+[]{} ")]
    pub chars: String,
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[arg(long, default_value_t = num_cpus::get())]
    pub max_threads: usize,
    #[arg(short = 'L', long, default_value = "info")]
    pub log_level: String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Length(LengthArgs),
    Count(CountArgs),
}

#[derive(Args, Debug)]
pub struct LengthArgs {
    #[arg(short, long, default_value_t = 2)]
    pub min: usize,
    #[arg(short = 'M', long, default_value_t = 16)]
    pub max: usize,
    #[arg(short, long)]
    pub append: bool,
    #[arg(short, long)]
    pub prepend: bool,
    #[arg(short, long)]
    pub insert: bool,
    #[arg(long, default_value_t = false)]
    pub skip_dedup: bool,
}

#[derive(Args, Debug)]
pub struct CountArgs {
    #[arg(short, long, default_value_t = 0)]
    pub append: usize,
    #[arg(short, long, default_value_t = 0)]
    pub prepend: usize,
    #[arg(short, long, default_value_t = 0)]
    pub insert: usize,
}
