use std::{fs, path::Path};

use clap::Args;

use crate::find_root_dir;

#[derive(Args, Debug)]
pub struct Search {
    /// Specify an exact date ("MM-DD-YYY")
    #[clap(long, group("search"))]
    pub date: Option<String>,

    /// Specify the start of a date range, used with "--to" ("MM-DD-YYYY")
    #[clap(long, group("search"), requires = "to")]
    pub from: Option<String>,

    /// Specify the end of a date range, used with "--from" ("MM-DD-YYYY")
    #[clap(long, requires = "from")]
    pub to: Option<String>,
}

impl Search {
    pub fn find_by_date(date: String) -> Option<String> {
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

    pub fn _find_by_range(from: String, to: String) {
        println!("find_by_range: {} {}", from, to);
    }
}
