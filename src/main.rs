// Output

use structopt::StructOpt;
use anyhow::{Context, Result};
//use std::fs::File;
//use std::io::{BufReader, BufRead};


#[derive(StructOpt)]
#[derive(Debug)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}


/* 
 * Box<dyn std::error::Error> is also an interesting type. 
 * It’s a Box that can contain any type that implements the standard Error trait. 
 * This means that basically all errors can be put into this box, 
 * so we can use ? on all of the usual functions that return Results.
*/
fn main() -> Result<()> {
    let args = Cli::from_args();
    println!("{:?}", args);

    // TODO: use BufReader instead of read_to_string
    //let f = File::open(&args.path);
    //let reader = BufReader::new(f.unwrap());

    //for line in reader.lines() {
    //    if line.contains(&args.pattern) {
    //        println!("{}", line);
    //    }
    //}
    let path = "tett.txt";
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path))?;
    
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())



}
