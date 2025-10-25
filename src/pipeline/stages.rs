use std::sync::Arc;
use super::metrics::Metrics;
use super::options::{PipelineCommand, PipelineOptions};
use super::worker::WorkerState;
use crate::character_combinations::{stream_count, stream_length};
use crate::{case_combinations, leet_combinations, sanitize};

pub fn process_chunk(
    chunk: &[u8],
    options: &Arc<PipelineOptions>,
    metrics: &Arc<Metrics>,
    state: &mut WorkerState,
    capacity: usize,
) {
    let line_bytes = if matches!(chunk.last(), Some(b'\r')) {
        &chunk[..chunk.len().saturating_sub(1)]
    } else {
        chunk
    };

    match std::str::from_utf8(line_bytes) {
        Ok(line) => {
            metrics.record_line();
            for_each_sanitized(line, options, |sanitized| {
                for_each_case(sanitized, options, |case_variant| {
                    for_each_leet(case_variant, options, |leet_variant| {
                        for_each_chars(leet_variant, options, |final_variant| {
                            state.push_line(&final_variant, capacity);
                            metrics.record_variant();
                        });
                    });
                });
            });
        }
        Err(_) => metrics.record_invalid(),
    }
}
fn for_each_sanitized<F>(line: &str, options: &Arc<PipelineOptions>, mut visit: F)
where
    F: FnMut(String),
{
    if options.sanitize {
        for candidate in sanitize::stream(line) {
            visit(candidate);
        }
    } else {
        visit(line.to_owned());
    }
}
fn for_each_case<F>(word: String, options: &Arc<PipelineOptions>, mut visit: F)
where
    F: FnMut(String),
{
    if options.case {
        for variant in case_combinations::stream_cases(&word, options.case_max_changes) {
            visit(variant);
        }
    } else {
        visit(word);
    }
}
fn for_each_leet<F>(word: String, options: &Arc<PipelineOptions>, mut visit: F)
where
    F: FnMut(String),
{
    if options.leet {
        for variant in leet_combinations::stream_leet(&word, options.leet_max_substitutions) {
            visit(variant);
        }
    } else {
        visit(word);
    }
}
fn for_each_chars<F>(word: String, options: &Arc<PipelineOptions>, mut visit: F)
where
    F: FnMut(String),
{
    match &options.command {
        PipelineCommand::None => visit(word),
        PipelineCommand::Length {
            min,
            max,
            append,
            prepend,
            insert,
            dedup,
        } => {
            for variant in stream_length(
                &word,
                &options.chars,
                *min,
                *max,
                *append,
                *prepend,
                *insert,
                *dedup,
            ) {
                visit(variant);
            }
        }
        PipelineCommand::Count {
            append,
            prepend,
            insert,
        } => {
            for variant in stream_count(
                &word,
                &options.chars,
                *append,
                *prepend,
                *insert,
            ) {
                visit(variant);
            }
        }
    }
}
