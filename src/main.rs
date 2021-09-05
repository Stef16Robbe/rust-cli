extern crate chrono;

use std::io;
use std::fs;
use std::path::Path;
use structopt::StructOpt;
use chrono::{DateTime, Local};

#[derive(Debug)]
#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value = ".")]
    path: String,
}

fn print_dir_content(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let metadata = entry.metadata().unwrap();
            let modified: DateTime<Local> = DateTime::from(metadata.modified()?);
            let size = metadata.len();
            let file_name = entry.file_name().into_string();

            println!(
                "{:>10} {} {}",
                size,
                modified.format("%_d %b %H:%M").to_string(),
                file_name.unwrap(),
            );
        }
    }
    Ok(())
}

/*
* 
* https://endler.dev/2018/ls/
* usage: 'cargo run -- .'
* 
*/
fn main() -> std::io::Result<()> {
    let args = Cli::from_args();
    
    print_dir_content(Path::new(&args.path)).expect("Something went wrong.");

    Ok(())
}
