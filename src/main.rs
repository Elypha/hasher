use bytesize::ByteSize;
use clap::Parser;
use regex::Regex;
use std::env;
use std::path::PathBuf;
use utils::helper::ToStringNormalised;

mod data {
    pub mod enums;
}
mod utils {
    pub mod helper;
}
mod tasks {
    pub mod hash;
    pub mod user_action;
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The action to perform. Valid values are: size, xxh3, check
    action: String,
    /// The path to the directory to process. If not provided, the current working directory is used.
    path: Option<PathBuf>,

    /// A list of regex patterns to exclude files from processing. Can be specified multiple times and evaluated to true if any pattern matches.
    #[arg(short, long)]
    exclude: Option<Vec<PathBuf>>,
    // #[arg(short, long)]
    // save: Option<PathBuf>,
}

fn main() {
    // parse CLI arguments
    // --------------------------------
    let cli = Cli::parse();

    let user_action = match cli.action.parse::<data::enums::UserAction>() {
        Ok(action) => action,
        Err(e) => {
            eprintln!(
                "Error: CLI args {} is not a valid UserAction: {e}",
                cli.action
            );
            std::process::exit(1);
        }
    };

    let root_path = match cli.path {
        Some(path) => path,
        None => {
            // use current directory if no path is provided
            match env::current_dir() {
                Ok(path) => path,
                Err(e) => {
                    eprintln!("Failed to get current directory: {error}", error = e);
                    std::process::exit(1);
                }
            }
        }
    };

    // get local files
    // --------------------------------
    let files_all = utils::helper::get_all_local_files(&root_path);
    let files_filtered = match cli.exclude {
        Some(excludes) => {
            let regexes: Vec<Regex> = excludes
                .iter()
                .filter_map(|x| Regex::new(&x.to_string_lossy()).ok())
                .collect();
            files_all
                .into_iter()
                .filter(|x| {
                    !regexes
                        .iter()
                        .any(|re| re.is_match(&x.to_string_normalised()))
                })
                .collect()
        }
        None => files_all,
    };

    // process files based on user action
    // --------------------------------
    match user_action {
        data::enums::UserAction::Size => {
            let total_files = files_filtered.len();
            let results =
                tasks::user_action::get_hash_results(&user_action, &root_path, files_filtered);
            // print results
            let relative_path_width = results
                .iter()
                .map(|x| x.relative_path.len())
                .max()
                .unwrap_or(0);
            for result in &results {
                println!(
                    "{}: {:relative_path_width$}  - {:>6}",
                    result.action,
                    result.relative_path,
                    ByteSize::b(result.value).display().iec_short().to_string(),
                );
            }
            // save results to file
            let (save_path, file_hash) =
                utils::helper::save_hash_results(&user_action, &root_path, &results);
            // print results
            println!(
                "{} files processed.\n'{}' checksum: {file_hash:X}",
                total_files,
                save_path.file_name().unwrap_or_default().to_string_normalised()
            );
        }
        data::enums::UserAction::XXH3 => {
            let total_files = files_filtered.len();
            let results =
                tasks::user_action::get_hash_results(&user_action, &root_path, files_filtered);
            // save results to file
            let (save_path, file_hash) =
                utils::helper::save_hash_results(&user_action, &root_path, &results);
            // print results
            println!(
                "{} files processed.\n'{}' checksum: {file_hash:X}",
                total_files,
                save_path.file_name().unwrap_or_default().to_string_normalised()
            );
        }
        data::enums::UserAction::Check => {
            let hasher_file = utils::helper::get_hasher_file(&root_path);
            // let hasher_file2 = hasher_file.clone();
            let content = std::fs::read_to_string(&hasher_file).unwrap_or_else(|e| {
                eprintln!("Failed to read .hasher file: {error}", error = e);
                std::process::exit(1);
            });
            let file_hash = tasks::hash::xxh3_bytes(content.as_bytes());
            // parse the .hasher file
            let results = utils::helper::hash_results_from_string(&content);
            // validate files
            let mut invalid_count: u32 = 0;
            for result in results {
                let file_path = root_path.join(&result.relative_path);
                if !file_path.exists() {
                    eprintln!("{path}: not found", path = result.relative_path);
                    continue;
                }
                match result.action {
                    data::enums::UserAction::Size => {
                        let result_local = tasks::hash::size(&file_path, &root_path).unwrap();
                        if result.value != result_local.value {
                            eprintln!(
                                "{path}: expected {}, found {}",
                                ByteSize::b(result.value).display().iec_short(),
                                ByteSize::b(result_local.value).display().iec_short(),
                                path = result.relative_path
                            );
                            invalid_count += 1;
                        }
                    }
                    data::enums::UserAction::XXH3 => {
                        let result_local = tasks::hash::xxh3(&file_path, &root_path).unwrap();
                        if result.value != result_local.value {
                            eprintln!(
                                "{path}: expected {:X}, found {:X}",
                                result.value,
                                result_local.value,
                                path = result.relative_path
                            );
                            invalid_count += 1;
                        }
                    }
                    _ => {
                        eprintln!(
                            "Unsupported action in .hasher file: {action}",
                            action = result.action
                        );
                        std::process::exit(1);
                    }
                }
            }

            eprintln!(
                "{count} invalid files.\n'{}' checksum: {file_hash:X}",
                &hasher_file
                    .file_name()
                    .unwrap_or_default()
                    .to_string_normalised(),
                count = invalid_count
            );
            if invalid_count > 0 {
                std::process::exit(1);
            }
        }
    }
}
