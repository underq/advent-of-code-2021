use std::fs;
use std::error::Error;

pub fn parse_config(args: &[String]) -> Result<&str, &str> {
    if args.len() < 2 {
        return Err("il n'y a pas assez d'arguments");
    }

    let input_filename = &args[1];

    Ok(input_filename)
}

pub fn run(input_filename: &str) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(input_filename)?;

    let contents: Vec<u32> = content.lines().clone().map(|a| {let b: u32 = a.parse().unwrap(); b}).collect();

    println!("{}", step_one(contents.clone()));
    println!("{}", step_two(contents.clone()));
    Ok(())
}

pub fn step_one(content: Vec<u32>) -> usize {
    let mut total: usize = 0;
    let mut previous_line = u32::MAX;

    content
        .iter()
        .for_each( |&current_line| {
            let current_line = current_line;
            if previous_line < current_line { 
                total += 1;
            }
            previous_line = current_line
        });
    
    total
}

pub fn step_two(content: Vec<u32>) -> usize {
    let mut total: usize = 0;
    let mut previous_value = u32::MAX;
    
    content
        .iter()
        .enumerate()
        .for_each( |(i, &v)| {
            let current_value: u32;
            if content.len() > i + 2 {
                current_value = v + content[i + 1] + content[ i + 2 ];
            } else {
                current_value = 0;
            }

            //println!("{} : {}", previous_value, current_value);

            if previous_value < current_value { 
                total += 1;
            }
            previous_value = current_value
        });
    
    total
}