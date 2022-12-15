use aoc2022::cmd;
use std::collections::HashMap;
use std::error::Error;

const CHARS: &str = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn check_compartments(charmap: &HashMap<char, usize>, comp1: &str, comp2: &str) 
    -> Result<usize, String>
{
    for c in comp1.chars() {
        if comp2.contains(c) {
            return Ok(*charmap.get(&c).ok_or("Character not found")?);
        }
    } 
    Err(format!("Invalid input: {} {}", comp1, comp2))
}

fn check_group(charmap: &HashMap<char, usize>,
               line1: &str, line2: &str, line3: &str) 
    -> Result<usize, String>
{
    for c in line1.chars() {
        if line2.contains(c) && line3.contains(c) {
            return Ok(*charmap.get(&c).ok_or("Character not found")?);
        }
    } 
    Err(format!("Invalid input: {} {} {}", line1 , line2, line3))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut charmap = HashMap::new();
    for (i, c) in CHARS.chars().enumerate() {
        charmap.insert(c, i);
    }

    // Get input
    let mut buffer = String::new();
    let mut input = cmd::get_input()?;
    input.read_to_string(&mut buffer)?;
  
    // Part 1
    let mut acc = 0;
    for line in buffer.lines() {
        let len = line.len();
        let comp1 = &line[..len/2];
        let comp2 = &line[len/2..];
       
        let priority = check_compartments(&charmap, comp1, comp2)?;
        acc += priority;
    }

    println!("SUM Part 1: {}", acc);

    // Part 2
    let mut acc = 0;
    let mut lines = buffer.lines();
    while let Some(line1) = lines.next() {
        let line2 = lines.next().ok_or("Error reading elf group")?;
        let line3 = lines.next().ok_or("Error reading elf group")?;
         
        acc += check_group(&charmap, line1, line2, line3)?;
    }

    println!("SUM Part 2: {}", acc);

   Ok(()) 
}
