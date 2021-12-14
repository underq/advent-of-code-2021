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

    let mut position = day02::Position::new(0, 0, 0);

    // Parse file to get Vec<(String, u32)>
    let content = fs::read_to_string(input_filename).unwrap();
    let matrix: Vec<_> = content
        .lines()
        .clone()
        .map(|line| {
            let words: Vec<_> = line.split(" ").collect();
            (words[0], words[1].parse::<u32>().unwrap())
        })
        .collect();

    // Execute operation on matrix
    matrix
        .into_iter()
        .for_each(|l| {
            let (action, value) = l;
            match action {
                "down" => position.down(value),
                "up" => position.up(value),
                "forward" => position.forward(value),
                _ => ()
            }
        });

    println!("{}", position.horizontal() * position.depth());
}

pub fn parse_cmd(args: &[String]) -> Result<&str, &str> {
    if args.len() < 2 {
        return Err("il n'y a pas assez d'arguments");
    }

    let input_filename = &args[1];

    Ok(input_filename)
}
