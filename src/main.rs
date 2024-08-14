mod error;
mod main_tests;

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
    /// store information that you learned today
    That {
        #[clap(flatten)]
        entry: Entry,
    },
    /// recall information that you learned on a specific day
    On {
        #[clap(flatten)]
        search_params: SearchParams,
    },
}

#[derive(Args, Debug)]
struct Entry {
    #[clap(short, long)]
    message: String,

    #[clap(short, long, default_value = "default")]
    title: String,
}

impl Entry {
    fn write(&self) -> error::Result<()> {
        let path = self.build_path().map_err(|_| Error::CannotBuildPath)?;

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path)
            .expect("cannot open or create file");

        file.write_all(format!("- {}\n", self.message).as_bytes())
            .expect("cannot write to file");

        Ok(())
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
        Some(command) => Ok(match command {
            Command::That { entry } => entry.write()?,
            Command::On { search_params } => Entry::retrieve_from(search_params),
        }),
        None => Err(Error::CannotProcessArgs),
    }
}
