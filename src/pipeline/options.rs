use crate::cli::{Cli, Commands};

#[derive(Clone)]
pub struct PipelineOptions {
    pub sanitize: bool,
    pub case: bool,
    pub leet: bool,
    pub chars: String,
    pub command: PipelineCommand,
    pub case_max_changes: Option<usize>,
    pub leet_max_substitutions: Option<usize>,
}

impl PipelineOptions {
    pub fn from_cli(cli: &Cli) -> Self {
        let command = match &cli.command {
            Some(Commands::Length(opts)) => PipelineCommand::Length {
                min: opts.min,
                max: opts.max,
                append: opts.append,
                prepend: opts.prepend,
                insert: opts.insert,
                dedup: !opts.skip_dedup,
            },
            Some(Commands::Count(opts)) => PipelineCommand::Count {
                append: opts.append,
                prepend: opts.prepend,
                insert: opts.insert,
            },
            None => PipelineCommand::None,
        };
        Self {
            sanitize: cli.sanitize,
            case: cli.case,
            leet: cli.leet,
            chars: cli.chars.clone(),
            command,
            case_max_changes: cli.case_max_changes,
            leet_max_substitutions: cli.leet_max_substitutions,
        }
    }
}

#[derive(Clone)]
pub enum PipelineCommand {
    None,
    Length {
        min: usize,
        max: usize,
        append: bool,
        prepend: bool,
        insert: bool,
        dedup: bool,
    },
    Count {
        append: usize,
        prepend: usize,
        insert: usize,
    },
}
