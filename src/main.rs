use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use chrono::{Datelike, Local};
use clap::{Args, Parser, Subcommand};

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

    #[clap(short, long, default_value = "")]
    title: String,
}

impl Entry {
    fn write(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(self.build_path())
            .expect("cannot open or create file");

        file.write_all(format!("- {}\n", self.message).as_bytes())
            .expect("cannot write to file");
    }

    fn build_path(&self) -> String {
        let time = Local::now();
        let date = format!("{}-{}-{}", time.month(), time.day(), time.year());

        // TODO: path should point to a globally accessible route.
        let path = if self.title.is_empty() {
            format!("./notes/{}/default.csv", date)
        } else {
            format!("./notes/{}/{}.csv", date, self.title)
        };

        let directory = Path::new(&path)
            .parent()
            .expect("cannot determine parent directory from path");

        if !directory.exists() {
            fs::create_dir_all(directory)
                .unwrap_or_else(|err| panic!("cannot create directory {:?}: {}", directory, err));
        }

        path
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

fn main() {
    let args = Cli::parse();

    if let Some(command) = args.command {
        match command {
            Command::That { entry } => entry.write(),
            Command::On { search_params } => Entry::retrieve_from(search_params),
        }
    }
}
