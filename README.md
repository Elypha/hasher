# Hasher

A command-line tool designed to verify the integrity of files within a directory by calculating and comparing their cryptographic hashes (checksums).

## Purpose

This is particularly useful for ensuring data integrity after unreliable transfer processes. It generates a record of file hashes, which can be used for check at a later time.

It uses `xxh3_64` by default, with a wide range of options for popular hash algorithms.

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
# calculate hashes for all files in the current directory (this repo), excluding .log files and .git and target directories
hasher xxh3 -e '.+\.log$' -e /\.git/ -e "(/|^)target/"
# > 10 files processed.
# > 'XXH3.hasher' checksum: 4A5A9A2997E68875

tail xxh3.hasher
# > ...
# > XXH3:28D2B8A3B6515B0E,src/tasks/user_action.rs
# > XXH3:2D06800538D394C2,src/test.rs
# > XXH3:B58677E3B5190C33,src/utils/helper.rs

hasher check
# > README.md: expected F596FDAEA81A39CC, found 3016CA6821596A7F
# > 1 invalid files.
# > 'xxh3.hasher' checksum: 4A5A9A2997E68875
```
