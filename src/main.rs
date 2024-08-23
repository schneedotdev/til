mod error;

use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use chrono::{Datelike, Local};
use clap::{Args, Parser, Subcommand};
use error::Error;
use regex::Regex;

const PATH_FROM_ROOT: &str = ".til/notes";

fn find_root_dir() -> Option<PathBuf> {
    Some(Path::new(&dirs::home_dir()?).join(PATH_FROM_ROOT))
}

#[derive(Parser, Debug)]
#[command(name = "til", version, about = "✨ 'today i learned' is used to keep track of the important sh%t you want to remember ✨", long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Add a new note entry
    Add {
        #[clap(flatten)]
        entry: Entry,
    },
    /// Search for a note by an exact date or within a date range
    Search {
        #[clap(flatten)]
        params: SearchParams,
    },
}

#[derive(Args, Debug)]

struct SearchParams {
    /// Specify an exact date ("MM-DD-YYY")
    #[arg(long, group("search"))]
    date: Option<String>,

    /// Specify the start of a date range, used with "--to" ("MM-DD-YYYY")
    #[arg(long, group("search"), requires = "to")]
    from: Option<String>,

    /// Specify the end of a date range, used with "--from" ("MM-DD-YYYY")
    #[arg(long, requires = "from")]
    to: Option<String>,
}

#[derive(Args, Debug)]
struct Entry {
    content: String,

    #[clap(long, use_value_delimiter = true, default_value = "")]
    tags: Vec<String>,
}

impl Entry {
    fn write(&self) -> error::Result<()> {
        let path = self.build_path().map_err(|_| Error::CannotBuildPath)?;

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&path)
            .map_err(|_| Error::CannotOpenOrCreatePath(path.clone()))?;

        let file_size = file
            .metadata()
            .map_err(|_| Error::CannotReadFile(path.clone()))?
            .len();

        if file_size == 0 {
            file.write_all(self.generate_meta().as_bytes())
                .map_err(|_| Error::CannotWriteToFile(path.clone()))?;
        } else if !self.tags.is_empty() {
            self.update_meta(&path)?;
        }

        file.write_all(format!("- {}\n", self.content).as_bytes())
            .map_err(|_| Error::CannotWriteToFile(path.clone()))
    }

    fn build_path(&self) -> error::Result<PathBuf> {
        let time = Local::now();
        let date = format!("{}-{}-{}", time.month(), time.day(), time.year());

        let root_dir = find_root_dir().ok_or(Error::CannotFindDir("root".to_owned()))?;
        let path = {
            let mut path = Path::new(&root_dir).join(&date).join("default");
            path.set_extension("md");
            path
        };

        let directory = path
            .parent()
            .ok_or(Error::CannotFindDir("parent".to_owned()))?;

        if !directory.exists() {
            fs::create_dir_all(directory)
                .map_err(|_| Error::CannotCreateDir(path.display().to_string()))?;
        }

        Ok(path)
    }

    /// Generates a metadata block for a note entry.
    ///
    /// This function will create a front matter block which includes the
    /// title and tags of the note.
    ///
    /// ## Returns
    ///
    /// Returns a `String` containing the formatted metadata block.
    ///
    /// ## Examples
    ///
    /// ```
    /// let entry = Entry {
    ///     title: "Example Title".to_string(),
    ///     tags: vec!["tag1".to_string(), "tag2".to_string()],
    /// };
    /// let meta = entry.generate_meta();
    /// assert_eq!(meta, r#"---
    /// title: "Example Title"
    /// tags: [tag1, tag2]
    /// ---
    /// "#);
    /// ```
    fn generate_meta(&self) -> String {
        format!(
            r#"---
title: "default"
tags: [{}]
---

"#,
            self.tags.join(", ")
        )
    }

    /// Updates the metadata block for a note entry.
    ///
    /// This function reads the contents of a note entry, parses the metadata,
    /// and updates the "tags" field with any new tags provided in the `Entry`. Tags
    /// already present are not duplicated. The function assumes the metadata is at the
    /// beginning of the file, separated from the content by a `---` delimiter. If the
    /// metadata is missing or cannot be parsed, an error is returned.
    ///
    /// ## Arguments
    ///
    /// * `path` - A reference to the path of the file where the metadata should be updated.
    ///
    /// ## Returns
    ///
    /// Returns a `Result` indicating success (`Ok(())`) or failure (`Error`).
    ///
    /// ## Errors
    ///
    /// * `Error::CannotOpenOrCreatePath` - If the file cannot be opened.
    /// * `Error::CannotReadFile` - If the file cannot be read.
    /// * `Error::CannotParseMetaData` - If the metadata cannot be parsed.
    /// * `Error::CannotWriteToFile` - If the updated contents cannot be written back to the file.
    fn update_meta(&self, path: &PathBuf) -> error::Result<()> {
        let mut contents =
            fs::read_to_string(&path).map_err(|_| Error::CannotReadFile(path.clone()))?;

        let meta = contents
            .split("\n---\n")
            .next()
            .ok_or(Error::CannotParseMetaData)?;

        let tags_regex =
            Regex::new(r"(?m)^tags:\s*\[(.*?)\]$").map_err(|_| Error::CannotParseMetaData)?;
        let mut new_tags = self.tags.clone();

        if let Some(captures) = tags_regex.captures(meta) {
            let existing_tags: Vec<String> = captures[1]
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();

            new_tags.retain(|tag| !existing_tags.contains(tag));

            if !new_tags.is_empty() {
                let updated_tags = existing_tags
                    .into_iter()
                    .chain(new_tags)
                    .collect::<Vec<_>>()
                    .join(", ");
                contents = contents.replace(&captures[0], &format!("tags: [{}]", updated_tags));
            }
        } else {
            return Err(Error::CannotParseMetaData);
        }

        fs::write(&path, contents).map_err(|_| Error::CannotWriteToFile(path.clone()))?;

        Ok(())
    }

    fn find_by_date(date: String) -> Option<String> {
        let root_dir = find_root_dir()?;

        let path = {
            let mut path = Path::new(&root_dir).join(&date).join("default");
            path.set_extension("md");
            path
        };

        let directory = path.parent()?;

        if !directory.exists() {
            return None;
        }

        fs::read_to_string(path).ok()
    }

    fn find_by_range(from: String, to: String) {
        println!("find_by_range: {} {}", from, to);
    }
}

fn main() -> error::Result<()> {
    let args = Cli::parse();

    match args.command {
        Some(command) => {
            match command {
                Command::Add { entry } => entry.write()?,
                Command::Search { params } => {
                    // handle exit case when "--date" is used with "--to"
                    if params.date.is_some() && params.to.is_some() {
                        eprintln!(
                            "\x1b[1;31merror\x1b[0m: the argument '\x1b[33m--date <DATE>\x1b[0m' cannot be used with '\x1b[33m--to <TO>\x1b[0m'\n\n\x1b[4mUsage\x1b[0m: \x1b[1mtil search --date\x1b[0m <DATE>\n\nFor more information, try '\x1b[1m--help\x1b[0m'."
                        );

                        std::process::exit(1);
                    } else {
                        let mut entry = String::default();
                        if let Some(date) = params.date {
                            // must use MM-DD-YYYY for date argument
                            let re = Regex::new(r"^\d{1,2}-\d{1,2}-\d{4}$").unwrap();
                            if !re.is_match(&date) {
                                let err = Error::InvalidDateFormat;
                                eprintln!("{err}");
                                std::process::exit(1);
                            }

                            entry = match Entry::find_by_date(date.clone()) {
                                Some(contents) => contents,
                                None => {
                                    eprintln!("no notes were found from {}", date);
                                    std::process::exit(1);
                                }
                            };
                        } else if let (Some(_from), Some(_to)) = (params.from, params.to) {
                        }
                        println!("{}", entry.trim())
                    }
                }
            };

            Ok(())
        }
        None => Err(Error::CannotProcessArgs),
    }
}
