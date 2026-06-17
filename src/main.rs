use chrono::{Duration, Local, NaiveDate, NaiveDateTime};
use clap::Parser;
use std::env;
use std::fs::{self, OpenOptions, read_to_string};
use std::io::{IsTerminal, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
#[command(name = "noted", about = "Timestamped notes CLI")]
struct Cli {
    message: Option<String>,

    #[arg(short, long, value_name = "N", num_args = 0..=1, default_missing_value = "20")]
    list: Option<usize>,

    #[arg(short, long, value_name = "QUERY")]
    search: Option<String>,

    #[arg(short = 'T', long, value_name = "TAG")]
    tag: Option<String>,

    #[arg(long)]
    tags: bool,

    #[arg(long)]
    yesterday: bool,

    #[arg(short, long, value_name = "YYYY-MM-DD")]
    date: Option<String>,

    #[arg(long)]
    editor: bool,
}

fn notes_dir() -> PathBuf {
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));
    env::var("NOTED_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| home.join(".noted"))
}

fn day_file(date: NaiveDate) -> PathBuf {
    notes_dir().join(format!("{}.md", date.format("%Y-%m-%d")))
}

fn add_entry(message: &str, date: Option<NaiveDate>) -> std::io::Result<PathBuf> {
    let now = Local::now().naive_local();
    let when = match date {
        Some(d) => NaiveDateTime::new(d, now.time()),
        None => now,
    };
    let path = day_file(when.date());
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new().create(true).append(true).open(&path)?;
    writeln!(file, "{} {}", when.format("%H:%M"), message)?;
    Ok(path)
}

fn iter_entries() -> Vec<(PathBuf, String)> {
    let dir = notes_dir();
    let Ok(paths) = fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut files: Vec<_> = paths
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("md"))
        .collect();
    files.sort();

    let mut entries = Vec::new();
    for path in files {
        if let Ok(text) = read_to_string(&path) {
            for line in text
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
            {
                entries.push((path.clone(), line));
            }
        }
    }
    entries
}

fn list_entries(n: usize) -> Vec<(PathBuf, String)> {
    let mut all = iter_entries();
    let start = all.len().saturating_sub(n);
    all.split_off(start)
}

fn search_entries(query: &str) -> Vec<(PathBuf, String)> {
    let q = query.to_lowercase();
    iter_entries()
        .into_iter()
        .filter(|(_, line)| line.to_lowercase().contains(&q))
        .collect()
}

fn tag_entries(tag: &str) -> Vec<(PathBuf, String)> {
    iter_entries()
        .into_iter()
        .filter(|(_, line)| has_tag(line, tag))
        .collect()
}

fn all_tags() -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();
    for (_, line) in iter_entries() {
        tags.extend(extract_tags(&line).into_iter().map(|s| s.to_string()));
    }
    tags.sort();
    tags.dedup();
    tags
}

fn extract_tags(line: &str) -> Vec<&str> {
    line.split_whitespace()
        .filter_map(|word| {
            let tag = word.strip_prefix('#')?;
            if tag.is_empty() || !tag.chars().all(|c| c.is_alphanumeric() || c == '_') {
                return None;
            }
            Some(tag)
        })
        .collect()
}

fn has_tag(line: &str, tag: &str) -> bool {
    line.split_whitespace()
        .filter_map(|w| w.strip_prefix('#'))
        .any(|t| t == tag)
}

fn format_entry(path: &Path, line: &str) -> String {
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    format!("{} {}", stem, line)
}

fn parse_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| format!("date must be YYYY-MM-DD: {e}"))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(n) = cli.list {
        for (path, line) in list_entries(n) {
            println!("{}", format_entry(&path, &line));
        }
        return Ok(());
    }

    if let Some(query) = cli.search {
        for (path, line) in search_entries(&query) {
            println!("{}", format_entry(&path, &line));
        }
        return Ok(());
    }

    if let Some(tag) = cli.tag {
        for (path, line) in tag_entries(&tag) {
            println!("{}", format_entry(&path, &line));
        }
        return Ok(());
    }

    if cli.tags {
        for tag in all_tags() {
            println!("{}", tag);
        }
        return Ok(());
    }

    if cli.yesterday {
        if let Ok(text) = read_to_string(&day_file(Local::now().date_naive() - Duration::days(1))) {
            print!("{}", text);
        }
        return Ok(());
    }

    if cli.editor {
        let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
        let path = day_file(Local::now().date_naive());
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        Command::new(editor).arg(&path).status()?;
        return Ok(());
    }

    let message = cli.message.or_else(|| {
        if std::io::stdin().is_terminal() {
            return None;
        }
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input).ok()?;
        let input = input.trim();
        if input.is_empty() {
            None
        } else {
            Some(input.to_string())
        }
    });

    if let Some(message) = message {
        let date = cli.date.map(|s| parse_date(&s)).transpose()?;
        let path = add_entry(&message, date)?;
        println!("noted: {}", path.display());
        return Ok(());
    }

    let date = cli.date.map(|s| parse_date(&s)).transpose()?;
    let path = day_file(date.unwrap_or_else(|| Local::now().date_naive()));
    if let Ok(text) = read_to_string(&path) {
        print!("{}", text);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_tags_finds_valid_tags() {
        assert_eq!(extract_tags("hello #world #rust"), vec!["world", "rust"]);
    }

    #[test]
    fn extract_tags_ignores_invalid() {
        assert_eq!(extract_tags("# #has-dash #ok"), vec!["ok"]);
    }

    #[test]
    fn has_tag_checks_exact_tag() {
        assert!(has_tag("deployed #work #ops", "work"));
        assert!(!has_tag("deployed #work #ops", "working"));
    }

    #[test]
    fn format_entry_shows_date_and_line() {
        let path = PathBuf::from("/home/user/.noted/2026-06-16.md");
        assert_eq!(
            format_entry(&path, "10:30 fixed nginx"),
            "2026-06-16 10:30 fixed nginx"
        );
    }
}
