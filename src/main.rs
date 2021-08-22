use structopt::StructOpt;
//use std::fs::File;
//use std::io::{BufReader, BufRead};


#[derive(StructOpt)]
#[derive(Debug)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn main() {
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

    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file");
    
    
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }



}
