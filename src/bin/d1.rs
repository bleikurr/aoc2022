use std::env;
use std::io::{self, BufRead, BufReader};
use std::fs::File;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let reader: Option<Box<dyn BufRead>>;
    let mut buffer = String::new();

    let mut elves: Vec<i32> = vec![];

    if args.len() > 1 {
        reader = Some(Box::new(BufReader::new(File::open(&args[1])?)));
    } else {
        reader = Some(Box::new(BufReader::new(io::stdin())));
    }

    let mut acc = 0;
    let mut reader = reader.unwrap();
    while reader.read_line(&mut buffer)? > 0 {
        let line = buffer.trim();
        if line.is_empty()  {
            elves.push(acc);
            acc = 0;
        } else {
            acc += line.parse::<i32>()?;
        }

        buffer.clear();
    }

    elves.sort_by(|a, b| b.cmp(a));

    println!("Top elves:");
    let top_elves = &elves[..3];
    for (i, calories) in top_elves.iter().enumerate() {
        println!("  {}. {}", i+1, calories);
    }

    let total_top_elves: i32 = top_elves.iter().sum();
    println!("\nTop 3 total: {}", total_top_elves);

    return Ok(())
}
