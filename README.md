# ✨ til - today i learned ✨

til is a command-line application designed to help you keep track of the important sh%t you want to remember. Whether it's a key insight from your work, a useful programming trick, or a valuable life lesson, this app helps you store and retrieve your notes in a friendly manner.

## Current Features
* Store notes by passing a message and an optional title.
* Retrieve your notes by searching a date (MM-DD-YYYY) or title.

## Installation

To install and use the CLI, follow these steps:

```
cargo install today-i-learned
```

_today-i-learned on [crates.io](https://crates.io/crates/today-i-learned)_

## Usage

### That

To store a note, use the `that` command with a message and an optional title:

```
til that --message "Your note message" --title "Optional title"
```

### On

To retrieve a note, use the `on` command with the date and title parameters:

```
til on --date "MM-DD-YYYY" --title "Title"
```

## Configuration

The app stores notes in a `.til/notes` directory under your home directory. This directory is created automatically if it does not exist. In future versions of this app, the location you store notes will be configurable.
