mod entry;
mod error;
mod search;

use clap::{Parser, Subcommand};
use entry::Entry;
use error::Error;
use regex::Regex;
use search::Search;
use std::path::{Path, PathBuf};

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
        search: Search,
    },
}

fn main() -> error::Result<()> {
    let args = Cli::parse();

    match args.command {
        Some(command) => {
            match command {
                Command::Add { entry } => entry.write()?,
                Command::Search { search } => {
                    // early exit when "--date" is used with "--to"
                    if search.date.is_some() && search.to.is_some() {
                        eprintln!(
                            "\x1b[1;31merror\x1b[0m: the argument '\x1b[33m--date <DATE>\x1b[0m' cannot be used with '\x1b[33m--to <TO>\x1b[0m'\n\n\x1b[4mUsage\x1b[0m: \x1b[1mtil search --date\x1b[0m <DATE>\n\nFor more information, try '\x1b[1m--help\x1b[0m'."
                        );

                        std::process::exit(1);
                    }

                    let mut entry = String::default();
                    if let Some(date) = search.date {
                        // must use MM-DD-YYYY for date argument
                        let re = Regex::new(r"^\d{1,2}-\d{1,2}-\d{4}$").unwrap();
                        if !re.is_match(&date) {
                            let err = Error::InvalidDateFormat;
                            eprintln!("{err}");
                            std::process::exit(1);
                        }

                        entry = match Search::by_date(date.to_owned()) {
                            Some(contents) => contents,
                            None => {
                                eprintln!("no notes were found from {}", date);
                                std::process::exit(1);
                            }
                        };
                    } else if let (Some(_from), Some(_to)) = (search.from, search.to) {
                    }
                    println!("{}", entry.trim())
                }
            };

            Ok(())
        }
        None => Err(Error::CannotProcessArgs),
    }
}
