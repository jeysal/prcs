# prcs

> The Swiss Army knife for running processes

[![Version](https://img.shields.io/crates/v/prcs?style=flat-square)](https://crates.io/crates/prcs)
[![License](https://img.shields.io/crates/l/prcs?style=flat-square)](LICENSE.txt)

`prcs` runs your command, but modifies its behavior in ways that you define with command-line arguments.  
For example, you could retry a command until it exits with status code `0`.  
Using the options that `prcs` implements requires less effort than shell scripting the behavior and is cross-platform.  

Note: The feature set of this project is not yet very comprehensive and is only documented in the `--help` output.

## Installation

With [Cargo](https://github.com/rust-lang/cargo):

```sh
cargo install prcs
```

Or [download binary releases](https://github.com/jeysal/prcs/releases) directly.

## Usage

Run `prcs -h` for an overview of how to use this application, or run `prcs --help` for detailed instructions.

## Features

| Feature                         | Implemented |
| ------------------------------- | ----------- |
| Stall                           | ✅          |
| Re-run                          | ✅          |
| Re-run max tries                |             |
| Re-run delay                    |             |
| Translate/swallow signals       |             |
| Logrotate                       |             |
| Empty closing/non-closing stdin |             |
| ...                             |             |
