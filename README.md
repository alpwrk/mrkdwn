# mrkdwn

A tiny Rust tool that renders Markdown nicely in the terminal: colored
headings, emphasis, lists, block quotes, tables, code blocks and rules.
Paragraphs are re-wrapped to fit the terminal.

Built on [termimad](https://crates.io/crates/termimad).

## Build & install

```sh
cargo install --path .
```

## Usage

```sh
mrkdwn README.md             # render a file
mrkdwn a.md b.md             # render several files
cat notes.md | mrkdwn        # render from stdin
mrkdwn -w 72 README.md       # limit the width
mrkdwn README.md | less -R   # page long documents
```

| Option | Description |
| --- | --- |
| `-w`, `--width N` | maximum output width (default: terminal width, capped at 100) |
| `-h`, `--help` | show help |
| `-V`, `--version` | show version |
