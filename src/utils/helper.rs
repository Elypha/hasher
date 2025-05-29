use crate::data;
use crate::tasks;
use glob::glob;
use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

pub trait ToStringNormalised {
    fn to_string_normalised(&self) -> String;
}

impl ToStringNormalised for PathBuf {
    fn to_string_normalised(&self) -> String {
        // always uses `/` instead of `\`
        self.to_string_lossy().replace("\\", "/")
    }
}
impl ToStringNormalised for Path {
    fn to_string_normalised(&self) -> String {
        // always uses `/` instead of `\`
        self.to_string_lossy().replace("\\", "/")
    }
}
impl ToStringNormalised for std::ffi::os_str::OsStr {
    fn to_string_normalised(&self) -> String {
        // always uses `/` instead of `\`
        self.to_string_lossy().replace("\\", "/")
    }
}

pub fn get_all_local_files(path: &PathBuf) -> Vec<PathBuf> {
    match glob(&path.join("**/*").to_string_lossy()) {
        Ok(paths) => {
            return paths
                .filter_map(|x| match x {
                    Ok(path) => {
                        if path.is_file() && !path.to_string_lossy().ends_with(".hasher") {
                            Some(path)
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
        }
        Err(e) => {
            eprintln!("Failed to read files: {e}");
            std::process::exit(1);
        }
    };
}

pub fn get_hasher_file(root_path: &PathBuf) -> PathBuf {
    let hasher_files = match glob(&root_path.join("*.hasher").to_string_lossy()) {
        Ok(paths) => paths
            .filter_map(|path| match path {
                Ok(path) if path.is_file() => Some(path),
                _ => None,
            })
            .collect::<Vec<_>>(),
        Err(e) => {
            eprintln!("Failed to read files: {e}");
            std::process::exit(1);
        }
    };
    // validate that there is exactly one .hasher file
    if hasher_files.len() != 1 {
        eprintln!(
            "Expected exactly one .hasher file, found {}: {:?}",
            hasher_files.len(),
            hasher_files
        );
        std::process::exit(1);
    }
    return hasher_files[0].clone();
}

pub fn save_hash_results(
    action: &data::enums::UserAction,
    root_path: &PathBuf,
    results: &[crate::tasks::hash::HashResult],
) -> (PathBuf, u64) {
    // create output file to save results
    let save_path = root_path.join(format!("{}.hasher", action));
    let mut file = File::create(&save_path).unwrap_or_else(|e| {
        eprintln!("Failed to create output file: {e}");
        std::process::exit(1);
    });
    // write to memory first
    let content = results
        .iter()
        .map(|x| format!("{}:{:X},{}", x.action, x.value, x.relative_path))
        .collect::<Vec<_>>()
        .join("\n");
    // then write to file
    file.write_all(content.as_bytes()).unwrap_or_else(|e| {
        eprintln!("Failed to write to output file: {e}");
        std::process::exit(1);
    });
    // and return the file hash
    let file_hash = tasks::hash::xxh3_bytes(content.as_bytes());
    return (save_path, file_hash);
}

pub fn hash_results_from_string(content: &str) -> Vec<tasks::hash::HashResult> {
    let hasher_pattern =
        Regex::new(r"^(?P<action>\w+):(?P<value>[0-9A-Fa-f]+),(?P<relative_path>.+)$")
            .unwrap_or_else(|e| {
                eprintln!("Failed to compile regex: {error}", error = e);
                std::process::exit(1);
            });

    let mut hasher_results = Vec::new();
    for line in content.lines() {
        if let Some(caps) = hasher_pattern.captures(line) {
            // action
            let action_str = caps.name("action").unwrap().as_str();
            let action = data::enums::UserAction::from_str(action_str).unwrap_or_else(|e| {
                eprintln!("Failed to parse action: {error}", error = e);
                std::process::exit(1);
            });
            // relative path
            let relative_path = caps.name("relative_path").unwrap().as_str().to_string();
            // value
            let value_str = caps.name("value").unwrap().as_str();
            let value = u64::from_str_radix(value_str, 16).unwrap_or_else(|e| {
                eprintln!("Failed to parse value: {error}", error = e);
                std::process::exit(1);
            });
            // create hasher result
            hasher_results.push(tasks::hash::HashResult {
                action,
                relative_path,
                value,
            });
        } else {
            eprintln!("Invalid line in .hasher file: {line}");
        }
    }
    return hasher_results;
}
