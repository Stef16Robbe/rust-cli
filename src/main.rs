use std::io;
use std::fs;
use std::fmt;
use std::env;
use std::path::Path;
use structopt::StructOpt;
// use std::str::FromStr;

/*
 * Structs get camel case.
 * Variables get snake case.
 * Constants get all upper case.
*/


#[derive(Debug)]
#[derive(StructOpt)]
struct Cli {
    // #[structopt(short = "pa", long = "path", default_value = ".")]
    #[structopt(default_value = ".")]
    path: String,
    // path_type: String,
}

fn print_dir_content(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let kak = entry.file_name().into_string();
            // let file_name = entry
			// 			.file_name()
			// 			.into_string()
			// 			.or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
            // println!("{}", file_name);

            println!("{:?}", kak.unwrap());
        }
    }
    Ok(())
}

/*
*
* Goal: ls
*
*/
fn main() -> std::io::Result<()> {
    let args = Cli::from_args();
    // println!("{:?}", args);
    
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    print_dir_content(Path::new(&args.path)).expect("Something went wrong.");

    Ok(())
}



















// #[derive(Debug)]
// enum PathType {
//     Rel,
//     Abs
// }

// impl FromStr for PathType {
//     type Err = Infallible;
//     fn from_str(pathType: &str) -> Result<Self, Self::Err> {
//         match pathType {
//             "rel" => Ok(PathType::Rel),
//             "abs" => Ok(PathType::Abs),
//             _ => Err("Could not parse a path type"),
//         }
//     }
// }