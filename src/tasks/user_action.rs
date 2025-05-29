use crate::data;
use crate::tasks;
use rayon::prelude::*;
use std::io;
use std::path::PathBuf;

pub fn get_hash_results(
    action: &data::enums::UserAction,
    cwd: &PathBuf,
    files: Vec<PathBuf>,
) -> Vec<tasks::hash::HashResult> {
    let hash_processor = map_hash_processor(action);
    let results: Vec<_> = files
        .par_iter()
        .map(|x| hash_processor(x, &cwd))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap_or_else(|e| {
            eprintln!("Error processing files: {error}", error = e);
            std::process::exit(1);
        });
    return results;
}

fn map_hash_processor(
    action: &data::enums::UserAction,
) -> fn(&PathBuf, &PathBuf) -> io::Result<tasks::hash::HashResult> {
    match action {
        data::enums::UserAction::Size => tasks::hash::size,
        data::enums::UserAction::XXH3 => tasks::hash::xxh3,
        _ => panic!("Unsupported action: {}", action),
    }
}
