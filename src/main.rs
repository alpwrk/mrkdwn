// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 alpwrk

//! mrkdwn - render Markdown nicely in the terminal.

use std::fs;
use std::io::{self, IsTerminal, Read, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use termimad::crossterm::style::{Attribute, Color};
use termimad::{MadSkin, terminal_size};

/// Render Markdown nicely in the terminal.
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Markdown files to render (reads stdin if none are given)
    files: Vec<PathBuf>,

    /// Maximum output width (default: terminal width, capped at 100)
    #[arg(short, long)]
    width: Option<usize>,
}

const MAX_DEFAULT_WIDTH: usize = 100;

fn main() -> ExitCode {
    let args = Args::parse();

    let sources = match read_sources(&args.files) {
        Ok(sources) => sources,
        Err(msg) => {
            eprintln!("mrkdwn: {msg}");
            return ExitCode::FAILURE;
        }
    };

    let width = args
        .width
        .unwrap_or_else(|| (terminal_size().0 as usize).min(MAX_DEFAULT_WIDTH));
    match render(&sources, width) {
        Ok(()) => ExitCode::SUCCESS,
        // Reader went away (e.g. `mrkdwn file | head`): not an error.
        Err(e) if e.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("mrkdwn: {e}");
            ExitCode::FAILURE
        }
    }
}

fn render(sources: &[(String, String)], width: usize) -> io::Result<()> {
    let skin = make_skin();
    let mut out = io::stdout().lock();
    let multiple = sources.len() > 1;

    for (i, (name, text)) in sources.iter().enumerate() {
        if multiple {
            if i > 0 {
                writeln!(out)?;
            }
            writeln!(out, "{}", header(name, width))?;
        }
        write!(out, "{}", skin.text(&reflow(text), Some(width)))?;
    }
    out.flush()
}

/// Joins soft-wrapped paragraph lines so termimad can wrap them to the
/// terminal width itself. Code blocks, lists, tables, quotes, headings and
/// hard line breaks are left untouched.
fn reflow(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut in_fence = false;
    let mut prev_joinable = false;

    for line in text.lines() {
        let trimmed = line.trim_start();
        let is_fence = trimmed.starts_with("```") || trimmed.starts_with("~~~");

        if is_fence {
            in_fence = !in_fence;
        }

        let joinable = !in_fence && !is_fence && is_plain_text(line);
        if joinable && prev_joinable {
            out.pop(); // drop the previous newline
            out.push(' ');
            out.push_str(trimmed);
        } else {
            out.push_str(line);
        }
        out.push('\n');

        // A line ending in two spaces or a backslash is a hard break.
        prev_joinable = joinable && !(line.ends_with("  ") || line.ends_with('\\'));
    }
    out
}

/// True for an ordinary paragraph line (not a block-level construct).
fn is_plain_text(line: &str) -> bool {
    if line.trim().is_empty() || line.starts_with("    ") || line.starts_with('\t') {
        return false;
    }
    let t = line.trim_start();
    let starts_block = ["#", ">", "|", "- ", "* ", "+ ", "---", "***", "___"]
        .iter()
        .any(|p| t.starts_with(p));
    let ordered_item = t
        .split_once(". ")
        .is_some_and(|(n, _)| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()));
    !starts_block && !ordered_item
}

/// Returns (name, contents) pairs for every input.
fn read_sources(files: &[PathBuf]) -> Result<Vec<(String, String)>, String> {
    if files.is_empty() {
        if io::stdin().is_terminal() {
            return Err("no input (pass a file or pipe Markdown via stdin)".into());
        }
        let mut text = String::new();
        io::stdin()
            .read_to_string(&mut text)
            .map_err(|e| format!("<stdin>: {e}"))?;
        return Ok(vec![("<stdin>".into(), text)]);
    }

    files
        .iter()
        .map(|path| {
            fs::read_to_string(path)
                .map(|text| (path.display().to_string(), text))
                .map_err(|e| format!("{}: {e}", path.display()))
        })
        .collect()
}

/// Builds a centered file-name banner like `──── notes.md ────`.
fn header(name: &str, width: usize) -> String {
    let label = format!(" {name} ");
    let fill = width.saturating_sub(label.chars().count());
    let left = "─".repeat(fill / 2);
    let right = "─".repeat(fill - fill / 2);
    format!("\x1b[36m{left}\x1b[1m{label}\x1b[22m{right}\x1b[0m")
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();

    skin.headers[0].set_fg(Color::Magenta);
    skin.headers[0].add_attr(Attribute::Bold);
    skin.headers[1].set_fg(Color::Cyan);
    skin.headers[1].add_attr(Attribute::Bold);
    for header in skin.headers.iter_mut().skip(2) {
        header.set_fg(Color::Blue);
        header.add_attr(Attribute::Bold);
    }

    skin.bold.set_fg(Color::White);
    skin.italic.set_fg(Color::Yellow);
    skin.strikeout.add_attr(Attribute::CrossedOut);
    skin.inline_code.set_fg(Color::Green);
    skin.inline_code.set_bg(Color::AnsiValue(236));
    skin.code_block.set_fg(Color::Green);
    skin.code_block.set_bg(Color::AnsiValue(235));
    skin.bullet.set_fg(Color::Cyan);
    skin.quote_mark.set_fg(Color::DarkGrey);
    skin.horizontal_rule.set_fg(Color::DarkGrey);
    skin.table.set_fg(Color::DarkGrey);

    skin
}
