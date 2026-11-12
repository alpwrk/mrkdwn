#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.10"
# dependencies = ["rich>=13"]
# ///
"""mrkdwn - render Markdown nicely in the terminal.

Usage:
    mrkdwn FILE [FILE ...]      render one or more files
    cat FILE | mrkdwn           render from stdin
    mrkdwn -p FILE              open in a pager
"""

import argparse
import sys
from pathlib import Path

from rich.console import Console
from rich.markdown import Markdown
from rich.rule import Rule


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        prog="mrkdwn", description="Render Markdown nicely in the terminal."
    )
    parser.add_argument("files", nargs="*", type=Path, help="Markdown files (default: stdin)")
    parser.add_argument("-p", "--pager", action="store_true", help="show output in a pager")
    parser.add_argument("-w", "--width", type=int, help="maximum output width")
    parser.add_argument(
        "-t", "--theme", default="monokai", help="code block theme (default: monokai)"
    )
    parser.add_argument(
        "--hyperlinks", action="store_true", help="render links as clickable terminal hyperlinks"
    )
    return parser.parse_args()


def read_sources(files: list[Path]) -> list[tuple[str, str]]:
    """Return (name, text) pairs for each input."""
    if not files:
        if sys.stdin.isatty():
            sys.exit("mrkdwn: no input (pass a file or pipe Markdown via stdin)")
        return [("<stdin>", sys.stdin.read())]

    sources = []
    for path in files:
        try:
            sources.append((str(path), path.read_text(encoding="utf-8")))
        except OSError as err:
            sys.exit(f"mrkdwn: {path}: {err.strerror}")
    return sources


def main() -> None:
    args = parse_args()
    sources = read_sources(args.files)

    console = Console(width=args.width)
    if args.width is None and console.width > 100:
        console.width = 100  # keep long prose readable on wide terminals

    def render() -> None:
        for i, (name, text) in enumerate(sources):
            if len(sources) > 1:
                if i:
                    console.print()
                console.print(Rule(f"[bold]{name}[/]", style="cyan"))
            console.print(
                Markdown(text, code_theme=args.theme, hyperlinks=args.hyperlinks)
            )

    if args.pager:
        with console.pager(styles=True):
            render()
    else:
        render()


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        sys.exit(130)
    except BrokenPipeError:
        sys.exit(0)
