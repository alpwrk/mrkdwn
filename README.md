# mrkdwn

A tiny tool that renders Markdown nicely in the terminal: headings, lists,
tables, block quotes and syntax-highlighted code blocks.

## Requirements

- [uv](https://docs.astral.sh/uv/) (it fetches the `rich` dependency automatically)

## Usage

```sh
./mrkdwn.py README.md          # render a file
./mrkdwn.py a.md b.md          # render several files
cat notes.md | ./mrkdwn.py     # render from stdin
./mrkdwn.py -p README.md       # open in a pager
./mrkdwn.py -t dracula FILE    # pick a code highlighting theme
```

| Option | Description |
| --- | --- |
| `-p`, `--pager` | show output in a pager |
| `-w`, `--width N` | maximum output width (default: terminal width, capped at 100) |
| `-t`, `--theme NAME` | Pygments theme for code blocks (default: `monokai`) |
| `--hyperlinks` | render links as clickable terminal hyperlinks |

## Install

```sh
ln -s "$PWD/mrkdwn.py" ~/.local/bin/mrkdwn
```
