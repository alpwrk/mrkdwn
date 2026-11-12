# mrkdwn

A small, fast command-line tool that renders Markdown nicely in the terminal.

```sh
mrkdwn README.md
```

mrkdwn turns Markdown into colored, formatted terminal output: headings,
emphasis, lists, block quotes, tables, code blocks and horizontal rules.
Paragraphs are re-wrapped to fit your terminal.

## Features

- Colored headings, **bold**, *italic*, ~~strikethrough~~ and `inline code`
- Bullet and numbered lists, block quotes, horizontal rules
- Tables with box-drawing borders
- Fenced and indented code blocks on a shaded background
- Paragraphs re-wrapped to the terminal width (hard line breaks are kept)
- Reads files or stdin, and renders several files in one go
- Exits cleanly when piped into `head`, `less` and similar tools

## Installation

### Requirements

- A Rust toolchain with Cargo (edition 2024, i.e. Rust 1.85 or newer).
  Get it from [rustup.rs](https://rustup.rs) or your package manager.
- A terminal with ANSI color support (any modern terminal).

### With the install script

```sh
git clone https://github.com/alpwrk/mrkdwn.git
cd mrkdwn
./install.sh
```

This builds a release binary and installs it to `~/.local/bin/mrkdwn`.
Make sure `~/.local/bin` is in your `PATH`; the script warns you if it isn't.

To install somewhere else, set `PREFIX`; the binary goes to `$PREFIX/bin`:

```sh
sudo PREFIX=/usr/local ./install.sh
```

### With Cargo

```sh
cargo install --path .
```

This installs to `~/.cargo/bin`.

### Updating

Pull the latest changes and run the install script again:

```sh
git pull && ./install.sh
```

### Uninstalling

```sh
./install.sh --uninstall
```

Use the same `PREFIX` you installed with. If you installed with Cargo, run
`cargo uninstall mrkdwn` instead.

## Manual

### Synopsis

```
mrkdwn [OPTIONS] [FILES]...
```

### Description

mrkdwn reads Markdown from each `FILE` and writes formatted output to
standard output. If no files are given, it reads from standard input.

When several files are given, each one is preceded by a banner with its
file name.

### Options

| Option | Description |
| --- | --- |
| `-w`, `--width <WIDTH>` | Maximum output width in columns. Default: the terminal width, capped at 100 so long prose stays readable. |
| `-h`, `--help` | Print help and exit. |
| `-V`, `--version` | Print the version and exit. |

### Examples

Render a file:

```sh
mrkdwn README.md
```

Render several files one after another:

```sh
mrkdwn CHANGELOG.md docs/*.md
```

Render from standard input:

```sh
cat notes.md | mrkdwn
curl -s https://example.com/README.md | mrkdwn
```

Use a fixed width, for example for narrow splits:

```sh
mrkdwn -w 72 README.md
```

Page long documents (`-R` keeps the colors):

```sh
mrkdwn README.md | less -R
```

Handy shell alias for paging:

```sh
mdless() { mrkdwn "$@" | less -R; }
```

### Exit status

| Code | Meaning |
| --- | --- |
| `0` | Success (also when the output pipe is closed early, e.g. by `head`). |
| `1` | A file could not be read, or no input was given. |
| `2` | Invalid command-line arguments. |

### Supported Markdown

| Element | Syntax |
| --- | --- |
| Headings | `#` to `######` |
| Emphasis | `*italic*`, `**bold**`, `~~strikethrough~~` |
| Inline code | `` `code` `` |
| Code blocks | ```` ``` ```` or `~~~` fences, or 4-space indentation |
| Lists | `-`, `*`, `+` and `1.` |
| Block quotes | `> quote` |
| Tables | GitHub-style pipe tables |
| Horizontal rules | `---`, `***`, `___` |
| Hard line breaks | two trailing spaces or a trailing `\` |

### Limitations

- Code blocks have no syntax highlighting.
- Links are shown as their Markdown source text, not as clickable links.
- Images, HTML and footnotes are not rendered.
- Colors are always written, even when the output is not a terminal.

## Building from source

```sh
cargo build --release     # binary at target/release/mrkdwn
cargo clippy              # lints
cargo fmt                 # formatting
```

Rendering is done by [termimad](https://crates.io/crates/termimad) and
argument parsing by [clap](https://crates.io/crates/clap).

## License

Copyright (C) 2026 alpwrk

mrkdwn is free software: you can redistribute it and/or modify it under the
terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version. See [LICENSE](LICENSE) for the full text.
