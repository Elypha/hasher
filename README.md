# Hasher

A command-line tool that deals with cryptographic hashes (checksums) of loose files and aims at QoL aspects.

[![Rust](https://github.com/Elypha/hasher/actions/workflows/rust.yml/badge.svg)](https://github.com/Elypha/hasher/actions/workflows/rust.yml)
[![Release](https://github.com/Elypha/hasher/actions/workflows/release.yml/badge.svg)](https://github.com/Elypha/hasher/actions/workflows/release.yml)

## Purpose

In simple words,

- it uses a hash algorithm (of your choice) to calculate hashes of files (of your choice) in a directory, and save the results to a plaintext file `<algo>.hasher`;
- the file can be later used to verify the hashes again.

All of these in one simple commmand for QoL. I find it particularly useful for ensuring data integrity, when you know your data *have to* go through some *unreliable transfer processes*.

Further development depends on ~~how much PAIN I have to suffer for my work, and~~ how many people find this project useful so please kindly let me know that x

## Example Usage

Command line reference:

```
Usage: hasher.exe [OPTIONS] <ACTION> [PATH]

Arguments:
  <ACTION>  The action to perform. Valid values are: size, xxh3, check
  [PATH]    The path to the directory to process. If not provided, the current working directory is
            used

Options:
  -e, --exclude <EXCLUDE>  A list of regex patterns to exclude files from processing. Can be
                           specified multiple times and evaluated to true if any pattern matches
  -h, --help               Print help
  -V, --version            Print version
```

Example commands:

```bash
hasher xxh3 .
hasher -e '\.log$' xxh3 src
```

Example usage:

```bash
# calculate hashes for all files in CWD (this repo), excluding .log files, .git and target dir
hasher xxh3 -e '.+\.log$' -e \.git/ -e "target/"
# > 10 files processed.
# > 'xxh3.hasher' checksum: 4A5A9A2997E68875

tail xxh3.hasher
# > ...
# > XXH3:28D2B8A3B6515B0E,src/tasks/user_action.rs
# > XXH3:2D06800538D394C2,src/test.rs
# > XXH3:B58677E3B5190C33,src/utils/helper.rs

# check the integrity of the files in the current directory against .hasher file
hasher check
# > README.md: expected F596FDAEA81A39CC, found 3016CA6821596A7F
# > 1 invalid files.
# > 'xxh3.hasher' checksum: 4A5A9A2997E68875
```
