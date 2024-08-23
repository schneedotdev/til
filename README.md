# 📚 today i learned 📚

til is a command-line application designed to help you keep track of the important sh%t you want to remember. Whether it's a key insight from your work, a useful programming trick, or a valuable life lesson, this app helps you store and retrieve your notes in a friendly manner.

## Current Features

- Store notes by passing a message and an optional title.
- Retrieve your notes by searching a date (MM-DD-YYYY) or title.

## Installation

To install and use the CLI, follow these steps:

```
cargo install today-i-learned
```

_today-i-learned on [crates.io](https://crates.io/crates/today-i-learned)_

## Usage

### Add

To store a note, use the `add` command, passing a message and _optional_ tags (comma-separated with no spaces):

```
til add "til is build with clap, a powerful command-line argument parser" --tags "rust,clap,crates"
```

### Search

To search for a note, use the `search` command, specifying a date or a range of dates ("MM-DD-YYYY").

#### Date

Search for a note from a specific date:

```
til search --date "8-18-2024"
```

#### Range

Search for a note within a range:

```
til search --from "8-16-2024" --to "8-18-2024"
```

## Configuration

The app stores notes in a `.til/notes` directory under your home directory. This directory is created automatically if it does not exist. In future versions of this app, the location you store notes will be configurable.
