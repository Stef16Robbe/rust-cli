use std::io;
use std::fs;
use colored::*;
use std::path::Path;
use structopt::StructOpt;
use chrono::{DateTime, Local};

#[derive(Debug)]
#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value = ".")]
    path: String,
}

fn get_dir_content(dir: &Path) -> io::Result<Vec<String>> {
    let mut printables = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let metadata = entry.metadata().unwrap();
            let modified: DateTime<Local> = DateTime::from(metadata.modified()?);
            let size = metadata.len();
            let mut file_name = entry.file_name().into_string().unwrap();
            if entry.path().is_dir() {
                file_name.insert(0, '/');
            }


            let dir_item: String = format!(
                "{:>10} {} {}",
                size.to_string().red(),
                modified.format("%_d %b %H:%M").to_string().blue(),
                file_name.green(),
            );
            printables.push(dir_item);
        }
    }
    Ok(printables)
}

/*
* 
* https://endler.dev/2018/ls/
* usage: 'cargo run -- .'
* 
*/
fn main() -> std::io::Result<()> {
    let args = Cli::from_args();
    
    let mut dir_contents: Vec<String> = get_dir_content(Path::new(&args.path)).expect("");

    // this works.
    dir_contents.sort_by_key(|k| k.to_string());

    for v in &dir_contents {
        println!("{}", v);
    }

    Ok(())
}
