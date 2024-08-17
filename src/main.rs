mod error;

use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use chrono::{Datelike, Local};
use clap::{Args, Parser, Subcommand};
use error::Error;

const PATH_FROM_ROOT: &str = ".til/notes";

fn find_root_dir() -> Option<PathBuf> {
    Some(Path::new(&dirs::home_dir()?).join(PATH_FROM_ROOT))
}

#[derive(Parser, Debug)]
#[command(name = "til", version = "0.1.0", about = "✨ 'today i learned' is used to keep track of the important sh%t you want to remember ✨", long_about = None, arg_required_else_help = true)]
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
    /// recalls a note entry from a specific date
    On {
        #[clap(flatten)]
        search_params: SearchParams,
    },
}

#[derive(Args, Debug)]
struct Entry {
    content: String,

    #[clap(short, long, default_value = "default")]
    title: String,

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
        }

        file.write_all(format!("- {}\n", self.content).as_bytes())
            .map_err(|_| Error::CannotWriteToFile(path.clone()))
    }

    fn build_path(&self) -> error::Result<PathBuf> {
        let time = Local::now();
        let date = format!("{}-{}-{}", time.month(), time.day(), time.year());

        let root_dir = find_root_dir().ok_or(Error::CannotFindDir("root".to_owned()))?;
        let path = {
            let mut path = Path::new(&root_dir).join(&date).join(&self.title);
            path.set_extension("md");
            path
        };

        let directory = Path::new(&path)
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
title: "{}"
tags: [{}]
---

"#,
            self.title,
            self.tags.join(", ")
        )
    }

    fn retrieve_from(_search_params: SearchParams) {
        todo!()
    }
}

#[derive(Args, Debug)]
struct SearchParams {
    #[arg(short, long, default_value = "")]
    date: Option<String>,
    #[arg(short, long, default_value = "")]
    title: Option<String>,
}

fn main() -> error::Result<()> {
    let args = Cli::parse();

    match args.command {
        Some(command) => {
            match command {
                Command::Add { entry } => entry.write()?,
                Command::On { search_params } => Entry::retrieve_from(search_params),
            };

            Ok(())
        }
        None => Err(Error::CannotProcessArgs),
    }
}
