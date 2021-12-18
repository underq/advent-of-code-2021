use std::env;
use std::process;
use std::fs;

fn main() {

    // Parse command line to get filename
    let args: Vec<String> = env::args().collect();
    let input_filename = parse_cmd(&args).unwrap_or_else(|err| {
        println!("Problème rencontré lors de l'interprétation des arguments : {}", err);
        process::exit(1);
    });

    // Parse file to get Vec<Vec<_>>
    let content = fs::read_to_string(input_filename).unwrap();
    let matrix: Vec<_> = content
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    
    println!("Step one: {}", day03::step_one(matrix));

    println!("Step two: {}", day03::step_two(matrix));
    
}


fn parse_cmd(args: &[String]) -> Result<&str, &str> {
    if args.len() < 2 {
        return Err("il n'y a pas assez d'arguments");
    }

    let input_filename = &args[1];

    Ok(input_filename)
}

