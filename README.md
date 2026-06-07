# fgrep

> A fast, tiny command-line tool that searches your file tree and prints every **path** that matches a pattern — with the match highlighted.

`fgrep` recursively walks a directory and prints each file or folder path whose name contains the text you're looking for, highlighting the matched portion in green so it's easy to spot.

```
$ fgrep handler
./src/handler.rs
            ^^^^^^^  (shown in green)
```

---

## Features

- 🔎 **Recursive search** — walks the entire directory tree from your starting point.
- 🎨 **Highlighted matches** — the matching substring is printed in bold green.
- 🔠 **Case-insensitive mode** — match regardless of letter casing with `-i`.
- ⚡ **Fast & lightweight** — written in Rust, single small binary, no runtime dependencies.
- 💻 **Cross-platform** — works on macOS, Linux, and Windows.

---

## Installation

### Prerequisites

- [Rust & Cargo](https://www.rust-lang.org/tools/install) (edition 2024)
- Python 3 (only if you use the install script)

### Quick install (recommended)

The included script builds `fgrep` in release mode, copies it to `~/.local/bin`, and adds that directory to your `PATH`:

```bash
python3 install.py
```

Then open a new terminal (or `source` your shell config) and you're ready to go.

### Manual build

```bash
cargo build --release
# the binary is at: target/release/fgrep
```

---

## Usage

```
fgrep <PATTERN> [PATH] [OPTIONS]
```

| Argument / Option      | Description                                   | Default |
|------------------------|-----------------------------------------------|---------|
| `<PATTERN>`            | Text to search for in file paths (required)   | —       |
| `[PATH]`               | Directory to search                           | `.`     |
| `-i`, `--ignore-case`  | Case-insensitive matching                     | off     |
| `-h`, `--help`         | Print help                                    | —       |

### Examples

Search the current directory for paths containing `config`:

```bash
fgrep config
```

Search a specific directory:

```bash
fgrep handler ./src
```

Ignore case (matches `README`, `readme`, `ReadMe`, …):

```bash
fgrep -i readme
```

---

## How it works

`fgrep` uses [`walkdir`](https://crates.io/crates/walkdir) to traverse the directory tree, checks each path for the pattern, and uses [`colored`](https://crates.io/crates/colored) to highlight matches. Command-line parsing is handled by [`clap`](https://crates.io/crates/clap).

```
src/
├── main.rs      # entry point — parses args, kicks off the search
├── cli.rs       # CLI definition (clap)
├── handler.rs   # walks the tree, matches & highlights paths
└── lib.rs       # module wiring
```

---

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
