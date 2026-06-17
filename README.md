# jot

A small CLI for timestamped notes. Stores everything as plain text in `~/.jot/`.

## Build

```bash
cargo build --release
```

The binary is at `./target/release/jot`.

## Usage

```bash
# Add a note for today
./target/release/jot "fixed the nginx config"

# Pipe a note from stdin
echo "deployed fix" | ./target/release/jot

# Show today's notes (default when run with no args)
./target/release/jot

# Show yesterday's notes
./target/release/jot --yesterday

# List recent notes (default 20)
./target/release/jot -l
./target/release/jot -l 5

# Search all notes
./target/release/jot -s nginx

# Filter by tag
./target/release/jot -T work

# List all tags
./target/release/jot --tags

# Add or view a note for a specific date
./target/release/jot -d 2026-06-15 "yesterday's retro"
./target/release/jot -d 2026-06-15

# Open today's file in $EDITOR
./target/release/jot --editor
```

## Install

### Prebuilt binary

```bash
curl -fsSL https://jot.may-be.gay/install.sh | bash
```

The script downloads the latest release from GitHub and installs to `~/.local/bin`.
Edit `install.sh` and replace `YOUR_GITHUB_USER/jot` with your actual repo before tagging a release.

### From source

```bash
cargo install --path .
# now `jot` is on your PATH
```

## Storage

Notes live in `~/.jot/YYYY-MM-DD.md`, one file per day. Each line is `HH:MM note`.

Use the `JOT_DIR` environment variable to change the notes directory:

```bash
export JOT_DIR=~/notes/jot
jot "entry stored in custom location"
```

## Test

```bash
cargo test
```
