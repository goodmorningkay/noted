# noted

A small CLI for timestamped notes. Stores everything as plain text in `~/.noted/`.

## Build

```bash
cargo build --release
```

The binary is at `./target/release/noted`.

## Usage

```bash
# Add a note for today
./target/release/noted "fixed the nginx config"

# Pipe a note from stdin
echo "deployed fix" | ./target/release/noted

# Show today's notes (default when run with no args)
./target/release/noted

# Show yesterday's notes
./target/release/noted --yesterday

# List recent notes (default 20)
./target/release/noted -l
./target/release/noted -l 5

# Search all notes
./target/release/noted -s nginx

# Filter by tag
./target/release/noted -T work

# List all tags
./target/release/noted --tags

# Add or view a note for a specific date
./target/release/noted -d 2026-06-15 "yesterday's retro"
./target/release/noted -d 2026-06-15

# Open today's file in $EDITOR
./target/release/noted --editor
```

## Install

### Prebuilt binary

```bash
curl -fsSL https://noted.may-be.gay/install.sh | bash
```

The script downloads the latest release from GitHub and installs to `~/.local/bin`.
Edit `install.sh` and replace `YOUR_GITHUB_USER/noted` with your actual repo before tagging a release.

### From source

```bash
cargo install --path .
# now `noted` is on your PATH
```

## Storage

Notes live in `~/.noted/YYYY-MM-DD.md`, one file per day. Each line is `HH:MM note`.

Use the `NOTED_DIR` environment variable to change the notes directory:

```bash
export NOTED_DIR=~/notes/noted
noted "entry stored in custom location"
```

## Test

```bash
cargo test
```
