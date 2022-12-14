use aoc2022::cmd;

const ROCKELF: char       = 'A';
const PAPERELF: char      = 'B';
const SCISSORSELF: char   = 'C';

const ROCKU: char       = 'X';
const PAPERU: char      = 'Y';
const SCISSORSU: char   = 'Z';

const WIN: char     = 'Z';
const DRAW: char    = 'Y';
const LOSE: char    = 'X';

fn resolve(strat: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let elf = strat.chars().nth(0).ok_or(
        format!("Error reading strategy: {}", strat))?;
    let you = strat.chars().nth(2).ok_or(
        format!("Error reading strategy: {}", strat))?;
    
    let result = if you == ROCKU {
        1 + match elf {
            ROCKELF => 3,
            PAPERELF => 0,
            SCISSORSELF => 6,
            _ => return Err(format!("Invalid elf strategy: {}", &strat).into())
        }
    } else if you == PAPERU {
        2 + match elf {
            ROCKELF => 6,
            PAPERELF => 3,
            SCISSORSELF => 0,
            _ => return Err(format!("Invalid elf strategy: {}", &strat).into())
        }
    } else if you == SCISSORSU {
        3 + match elf {
            ROCKELF => 0,
            PAPERELF => 6,
            SCISSORSELF => 3,
            _ => return Err(format!("Invalid elf strategy: {}", &strat).into())
        }
    } else {
        return Err("Invalid strategy".into());
    };

    Ok(result)
}

fn resolve2(strat: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let elf = strat.chars().nth(0).ok_or(
        format!("Error reading strategy: {}", strat))?;
    let you = strat.chars().nth(2).ok_or(
        format!("Error reading strategy: {}", strat))?;
    
    let result = if elf == ROCKELF {
        match you {
            WIN => 8,   // 6 + 2
            DRAW => 4,  // 3 + 1
            LOSE => 3,  // 0 + 3
            _ => return Err(format!("Invalid elf strategy: {}", &strat).into())
        }
    } else if elf == PAPERELF {
        match you {
            WIN => 9,   // 6 + 3
            DRAW => 5,  // 3 + 2
            LOSE => 1,  // 0 + 1
            _ => return Err(format!("Invalid elf strategy: {}", &strat).into())
        }

    } else if elf == SCISSORSELF {
        match you {
            WIN => 7,   // 6 + 1
            DRAW => 6,  // 3 + 3
            LOSE => 2,  // 0 + 2
            _ => return Err(format!("Invalid elf strategy: {}", &strat).into())
        }
    } else {
        return Err("Invalid strategy".into());
    };

    Ok(result)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();

    let mut reader = cmd::get_input()?;

    let mut acc = 0;
    let mut acc2 = 0;
    while reader.read_line(&mut buffer)? > 0 {
        acc += resolve(&buffer)?;
        acc2 += resolve2(&buffer)?;
        buffer.clear();
    }

    println!("Wrong strategy guide points: {}", acc);
    println!("Right strategy guide points: {}", acc2);

    Ok(())
}

