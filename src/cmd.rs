use std::env;
use std::io::{self, BufRead, BufReader};
use std::fs::File;

pub fn get_input() -> Result<Box<dyn BufRead>, Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        return Ok(Box::new(BufReader::new(File::open(&args[1])?)));
    } else {
        return Ok(Box::new(BufReader::new(io::stdin())));
    }
}
