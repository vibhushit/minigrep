# minigrep

`minigrep` is a command-line tool written in Rust that allows you to search for a word in a specified file. It supports both case-sensitive and case-insensitive searches.

## Features

1. Search for a word in a specified file.
2. Support for both case-sensitive and case-insensitive searches.

## Requirements

- Rust (stable version recommended)
- Cargo (Rust's build system and package manager)

## Installation

To use minigrep, clone the repository to your local machine:

```sh
git clone https://github.com/vibhushit/minigrep.git
cd minigrep
```

## Usage

### Basic usage
Run the program with two arguments: the query word and the file path:

```sh
cargo run -- <query> <file_path>
```

This will search for <query> in <file_path> in a case-sensitive manner.


### Case Insensitive Search
To perform a case-insensitive search, set the `IGNORE_CASE` environment variable to `1` before running the program:

```sh
IGNORE_CASE=1 cargo run -- <query> <file_path>
```

### Output Redirection
You can redirect the output to a file using standard shell redirection operators (`>` or `>>`):

```sh
cargo run -- <query> <file_path> > output.txt
```

